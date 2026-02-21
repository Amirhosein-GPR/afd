use std::fs;

use crate::module::{
    analyzer::AssemblyData,
    assembly::{AssemblyInstructionType, Subroutine},
    basic_block::{self},
    config::CFG_EXT,
    input::{self, FileType, InputManager},
};

pub struct ControlFlowGraph {
    pub asm_data_vec: Vec<AssemblyData>,
}

impl ControlFlowGraph {
    pub fn new(asm_data_vec: Vec<AssemblyData>) -> Self {
        Self { asm_data_vec }
    }

    pub fn compute_control_flow(&mut self) {
        for i in 0..self.asm_data_vec.len() {
            let main_sub_index =
                Subroutine::get_main_subroutine_index(&self.asm_data_vec[i].subroutines);

            for j in 0..self.asm_data_vec[i].basic_blocks.len() {
                println!(
                    "Computing Control Flow {}/{} (Progress: {:.1}%). Basic Block {}/{} (Progress: {:.1}%)",
                    i + 1,
                    self.asm_data_vec.len(),
                    (i + 1) as f32 / self.asm_data_vec.len() as f32 * 100.0,
                    j + 1,
                    self.asm_data_vec[i].basic_blocks.len(),
                    (j + 1) as f32 / self.asm_data_vec[i].basic_blocks.len() as f32 * 100.0,
                );

                let mut edges = Vec::new();

                let bb_last_inst = self.asm_data_vec[i].basic_blocks[j]
                    .last_asm_instruction(&self.asm_data_vec[i].subroutines);
                match &bb_last_inst.inst_type {
                    AssemblyInstructionType::CtiConditional(_inst_name, _ta)
                    | AssemblyInstructionType::CtiUnconditional(_inst_name, _ta) => {
                        let bb_sub_index = self.asm_data_vec[i].basic_blocks[j]
                            .asm_inst_ids
                            .first()
                            .unwrap()
                            .subroutine_index;

                        if bb_sub_index == main_sub_index
                            && self.asm_data_vec[i].basic_blocks[j]
                                .asm_inst_ids
                                .last()
                                .unwrap()
                                .asm_inst_index
                                == self.asm_data_vec[i].subroutines[main_sub_index]
                                    .asm_insts
                                    .len()
                                    - 1
                        {
                            continue;
                        }

                        let bb_target_id = bb_last_inst.get_target_id().unwrap();
                        let target_bb_index = basic_block::find_basic_block_index_by_id(
                            &self.asm_data_vec[i].basic_blocks,
                            &bb_target_id,
                        );

                        if let Some(target_bb_index) = target_bb_index {
                            edges.push(Edge::new(
                                bb_last_inst.branch_condition().unwrap(),
                                target_bb_index,
                            ));
                        } else {
                            // let last_inst_adr = usize::from_str_radix(
                            //     &self.asm_data_vec[i]
                            //         .basic_blocks
                            //         .last()
                            //         .unwrap()
                            //         .last_asm_instruction(&self.asm_data_vec[i].subroutines)
                            //         .get_address()
                            //         .unwrap(),
                            //     16,
                            // )
                            // .unwrap();
                            // // If basic block address is equal to the last asm_inst + 4, it means that it doesn't exist (doesn't belong to our program) and we can ignore it. If not, the program panics.
                            // if bb_target_id != format!("{:x}", last_inst_adr + 4) {
                            //     panic!(
                            //         "Error: No basic block was found with the given address: \"{}\"",
                            //         bb_target_id,
                            //     );
                            // }
                        }

                        match &bb_last_inst.inst_type {
                            AssemblyInstructionType::CtiConditional(_inst_name, _ta) => {
                                edges.push(Edge::new(
                                    bb_last_inst.branch_condition_complement().unwrap(),
                                    j + 1,
                                ));
                            }
                            _ => {}
                        }
                    }

                    AssemblyInstructionType::Ncti(_inst_name) => {}
                }

                self.asm_data_vec[i].basic_blocks[j].edges = edges;
            }
        }
    }

    fn programs_basic_blocks_flows_in_text(&self) -> Vec<Vec<String>> {
        let mut programs_bbf = Vec::new();

        for (i, asm_data) in self.asm_data_vec.iter().enumerate() {
            let mut program_bbf = Vec::new();
            for (j, basic_block) in asm_data.basic_blocks.iter().enumerate() {
                let bb_first_asm_inst_id = basic_block.asm_inst_ids.first().unwrap();
                if asm_data.subroutines[bb_first_asm_inst_id.subroutine_index].necessary {
                    let mut edges_info = Vec::new();
                    let mut raw_contents = Vec::new();
                    for (k, edge) in basic_block.edges.iter().enumerate() {
                        edges_info.push(format!(
                            "E #{} => BB #{}",
                            k + 1,
                            edge.destination_index + 1
                        ));
                    }
                    for asm_inst_id in &basic_block.asm_inst_ids {
                        raw_contents.push(format!(
                            "  {}",
                            asm_data.subroutines[asm_inst_id.subroutine_index].asm_insts
                                [asm_inst_id.asm_inst_index]
                                .content
                        ));
                    }
                    program_bbf.push(format!(
                        "Program #{}, BB #{}:\n  Content:\n  {}\n  Edges:\n    {}\n",
                        i + 1,
                        j + 1,
                        raw_contents.join("\n  "),
                        if edges_info.len() > 0 {
                            edges_info.join(", ")
                        } else {
                            "None".to_string()
                        },
                    ));
                }
            }
            programs_bbf.push(program_bbf);
        }

        programs_bbf
    }

