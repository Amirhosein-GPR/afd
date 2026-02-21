use regex::Regex;
use std::fs;

use crate::module::{
    assembly::{self, AssemblyInstruction, AssemblyInstructionId, Subroutine},
    basic_block::{self, BasicBlock},
    cfg::ControlFlowGraph,
    input::{FileType, InputManager},
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
        for ad in &self.cfg.asm_data_vec {
            for (index, basic_block) in ad.basic_blocks.iter().enumerate() {
                println!("Basic block #{}: {{", index + 1);
                for (i, asm_line_id) in basic_block.asm_inst_ids.iter().enumerate() {
                    println!(
                        "  Instruction #{} => [{}]: \"{}\"",
                        i + 1,
                        ad.subroutines[asm_line_id.subroutine_index].asm_insts
                            [asm_line_id.asm_inst_index]
                            .asm_line_number,
                        ad.subroutines[asm_line_id.subroutine_index].asm_insts
                            [asm_line_id.asm_inst_index]
                            .content
                    );
                }
                println!("}}");
            }
        }
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

        Self::save_necessary_subroutines(&subroutines, cn_asm_path);

        Self {
            asm_path,
            subroutines,
            basic_blocks,
        }
    }

    /// Extracts subroutines from an uncommented assmbly file.
    fn extract_subroutines(asm_path: &str, regex_container: &RegexContainer) -> Vec<Subroutine> {
        println!("=== Extracting Subroutines ===");
        // Reading the uncommented file
        let assembly_string = fs::read_to_string(asm_path).unwrap();
        let mut asm_lines = assembly_string.lines();

        let mut subroutines = Vec::new();
        let mut asm_line_number = 1;
        while let Some(asm_line) = asm_lines.next() {
            if regex_container.definition_label.is_match(asm_line) {
                let definition_label = asm_line.to_string();
                let start_line = asm_line_number + 1;

                let mut assembly_instructions = Vec::new();

                while let Some(asm_line) = asm_lines.next() {
                    asm_line_number += 1;

                    if asm_line.trim().is_empty() {
                        let end_line = asm_line_number - 1;

                        subroutines.push(Subroutine {
                            definition_label,
                            start_line,
                            end_line,
                            asm_insts: assembly_instructions,
                            necessary: false,
                        });

                        println!("Subroutine #{} was extracted", subroutines.len());

                        break;
                    } else {
                        let cleaned_asm_line = asm_line.split("//").collect::<Vec<_>>()[0].trim();

                        assembly_instructions.push(AssemblyInstruction::new(
                            AssemblyInstructionId {
                                subroutine_index: subroutines.len(),
                                asm_inst_index: assembly_instructions.len(),
                            },
                            0,
                            asm_line_number,
                            cleaned_asm_line,
                            &regex_container.instruction_name,
                        ));
                    }
                }
            }

            asm_line_number += 1;
        }

        subroutines
    }

    fn save_subroutines(subroutines: &[Subroutine], cf_asm_path: &str) {
        let mut buffer = Vec::new();

        for sub in subroutines {
            buffer.push(sub.definition_label.clone());

            for asm_line in &sub.asm_insts {
                buffer.push(asm_line.content.to_string());
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

                for asm_line in &subroutine.asm_insts {
                    buffer.push(asm_line.content.to_string());
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
        gem5_trace_indirect_branch_regex: &Regex,
    ) -> Vec<BasicBlock> {
        assembly::compute_target_ids(subroutines, trace_path, gem5_trace_indirect_branch_regex);

        let leader_ids = basic_block::extract_leaders_ids(subroutines);

        for leader_id in &leader_ids {
            println!(
                "Leaders:\n{}",
                subroutines[leader_id.subroutine_index].asm_insts[leader_id.asm_inst_index].content
            )
        }

        let basic_blocks = basic_block::extract_basic_blocks(subroutines, leader_ids);

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
