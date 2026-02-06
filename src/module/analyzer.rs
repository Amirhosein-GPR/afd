use regex::Regex;
use std::{
    collections::{HashSet, VecDeque},
    fs,
};

use crate::module::{
    basic_block::{self, BasicBlock},
    cfg::ControlFlowGraph,
    input::{FileType, InputManager},
    instruction::{AssemblyInstruction, InstructionType, TargetAddress},
    regex::RegexContainer,
};

/// Provides some functions for working with assembly data.
pub struct AssemblyAnalyzer {
    pub cfg: ControlFlowGraph,
}

impl AssemblyAnalyzer {
    /// Creates a new `AssemblyAnalyzer` that contains an array of assembly data fetch from program inputs.
    /// # Parameters
    /// - asm_path: The path to assembly files.
    /// # Example
    /// ```
    /// let assembly_analyzer = AssemblyAnalyzer::new("benchmarks/wcet_bench/disassembleds/original");
    /// ```
    pub fn new(input_manager: &mut InputManager, regex_container: &RegexContainer) -> Self {
        let asm_paths = input_manager
            .get_assembly_paths(&regex_container.bin_file_regex, FileType::AsmOriginal);
        let cf_asm_paths = input_manager
            .get_assembly_paths(&regex_container.bin_file_regex, FileType::AsmCleanedFull);
        let cn_asm_paths = input_manager.get_assembly_paths(
            &regex_container.bin_file_regex,
            FileType::AsmCleanedNecessary,
        );
        let trace_paths = input_manager.get_gem5_trace_paths(&regex_container.bin_file_regex);

        let mut asm_data_vec = Vec::new();
        for (i, asm_path) in asm_paths.into_iter().enumerate() {
            asm_data_vec.push(AssemblyData::new(
                &trace_paths[i],
                asm_path,
                &cf_asm_paths[i],
                &cn_asm_paths[i],
                regex_container,
            ));
        }

        let cfg = ControlFlowGraph::new(asm_data_vec);

        Self { cfg }
    }

    /// Prints subroutine data for each of the assembly data.
    pub fn print(&self) {
        // process::exit(0);

        for ad in &self.cfg.asm_data_vec {
            // for s in &ad.subroutines {
            //     println!("Subroutine {}: {{", s.get_name());
            //     println!(
            //         "  Start Line: {},\n  End Line: {},\n  Assembly Instructions: [",
            //         s.start_line, s.end_line,
            //     );
            //     for (index, ai) in s.assembly_instructions.iter().enumerate() {
            //         println!(
            //             "    Instruction #{}: {{\n      Content: \"{}\"\n      Meta Data: {}\n    }}",
            //             index + 1,
            //             ai.raw_content(),
            //             ai.formatted_string()
            //         );
            //     }
            //     println!("  ]\n");
            //     println!("}}\n");
            // }
            for (index, basic_block) in ad.basic_blocks.iter().enumerate() {
                println!("Basic block #{}: {{", index + 1);
                for (i, ai) in basic_block.assembly_instructions.iter().enumerate() {
                    println!(
                        "  Instruction #{} => [{}]: \"{}\"",
                        i + 1,
                        ai.line_number,
                        ai.raw_content()
                    );
                }
                println!("}}");
            }
        }
    }
}

/// Represents a subroutine in an assembly source file.
///
/// It cotains some useful info about a subroutine in an assembly file.
pub struct Subroutine {
    pub definition_label: String,
    pub start_line: u32,
    pub end_line: u32,
    pub assembly_instructions: Vec<AssemblyInstruction>,
    pub necessary: bool,
}

impl Subroutine {
    pub fn get_name(&self) -> String {
        self.definition_label
            .trim()
            .split(' ')
            .last()
            .unwrap()
            .to_string()
    }

    fn find_related_subroutine_label_for_address(
        subroutines: &[Subroutine],
        asm_inst_address: &str,
    ) -> String {
        for subroutine in subroutines {
            for asm_inst in &subroutine.assembly_instructions {
                if asm_inst.address == asm_inst_address {
                    return subroutine.get_name();
                }
            }
        }

        panic!("No subroutine label was found with the given address");
    }
}

/// Contains the assembly path to an assembly source file, and it's extracted necessary subroutines.
pub struct AssemblyData {
    pub asm_path: String,
    pub subroutines: Vec<Subroutine>,
    pub basic_blocks: Vec<BasicBlock>,
}

impl AssemblyData {
    /// Creates a new `AssemblyData` which has some useful methods used for extracting data from assembly files.
    pub fn new(
        trace_path: &str,
        asm_path: String,
        cf_asm_path: &str,
        cn_asm_path: &str,
        regex_container: &RegexContainer,
    ) -> Self {
        let mut subroutines = Self::extract_subroutines(&asm_path, regex_container);

        Self::save_subroutines(&subroutines, cf_asm_path);

        let basic_blocks = Self::extract_basic_blocks(
            &mut subroutines,
            trace_path,
            &regex_container.gem5_trace_indirect_branch,
        );

        Self::compute_and_flag_necessary_subroutines(&mut subroutines, regex_container);

        Self::save_necessary_subroutines(&subroutines, cn_asm_path);

        Self {
            asm_path,
            subroutines,
            basic_blocks,
        }
    }

