use std::{collections::HashSet, fs};

use regex::Regex;

use crate::module::{
    analyzer::Subroutine,
    cfg::Edge,
    instruction::{AssemblyInstruction, TargetAddress},
};

pub struct BasicBlock {
    pub assembly_instructions: Vec<AssemblyInstruction>,
    pub edges: Vec<Edge>,
    pub subroutine_index: usize,
}

impl BasicBlock {
    pub fn new(
        asm_instructions: Vec<AssemblyInstruction>,
        edges: Vec<Edge>,
        subroutine_index: usize,
    ) -> Self {
        Self {
            assembly_instructions: asm_instructions,
            edges,
            subroutine_index,
        }
    }

    pub fn get_address(&self) -> String {
        self.assembly_instructions.first().unwrap().address()
    }

    pub fn next_basic_block_adr(&self) -> String {
        let last_bb_adr = self.assembly_instructions.last().unwrap().address();
        let next_bb_adr = u32::from_str_radix(&last_bb_adr, 16).unwrap() + 4;

        format!("{:x}", next_bb_adr)
    }

    pub fn last_asm_instruction(&self) -> &AssemblyInstruction {
        self.assembly_instructions.last().unwrap()
    }
}

pub fn extract_leaders(
    subroutines: &mut [Subroutine],
    trace_path: &str,
    indirect_branch_in_trace_regex: &Regex,
) -> Vec<AssemblyInstruction> {
    println!("=== Extracting Leaders and Finding Possible Dynamic Target Addresses ===");

    let mut leaders = HashSet::new();

    // First assembly instruction of the first subroutine is a leader.
    leaders.insert(subroutines[0].assembly_instructions[0].clone());

    let mut resolved_dynamic_target_address_count = 0;
    let mut subroutines_counter = 0;
    for i in 0..subroutines.len() {
        subroutines_counter += 1;
        let mut next_asm_ins_is_leader = false;

        let mut j = 0;
        while j < subroutines[i].assembly_instructions.len() {
            match subroutines[i].assembly_instructions[j].get_target_address() {
                Some(bta) => {
                    match bta {
                        TargetAddress::Direct(target_address) => {
                            // The target instruction of a jump instruction is a leader.
                            if let Some(target_asm_ins) =
                                find_asm_ins_by_address(subroutines, &target_address)
                            {
                                leaders.insert(target_asm_ins);
                            } else {
                                let last_asm_ins_adr = u32::from_str_radix(
                                    subroutines
                                        .last()
                                        .unwrap()
                                        .assembly_instructions
                                        .last()
                                        .unwrap()
                                        .address
                                        .as_str(),
                                    16,
                                )
                                .unwrap();
                                if target_address == format!("{:x}", last_asm_ins_adr + 4) {
                                    println!(
                                        "Info: Instruction at address \"0x{target_address}\" was not found in the program's assembly.\nIt's the address of the first instruction that comes after the last instruction of the program (\"0x{last_asm_ins_adr:x}\")"
                                    );
                                } else {
                                    panic!(
                                        "Error: No insturciton with the given address was found: \"{target_address}\"."
                                    );
                                }
                            }
                        }
                        TargetAddress::Indirect(_register) => {
                            let target_address = extract_target_address_from_gem5_trace(
                                trace_path,
                                indirect_branch_in_trace_regex,
                                &subroutines[i].assembly_instructions[j].address(),
                            );

                            if let Some(target_address) = target_address {
                                subroutines[i].assembly_instructions[j]
                                    .set_direct_target_address(target_address);
                                resolved_dynamic_target_address_count += 1;
                                continue;
                            } else {
                                subroutines[i].assembly_instructions[j].flag_as_unreachable();
                            }
                        }
                    }

                    // If this instruction is followed by a jump instruction (next_asm_ins_is_leader == true), it is a leader.
                    if next_asm_ins_is_leader {
                        leaders.insert(subroutines[i].assembly_instructions[j].clone());
                    } else {
                        next_asm_ins_is_leader = true;
                    }
                }
                None => {
                    if next_asm_ins_is_leader {
                        leaders.insert(subroutines[i].assembly_instructions[j].clone());
                        next_asm_ins_is_leader = false;
                    }
                }
            }

            j += 1;
        }
        let sub_len = subroutines.len();
        println!(
            "Subroutine {subroutines_counter}/{sub_len} (Progress: {:.1}%), Leader Count: {}, Resolved Dynamic Target Address Count: {}",
            (subroutines_counter as f32 / sub_len as f32 * 100.0),
            leaders.len(),
            resolved_dynamic_target_address_count
        );
    }

    let mut leaders = leaders.into_iter().collect::<Vec<_>>();
    leaders.sort_unstable_by_key(|ai| ai.line_number);

    leaders
}

