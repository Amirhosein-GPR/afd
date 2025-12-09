/*!
AFD - ARM Flow Decoder: An AArch64 CFG Constructor in basic block level.
This program is used to create CFGs from disassembled ARM-64Bit (AArch64) binaries.
!*/

use afd::module::{
    analyzer::{AssemblyAnalyzer, Gem5TraceAnalyzer},
    input::{self, FileType, InputManager},
    regex::RegexContainer,
};

fn main() {
    let regex_container = RegexContainer::new();
    let mut input_manager = InputManager::new(&regex_container.src_file_regex);

    input::process_input_files(&mut input_manager, &regex_container);

    let casm_paths =
        input_manager.get_assembly_paths(&regex_container.bin_file_regex, FileType::Cleaned);

    let assembly_analyzer = AssemblyAnalyzer::new(&mut input_manager, &regex_container);
    assembly_analyzer.print();
    assembly_analyzer.save_to_file(&casm_paths);

    let gem5_trace_analyzer = Gem5TraceAnalyzer::new(regex_container.gem5_trace_indirect_branch);
    gem5_trace_analyzer.print();
    gem5_trace_analyzer.save_to_file();
}