    /// Extracts subroutines from an uncommented assmbly file.
    fn extract_subroutines(asm_path: &str, regex_container: &RegexContainer) -> Vec<Subroutine> {
        // Reading the uncommented file
        let assembly_string = fs::read_to_string(asm_path).unwrap();
        let mut asm_lines = assembly_string.lines();

        let mut subroutines = Vec::new();
        let mut line_number = 1;
        while let Some(asm_line) = asm_lines.next() {
            if regex_container.definition_label.is_match(asm_line) {
                let definition_label = asm_line.to_string();
                let start_line = line_number;

                let mut assembly_instructions = Vec::new();

                while let Some(asm_line) = asm_lines.next() {
                    line_number += 1;

                    if asm_line.trim().is_empty() {
                        let end_line = line_number - 1;

                        subroutines.push(Subroutine {
                            definition_label,
                            start_line,
                            end_line,
                            assembly_instructions,
                            necessary: false,
                        });

                        break;
                    } else {
                        let cleaned_asm_line = asm_line.split("//").collect::<Vec<_>>()[0].trim();

                        assembly_instructions.push(AssemblyInstruction::new(
                            cleaned_asm_line,
                            line_number,
                            regex_container,
                        ));
                    }
                }
            }

            line_number += 1;
        }

        subroutines
    }

    fn compute_and_flag_necessary_subroutines(
        subroutines: &mut [Subroutine],
        regex_container: &RegexContainer,
    ) {
        let mut visited_subroutine_labels = HashSet::new();
        let mut remaining_subroutine_labels = VecDeque::new();

        visited_subroutine_labels.insert("<main>:".to_string());
        remaining_subroutine_labels.push_back("<main>:".to_string());
        while let Some(sub_label) = remaining_subroutine_labels.pop_front() {
            for i in 0..subroutines.len() {
                if subroutines[i].definition_label.contains(&sub_label) {
                    subroutines[i].necessary = true;
                    for asm_inst in &subroutines[i].assembly_instructions {
                        if regex_container
                            .jump_to_subroutine
                            .is_match(&asm_inst.raw_content)
                        {
                            match &asm_inst.instruction_type {
                                InstructionType::Cti(cti_data) => match &cti_data.target_address {
                                    TargetAddress::Direct(target_address) => {
                                        let subroutine_label =
                                            Subroutine::find_related_subroutine_label_for_address(
                                                subroutines,
                                                &target_address,
                                            );

                                        if !visited_subroutine_labels.contains(&subroutine_label) {
                                            visited_subroutine_labels
                                                .insert(subroutine_label.clone());
                                            remaining_subroutine_labels.push_back(subroutine_label);
                                        }
                                    }
                                    TargetAddress::Indirect(_register) => {
                                        // If asm_inst is unreachable it's OK to ignore it.
                                        if cti_data.reachable {
                                            let last_asm_ins_adr = u32::from_str_radix(
                                                &subroutines
                                                    .last()
                                                    .unwrap()
                                                    .assembly_instructions
                                                    .last()
                                                    .unwrap()
                                                    .address,
                                                16,
                                            )
                                            .unwrap();
                                            // If basic block address is equal to the last asm_inst + 4, it means that it doesn't exist (doesn't belong to our program) and we can ignore it. If not, the program panics.
                                            if cti_data.target_address.value()
                                                != format!("{:x}", last_asm_ins_adr + 4)
                                            {
                                                panic!(
                                                    "Error in finding necessary subroutines: Can not find the related subroutine for the indirect jump: {}",
                                                    asm_inst.raw_content()
                                                );
                                            }
                                        }
                                    }
                                },
                                InstructionType::Ncti => {}
                            }
                        }
                    }
                    break;
                }
            }
        }
    }

    fn save_subroutines(subroutines: &[Subroutine], cf_asm_path: &str) {
        let mut buffer = Vec::new();

        for sub in subroutines {
            buffer.push(sub.definition_label.clone());

            for asm_ins in &sub.assembly_instructions {
                buffer.push(asm_ins.raw_content().to_string());
            }

            buffer.push("".to_string());
        }

        let buffer = buffer.join("\n");

        fs::write(cf_asm_path, buffer).unwrap();
    }

    fn save_necessary_subroutines(subroutines: &[Subroutine], cn_asm_path: &str) {
        let mut buffer = Vec::new();

        for subroutine in subroutines {
            if subroutine.necessary {
                buffer.push(subroutine.definition_label.clone());

                for asm_ins in &subroutine.assembly_instructions {
                    buffer.push(asm_ins.raw_content().to_string());
                }

                buffer.push("".to_string());
            }
        }

        let buffer = buffer.join("\n");

        fs::write(cn_asm_path, buffer).unwrap();
    }

    fn extract_basic_blocks(
        subroutines: &mut [Subroutine],
        trace_path: &str,
        indirect_branch_in_trace_regex: &Regex,
    ) -> Vec<BasicBlock> {
        let leaders =
            basic_block::extract_leaders(subroutines, trace_path, indirect_branch_in_trace_regex);
        let basic_blocks = basic_block::extract_basic_blocks(subroutines, leaders);

        basic_blocks
    }
}

pub struct Gem5TraceAnalyzer {
    pub gem5_trace_addresses: Regex,
}

impl Gem5TraceAnalyzer {
    pub fn new(gem5_trace_addresses: Regex) -> Self {
        Self {
            gem5_trace_addresses,
        }
    }

    pub fn print(&self) {}
    pub fn save_to_file(&self) {}
}
