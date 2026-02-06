/*!
AFD - ARM Flow Decoder: An AArch64 CFG Constructor in basic block level.
This program is used to create CFGs from disassembled ARM-64Bit (AArch64) binaries.
!*/

use afd::module::{
    analyzer::AssemblyAnalyzer,
    checkpoint,
    input::{self, FileType, InputManager},
    regex::RegexContainer,
};

fn main() {
    let regex_container = RegexContainer::new();
    let mut input_manager = InputManager::new(&regex_container.src_file_regex);

    // let assembly_analyzer = checkpoint::load_checkpoint();

    input::process_input_files(&mut input_manager, &regex_container);

    let mut assembly_analyzer = AssemblyAnalyzer::new(&mut input_manager, &regex_container);
    assembly_analyzer.print();

    assembly_analyzer.cfg.compute_control_flow();
    assembly_analyzer.cfg.print();

    assembly_analyzer.cfg.export(
        FileType::CfgText,
        input_manager.get_cfg_paths(&regex_container.bin_file_regex, FileType::CfgText),
    );
    assembly_analyzer.cfg.export(
        FileType::CfgGraphics,
        input_manager.get_cfg_paths(&regex_container.bin_file_regex, FileType::CfgGraphics),
    );

    // checkpoint::save_checkpoint(assembly_analyzer);
}