pub fn extract_basic_blocks(
    subroutines: &[Subroutine],
    leaders: Vec<AssemblyInstruction>,
) -> Vec<BasicBlock> {
    let mut basic_blocks = Vec::new();
    let mut asm_insts = Vec::new();
    let mut leader_iter = leaders.into_iter();

    // The first leader should be skipped because it is always the first instruction and is not needed for basic block detection.
    leader_iter.next();

    if let Some(mut current_leader) = leader_iter.next() {
        'outer: for (sub_index, subroutine) in subroutines.iter().enumerate() {
            for (ai_index, asm_ins) in subroutine.assembly_instructions.iter().enumerate() {
                // We compare line numbers because there's lower overhead doing this (they are u32) compared to string comparision.
                if asm_ins.line_number == current_leader.line_number {
                    basic_blocks.push(BasicBlock::new(asm_insts, Vec::new(), sub_index));
                    asm_insts = Vec::new();

                    if let Some(next_leader) = leader_iter.next() {
                        current_leader = next_leader;

                        asm_insts.push(asm_ins.clone());
                    } else {
                        for sub_index in sub_index..subroutines.len() {
                            for ai_index in
                                ai_index..subroutines[sub_index].assembly_instructions.len()
                            {
                                asm_insts.push(
                                    subroutines[sub_index].assembly_instructions[ai_index].clone(),
                                );
                            }
                        }
                        break 'outer;
                    }
                } else {
                    asm_insts.push(asm_ins.clone());
                }
            }
        }
    } else {
        for sub_index in 0..subroutines.len() {
            for ai_index in 0..subroutines[sub_index].assembly_instructions.len() {
                asm_insts.push(subroutines[sub_index].assembly_instructions[ai_index].clone());
            }
        }
    }
    // This is the last basic block.
    basic_blocks.push(BasicBlock::new(
        asm_insts,
        Vec::new(),
        subroutines.len() - 1,
    ));

    basic_blocks
}

fn extract_target_address_from_gem5_trace(
    trace_path: &str,
    indirect_branch_in_trace_regex: &Regex,
    asm_ins_address: &str,
) -> Option<String> {
    let trace = fs::read_to_string(trace_path).unwrap();

    for line in trace.lines() {
        if let Some(cap) = indirect_branch_in_trace_regex.captures(line) {
            if cap.get(1).unwrap().as_str() == asm_ins_address {
                return Some(cap.get(2).unwrap().as_str().to_string());
            }
        }
    }

    None
}

pub fn find_asm_ins_by_address(
    subroutines: &[Subroutine],
    asm_ins_address: &str,
) -> Option<AssemblyInstruction> {
    for subroutine in subroutines {
        for asm_ins in &subroutine.assembly_instructions {
            if asm_ins.address() == asm_ins_address {
                return Some(asm_ins.clone());
            }
        }
    }

    None
}

pub fn find_basic_block_index_by_address(
    basic_blocks: &[BasicBlock],
    basic_block_address: &str,
) -> Option<usize> {
    for (i, basic_block) in basic_blocks.iter().enumerate() {
        if basic_block_address == basic_block.get_address() {
            return Some(i);
        }
    }

    None
}
