use std::fs;

use regex::{Captures, Regex};

use crate::module::{
    analyzer::{AssemblyAnalyzer, AssemblyData, Subroutine},
    basic_block::BasicBlock,
    cfg::{ControlFlowGraph, Edge},
    instruction::{
        AssemblyInstruction, BranchType, ConditionalBranch, CtiData, InstructionType,
        TargetAddress, UnconditionalBranch,
    },
};

pub fn save_checkpoint(assembly_analyzer: AssemblyAnalyzer) {
    let mut buffer: Vec<String> = Vec::new();

    buffer.push("asm_analyzer:{\n  cfg:{\n    asm_data_vec:[".to_string());

    for asm_data in assembly_analyzer.cfg.asm_data_vec {
        buffer.push(format!(
            "      asm_data:{{\n        asm_path:\"{}\",\n        subroutines:[",
            asm_data.asm_path
        ));

        for subroutine in asm_data.subroutines {
            buffer.push(format!(
                    "          subroutine:{{\n            definition_label:\"{}\",\n            start_line:{},\n            end_line:{},\n            asm_insts:[",
                    subroutine.definition_label, subroutine.start_line, subroutine.end_line
                ));

            for asm_inst in subroutine.assembly_instructions {
                let inst_type_string = match asm_inst.instruction_type {
                    InstructionType::Cti(cti_data) => {
                        let target_adr = match cti_data.target_address {
                            TargetAddress::Direct(s) => format!("Direct({s})"),
                            TargetAddress::Indirect(s) => format!("Indirect({s})"),
                        };
                        let branch_type = match cti_data.branch_type {
                            BranchType::ConditionalBranch(cb) => {
                                format!("ConditionalBranch({})", cb.value())
                            }
                            BranchType::UnconditionalBranch(ub) => {
                                format!("UnconditionalBranch({})", ub.value())
                            }
                        };

                        format!(
                            "Cti(\n                  cti_data:{{\n                    target_adr:{target_adr},\n                    branch_type:{branch_type},\n                    reachable:{}\n                  }}\n                )",
                            cti_data.reachable
                        )
                    }
                    InstructionType::Ncti => "Ncti".to_string(),
                };

                buffer.push(format!(
                        "              asm_inst:{{\n                raw_content:\"{}\",\n                line_number:{},\n                address:\"{}\",\n                inst_type:{}\n              }},",
                        asm_inst.raw_content,
                        asm_inst.line_number,
                        asm_inst.address,
                        inst_type_string
                    ));
            }

            buffer.push(format!(
                "            ],\n            necessary:{}\n          }},",
                subroutine.necessary
            ));
        }
        buffer.push("        ],\n        basic_blocks:[".to_string());

        for basic_block in asm_data.basic_blocks {
            buffer.push("          basic_block:{\n            asm_insts:[".to_string());

            for asm_inst in basic_block.assembly_instructions {
                let inst_type_string = match asm_inst.instruction_type {
                    InstructionType::Cti(cti_data) => {
                        let target_adr = match cti_data.target_address {
                            TargetAddress::Direct(s) => format!("Direct({s})"),
                            TargetAddress::Indirect(s) => format!("Indirect({s})"),
                        };
                        let branch_type = match cti_data.branch_type {
                            BranchType::ConditionalBranch(cb) => {
                                format!("ConditionalBranch({})", cb.value())
                            }
                            BranchType::UnconditionalBranch(ub) => {
                                format!("UnconditionalBranch({})", ub.value())
                            }
                        };

                        format!(
                            "Cti(\n                  cti_data:{{\n                    target_adr:{target_adr},\n                    branch_type:{branch_type},\n                    reachable:{}\n                  }}\n                )",
                            cti_data.reachable
                        )
                    }
                    InstructionType::Ncti => "Ncti".to_string(),
                };

                buffer.push(format!(
                        "              asm_inst:{{\n                raw_content:\"{}\",\n                line_number:{},\n                address:\"{}\",\n                inst_type:{}\n              }},",
                        asm_inst.raw_content,
                        asm_inst.line_number,
                        asm_inst.address,
                        inst_type_string
                    ));
            }

            buffer.push("            ],\n            edges:[".to_string());

            for edge in basic_block.edges {
                buffer.push(
                format!("              edge:{{\n                condition:\"{}\",\n                dest_index:{}\n              }},", edge.condition, edge.destination_index)
            );
            }

            buffer.push(format!(
                "            ],\n            subr_index:{}\n          }},",
                basic_block.subroutine_index
            ));
        }

        buffer.push("        ]\n      },".to_string());
    }

    buffer.push("    ]\n  }\n}".to_string());

    println!("Writing all the buffered data to \"../../afd.backup\"");
    fs::write("../../afd.backup", buffer.join("\n")).unwrap();
}

