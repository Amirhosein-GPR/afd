use regex::Regex;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

use crate::module::{
    basic_block::{self, BasicBlock},
    input::{FileType, InputManager},
    instruction::{self, AssemblyInstruction},
    regex::RegexContainer,
};

/// Provides some functions for working with assembly data.
pub struct AssemblyAnalyzer {
    asm_data_vec: Vec<AssemblyData>,
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
        let uasm_paths = input_manager
            .get_assembly_paths(&regex_container.bin_file_regex, FileType::Uncommented);
        let trace_paths = input_manager.gem5_trace_paths(&regex_container.bin_file_regex);

        let mut asm_data_vec = Vec::new();
        for (i, uasm_path) in uasm_paths.into_iter().enumerate() {
            asm_data_vec.push(AssemblyData::new(
                &trace_paths[i],
                uasm_path,
                regex_container,
            ));
        }

        Self { asm_data_vec }
    }

    /// Prints subroutine data for each of the assembly data.
    pub fn print(&self) {
        // process::exit(0);

        for ad in &self.asm_data_vec {
            for s in &ad.subroutines {
                println!("Subroutine {}: {{", s.get_name());
                println!(
                    "  Start Line: {},\n  End Line: {},\n  Assembly Instructions: [",
                    s.start_line, s.end_line,
                );
                for (index, ai) in s.assembly_instructions.iter().enumerate() {
                    println!(
                        "    Instruction #{}: {{\n      Content: \"{}\"\n      Meta Data: {}\n    }}",
                        index + 1,
                        ai.raw_content(),
                        ai.formatted_string()
                    );
                }
                println!("  ]\n");
                println!("}}\n");
            }
            for (index, basic_block) in ad.basic_blocks.iter().enumerate() {
                println!("Basic block #{}: {{", index + 1);
                println!(
                    "  Starting Instruction: \"{}\"",
                    basic_block.starting_instruction.raw_content()
                );
                println!(
                    "  Ending   Instruction: \"{}\"",
                    basic_block.ending_instruction.raw_content()
                );
                println!("}}");
            }
        }
    }

    /// Saves the extracted necessary subroutines of input programs in "workspace/cleaned_assemblies" directory.
    pub fn save_to_file(self, casm_paths: &[String]) {
        for i in 0..self.asm_data_vec.len() {
            let mut buffer = Vec::new();

            for sub in &self.asm_data_vec[i].subroutines {
                buffer.push(sub.definition_label.clone());

                for asm_ins in &sub.assembly_instructions {
                    buffer.push(asm_ins.raw_content().to_string());
                }

                buffer.push("".to_string());
            }

            let buffer = buffer.join("\n");

            fs::write(&casm_paths[i], buffer).unwrap();
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
}

impl Subroutine {
    pub fn get_name(&self) -> String {
        self.definition_label
            .trim()
            .split(' ')
            .last()
            .unwrap()
            .replace(":", "")
    }
}

/// Contains the assembly path to an assembly source file, and it's extracted necessary subroutines.
pub struct AssemblyData {
    uasm_path: String,
    subroutines: Vec<Subroutine>,
    basic_blocks: Vec<BasicBlock>,
}

impl AssemblyData {
    /// Creates a new `AssemblyData` which has some useful methods used for extracting data from assembly files.
    pub fn new(trace_path: &str, uasm_path: String, regex_container: &RegexContainer) -> Self {
        let subroutines = Self::extract_subroutines(&uasm_path, regex_container);

        let basic_blocks = Self::extract_basic_blocks(
            trace_path,
            &regex_container.gem5_trace_indirect_branch,
            &subroutines,
        );

        Self {
            uasm_path,
            subroutines,
            basic_blocks,
        }
    }

    /// Extracts subroutines from an uncommented assmbly file.
    fn extract_subroutines(uasm_path: &str, regex_container: &RegexContainer) -> Vec<Subroutine> {
        // Reading the uncommented file
        let assembly_string = fs::read_to_string(uasm_path).unwrap();

        let mut subroutines = Vec::new();
        let mut labels = VecDeque::new();
        let mut visited_labels = HashSet::new();

        labels.push_back("<main>:$".to_string());
        visited_labels.insert("<main>:$".to_string());

        while let Some(l) = labels.pop_front() {
            subroutines.push(Self::create_subroutine(
                &assembly_string,
                &l,
                regex_container,
            ));

            for subroutine in &subroutines {
                let extracted_labels = Self::find_labels_in_subroutine(
                    &subroutine.assembly_instructions,
                    &regex_container.branches,
                    &regex_container.target_label,
                );
                for el in extracted_labels {
                    if visited_labels.insert(el.clone()) {
                        labels.push_back(el);
                    }
                }
            }
        }

        subroutines
    }

    /// Creates a [`Subroutine`] from an input assembly string.
    fn create_subroutine(
        assembly_string: &str,
        definition_label_pattern: &str,
        regex_container: &RegexContainer,
    ) -> Subroutine {
        // Creating a mutable enumarated iterator of lines.
        let mut assembly_lines = assembly_string.lines();
        let mut assembly_lines = assembly_lines.by_ref().enumerate();

        let mut start_line = 0;
        let mut end_line = 0;

        let mut definition_label = String::new();

        let mut assembly_instructions = Vec::new();

        let definition_label_regex = Regex::new(definition_label_pattern).unwrap();

        // First finding the main subroutine, extracting and storing some info from it, then doing it for other subroutines in a seperate loop.
        'outer: while let Some((line_index, asm_line)) = assembly_lines.next() {
            if definition_label_regex.is_match(asm_line) {
                start_line = line_index as u32 + 1;
                definition_label = asm_line.to_string();

                while let Some((line_index, asm_line)) = assembly_lines.next() {
                    if asm_line.trim().is_empty() {
                        end_line = line_index as u32 + 1;
                        break 'outer;
                    }
                    assembly_instructions.push(instruction::determine_instruction_type(
                        asm_line,
                        line_index as u32 + 1,
                        regex_container,
                    ));
                }
            }
        }

        let mut dlp_chars = definition_label_pattern.chars();
        for _i in 0..2 {
            dlp_chars.next_back().unwrap();
        }

        Subroutine {
            definition_label,
            start_line,
            end_line,
            assembly_instructions,
        }
    }

    /// Finds target labels of branch instructions in a subroutine and returns them as a vector of regex formatted strings.
    fn find_labels_in_subroutine(
        assembly_instructions: &Vec<AssemblyInstruction>,
        branch_regex: &Regex,
        target_label_regex: &Regex,
    ) -> Vec<String> {
        let mut labels = Vec::new();

        for ai in assembly_instructions {
            if branch_regex.is_match(ai.raw_content())
                && let Some(m) = target_label_regex.find(ai.raw_content())
            {
                labels.push(format!("{}:$", m.as_str()));
            }
        }

        labels
    }

    fn extract_basic_blocks(
        trace_path: &str,
        indirect_branch_in_trace_regex: &Regex,
        subroutines: &[Subroutine],
    ) -> Vec<BasicBlock> {
        let leaders =
            basic_block::extract_leaders(trace_path, indirect_branch_in_trace_regex, subroutines);
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

/// Replaces last n characters of a string with some arbitrary string.
fn replace_last_n_chars(n: u8, target_string: &str, replace_with: &str) -> String {
    let mut input_chars = target_string.chars();
    for _i in 0..n {
        input_chars.next_back().unwrap();
    }
    let mut output = input_chars.collect::<String>();
    output.push_str(replace_with);
    output
}