    pub fn print(&self) {
        let programs_bbf = self.programs_basic_blocks_flows_in_text();

        for (i, program_bbf) in programs_bbf.iter().enumerate() {
            println!("=== Program #{i} ===");
            for bbf in program_bbf {
                println!("{bbf}");
            }
            println!("");
        }
    }

    pub fn export(&self, file_type: FileType, cfg_paths: &[String]) {
        match file_type {
            FileType::CfgText => {
                let programs_bbf = self.programs_basic_blocks_flows_in_text();
                InputManager::create_parent_dirs(cfg_paths.first().unwrap());

                for (i, program_bbf) in programs_bbf.iter().enumerate() {
                    fs::write(
                        format!("{}{CFG_EXT}.txt", cfg_paths[i]),
                        program_bbf.join("\n"),
                    )
                    .unwrap();
                }
            }
            FileType::CfgGraphics => {
                InputManager::create_parent_dirs(cfg_paths.first().unwrap());
                serialize_in_graphviz_dot(&self.asm_data_vec, cfg_paths);
                export_to_graphical_with_dot(cfg_paths);
            }
            _ => {
                panic!("Wrong file type passed to export function: {file_type:?}");
            }
        }
    }
}

pub struct Edge {
    pub condition: String,
    pub destination_index: usize,
}

impl Edge {
    pub fn new(condition: String, destination_index: usize) -> Self {
        Self {
            condition,
            destination_index,
        }
    }
}

fn serialize_in_graphviz_dot(assembly_data_vec: &[AssemblyData], cfg_graphics_paths: &[String]) {
    println!("\n===== Serializing CFG Data In Graphviz Dot Format To the Related File =====");

    for (i, asm_data) in assembly_data_vec.iter().enumerate() {
        println!(
            "Serializing CFG Data And Storing Them In {}{CFG_EXT}.dot ({}/{} [{:.1}%])",
            cfg_graphics_paths[i],
            i + 1,
            assembly_data_vec.len(),
            (i + 1) as f32 / assembly_data_vec.len() as f32 / 100.0
        );

        let mut graphviz_dot_wrapper = Vec::new();
        graphviz_dot_wrapper.push("digraph cfg {\n".to_string());

        for (j, basic_block) in asm_data.basic_blocks.iter().enumerate() {
            let bb_first_asm_inst_id = basic_block.asm_inst_ids.first().unwrap();
            if asm_data.subroutines[bb_first_asm_inst_id.subroutine_index].necessary {
                let mut asm_insts_contents = Vec::new();
                for asm_inst_id in &basic_block.asm_inst_ids {
                    asm_insts_contents.push(
                        asm_data.subroutines[asm_inst_id.subroutine_index].asm_insts
                            [asm_inst_id.asm_inst_index]
                            .content
                            .clone(),
                    );
                }
                graphviz_dot_wrapper.push(format!(
                    "    {} [label=\"BB {}\\n\\n{}\"];\n",
                    j + 1,
                    j + 1,
                    asm_insts_contents.join("\\l")
                ));

                if basic_block.edges.len() > 0 {
                    for edge in &basic_block.edges {
                        graphviz_dot_wrapper.push(format!(
                            "    {} -> {} [label=\"{}\"];\n",
                            j + 1,
                            edge.destination_index + 1,
                            edge.condition
                        ));
                    }
                }
            }
        }

        graphviz_dot_wrapper.push("}".to_string());

        fs::write(
            format!("{}{CFG_EXT}.dot", cfg_graphics_paths[i]),
            graphviz_dot_wrapper.join(""),
        )
        .unwrap();
    }

    println!("\n===== Writing Serialized CFG Data Finisehd! =====");
}

fn export_to_graphical_with_dot(cfg_graphics_paths: &[String]) {
    println!("\n============== Compiling Control Flow Graph Dot Files ==============");
    println!("CMD ---> dot [DOT_FILE_PATH] -Tpdf -o [PDF_FILE_PATH] -Tsvg -o [SVG_FILE_PATH] -v\n");

    for i in 0..cfg_graphics_paths.len() {
        println!(
            "Compiling {}{CFG_EXT}.dot ({}/{} [{:.1}%])",
            cfg_graphics_paths[i],
            i + 1,
            cfg_graphics_paths.len(),
            (i + 1) as f32 / cfg_graphics_paths.len() as f32 / 100.0
        );

        input::run_command(
            "dot",
            &[
                format!("{}{CFG_EXT}.dot", cfg_graphics_paths[i]).as_str(),
                "-Tpdf",
                "-o",
                format!("{}{CFG_EXT}.pdf", cfg_graphics_paths[i]).as_str(),
                "-Tsvg",
                "-o",
                format!("{}{CFG_EXT}.svg", cfg_graphics_paths[i]).as_str(),
                "-v",
            ],
        );
    }

    println!("\n===== Control Flow Graph Dot Files were compiled successfully! =====");
}