pub fn load_checkpoint() -> AssemblyAnalyzer {
    println!("=== Loading Checkpoint ===");
    let asm_data_regex =
        Regex::new(r#"asm_data:\{\s*asm_path:"(.*)"[\s\S]*?subr_index:[\s\S]*?\]\s*\},"#).unwrap();
    let subroutine_regex = Regex::new(r#"subroutine:\{\s*definition_label:"(.*)",\s*start_line:(\d+),\s*end_line:(\d+),[\s\S]*?necessary:(\w+)\s*\},"#).unwrap();
    let asm_inst_regex = Regex::new(r#"asm_inst:\{\s*raw_content:"(.*)",\s*line_number:(\d+),\s*address:"(\w+)",\s*inst_type:(Cti\(\s*cti_data:\{\s*target_adr:(.+),\s*branch_type:(.+),\s*reachable:(\w+)\s*\}\s*\)|Ncti)\s*\},"#).unwrap();
    let basic_block_regex =
        Regex::new(r"basic_block:\{[\s\S]*?subr_index:(\d+)[\s\S]*?\},").unwrap();
    let edge_regex = Regex::new(r#"edge:\{\s*condition:"(.*)",\s*dest_index:(\d+)\s*\},"#).unwrap();

    let data_string = fs::read_to_string("../../afd.backup").unwrap();

    let mut asm_data_vec = Vec::new();
    println!("Detecting Assembly Data");
    let asm_data_caps_vec = asm_data_regex
        .captures_iter(&data_string)
        .collect::<Vec<_>>();
    for (i, asm_data_caps) in asm_data_caps_vec.iter().enumerate() {
        let asm_path = asm_data_caps[1].to_string();
        let mut subroutines = Vec::new();
        let mut basic_blocks = Vec::new();

        println!("Detecting Subroutines");
        let subroutine_caps_vec = subroutine_regex
            .captures_iter(&asm_data_caps[0])
            .collect::<Vec<_>>();
        for (j, subroutine_caps) in subroutine_caps_vec.iter().enumerate() {
            let definition_label = subroutine_caps[1].to_string();
            let start_line = subroutine_caps[2].parse::<u32>().unwrap();
            let end_line = subroutine_caps[3].parse::<u32>().unwrap();
            let necessary = subroutine_caps[4].parse::<bool>().unwrap();
            let mut assembly_instructions = Vec::new();

            for asm_inst_caps in asm_inst_regex.captures_iter(&subroutine_caps[0]) {
                println!(
                    "Assembly Data {}/{} ({:.1}%) --> Subroutine {}/{} ({:.1}%)",
                    i + 1,
                    asm_data_caps_vec.len(),
                    (i + 1) as f32 / asm_data_caps_vec.len() as f32 * 100.0,
                    j + 1,
                    subroutine_caps_vec.len(),
                    (j + 1) as f32 / subroutine_caps_vec.len() as f32 * 100.0,
                );

                assembly_instructions.push(capture_assembly_instruction(asm_inst_caps));
            }

            subroutines.push(Subroutine {
                definition_label,
                start_line,
                end_line,
                assembly_instructions,
                necessary,
            });
        }

        let basic_block_caps_vec = basic_block_regex
            .captures_iter(&asm_data_caps[0])
            .collect::<Vec<_>>();
        for (j, basic_block_caps) in basic_block_caps_vec.iter().enumerate() {
            let subroutine_index = basic_block_caps[1].parse::<usize>().unwrap();
            let mut assembly_instructions = Vec::new();
            let mut edges = Vec::new();

            for asm_inst_caps in asm_inst_regex.captures_iter(&basic_block_caps[0]) {
                println!(
                    "Assembly Data {}/{} ({:.1}%) --> Basic Block {}/{} ({:.1}%)",
                    i + 1,
                    asm_data_caps_vec.len(),
                    (i + 1) as f32 / asm_data_caps_vec.len() as f32 * 100.0,
                    j + 1,
                    basic_block_caps_vec.len(),
                    (j + 1) as f32 / basic_block_caps_vec.len() as f32 * 100.0,
                );
                assembly_instructions.push(capture_assembly_instruction(asm_inst_caps));
            }

            for edge_caps in edge_regex.captures_iter(&basic_block_caps[0]) {
                println!(
                    "Assembly Data {}/{} ({:.1}%) --> Basic Block {}/{} ({:.1}%)",
                    i + 1,
                    asm_data_caps_vec.len(),
                    (i + 1) as f32 / asm_data_caps_vec.len() as f32 * 100.0,
                    j + 1,
                    basic_block_caps_vec.len(),
                    (j + 1) as f32 / basic_block_caps_vec.len() as f32 * 100.0,
                );

                let condition = edge_caps[1].to_string();
                let destination_index = edge_caps[2].parse::<usize>().unwrap();
                edges.push(Edge {
                    condition,
                    destination_index,
                });
            }

            basic_blocks.push(BasicBlock {
                assembly_instructions,
                edges,
                subroutine_index,
            });
        }

        asm_data_vec.push(AssemblyData {
            asm_path,
            subroutines,
            basic_blocks,
        });
    }

    let cfg = ControlFlowGraph { asm_data_vec };

    AssemblyAnalyzer { cfg }
}

fn capture_assembly_instruction(asm_inst_caps: Captures) -> AssemblyInstruction {
    let raw_content = asm_inst_caps[1].to_string();
    let line_number = asm_inst_caps[2].parse::<u32>().unwrap();
    let address = asm_inst_caps[3].to_string();
    let instruction_type_string = &asm_inst_caps[4];
    let instruction_type = if instruction_type_string.starts_with("Cti") {
        let cti_target_address_string = &asm_inst_caps[5];
        let (cti_ta_key, cti_ta_value) = cti_target_address_string.split_once('(').unwrap();
        // Dropping the closing paranthesis ( ) ).
        let cti_ta_value = cti_ta_value[0..cti_ta_value.len() - 1].to_string();
        let cti_target_address = match cti_ta_key {
            "Direct" => TargetAddress::Direct(cti_ta_value),
            "Indirect" => TargetAddress::Indirect(cti_ta_value),
            _ => {
                panic!("Error in reading target address value in load_checkpoint function.")
            }
        };
        let cti_branch_type_string = &asm_inst_caps[6];
        let (cti_bt_key, cti_bt_value) = cti_branch_type_string.split_once('(').unwrap();
        let cti_bt_value = &cti_bt_value[0..cti_bt_value.len() - 1];
        let cti_branch_type = match cti_bt_key {
            "ConditionalBranch" => {
                let conditional_branch = match cti_bt_value {
                    "b.eq" => ConditionalBranch::BDotEq,
                    "b.ne" => ConditionalBranch::BDotNe,
                    "b.cs" => ConditionalBranch::BDotCs,
                    "b.cc" => ConditionalBranch::BDotCc,
                    "b.mi" => ConditionalBranch::BDotMi,
                    "b.pl" => ConditionalBranch::BDotPl,
                    "b.vs" => ConditionalBranch::BDotVs,
                    "b.vc" => ConditionalBranch::BDotVc,
                    "b.hi" => ConditionalBranch::BDotHi,
                    "b.ls" => ConditionalBranch::BDotLs,
                    "b.ge" => ConditionalBranch::BDotGe,
                    "b.lt" => ConditionalBranch::BDotLt,
                    "b.gt" => ConditionalBranch::BDotGt,
                    "b.le" => ConditionalBranch::BDotLe,
                    "bc.eq" => ConditionalBranch::BCDotEq,
                    "bc.ne" => ConditionalBranch::BCDotNe,
                    "bc.cs" => ConditionalBranch::BCDotCs,
                    "bc.cc" => ConditionalBranch::BCDotCc,
                    "bc.mi" => ConditionalBranch::BCDotMi,
                    "bc.pl" => ConditionalBranch::BCDotPl,
                    "bc.vs" => ConditionalBranch::BCDotVs,
                    "bc.vc" => ConditionalBranch::BCDotVc,
                    "bc.hi" => ConditionalBranch::BCDotHi,
                    "bc.ls" => ConditionalBranch::BCDotLs,
                    "bc.ge" => ConditionalBranch::BCDotGe,
                    "bc.lt" => ConditionalBranch::BCDotLt,
                    "bc.gt" => ConditionalBranch::BCDotGt,
                    "bc.le" => ConditionalBranch::BCDotLe,
                    "cbz" => ConditionalBranch::Cbz,
                    "cbnz" => ConditionalBranch::Cbnz,
                    "tbz" => ConditionalBranch::Tbz,
                    "tbnz" => ConditionalBranch::Tbnz,
                    _ => {
                        panic!("Error in reading branch type value in load_checkpoint function.")
                    }
                };
                BranchType::ConditionalBranch(conditional_branch)
            }
            "UnconditionalBranch" => {
                let unconditional_branch = match cti_bt_value {
                    "b" => UnconditionalBranch::B,
                    "bl" => UnconditionalBranch::Bl,
                    "ret" => UnconditionalBranch::Ret,
                    "br" => UnconditionalBranch::Br,
                    "blr" => UnconditionalBranch::Blr,
                    "blraa" => UnconditionalBranch::Blraa,
                    "blraaz" => UnconditionalBranch::Blraaz,
                    "blrab" => UnconditionalBranch::Blrab,
                    "blrabz" => UnconditionalBranch::Blrabz,
                    "b.al" => UnconditionalBranch::BDotAl,
                    "bc.al" => UnconditionalBranch::BCDotAl,
                    "b.nv" => UnconditionalBranch::BDotNv,
                    "bc.nv" => UnconditionalBranch::BCDotNv,
                    _ => {
                        panic!("Error in reading branch type value in load_checkpoint function.")
                    }
                };
                BranchType::UnconditionalBranch(unconditional_branch)
            }
            _ => {
                panic!("Error in reading branch type value in load_checkpoint function.")
            }
        };

        let cti_reachable = asm_inst_caps[7].parse::<bool>().unwrap();

        InstructionType::Cti(CtiData {
            target_address: cti_target_address,
            branch_type: cti_branch_type,
            reachable: cti_reachable,
        })
    } else {
        InstructionType::Ncti
    };

    AssemblyInstruction {
        raw_content,
        line_number,
        address,
        instruction_type,
    }
}
