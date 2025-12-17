use std::{collections::HashSet, fs};

use regex::Regex;

use crate::module::{
    analyzer::Subroutine,
    instruction::{self, AssemblyInstruction, BranchTargetAddress},
};

pub struct BasicBlock {
    pub starting_instruction: AssemblyInstruction,
    pub ending_instruction: AssemblyInstruction,
}

pub fn extract_leaders(
    trace_path: &str,
    indirect_branch_in_trace_regex: &Regex,
    subroutines: &[Subroutine],
) -> HashSet<AssemblyInstruction> {
    let mut leaders = HashSet::new();

    // First assembly instruction of the first subroutine is a leader.
    leaders.insert(subroutines[0].assembly_instructions[0].clone());

    for subroutine in subroutines {
        let mut next_asm_ins_is_leader = false;

        for i in 0..subroutine.assembly_instructions.len() {
            let asm_ins = subroutine.assembly_instructions[i].clone();

            match extract_branch_target_address(&asm_ins) {
                Some(bta) => match bta {
                    BranchTargetAddress::Direct(target_address) => {
                        let target_asm_ins = find_asm_ins_by_address(subroutines, &target_address)
                            .expect("Error: No insturciton with the given address was found.");

                        // If this instruction is followed by a jump instruction (next_asm_ins_is_leader == true), it is a leader.
                        if next_asm_ins_is_leader {
                            leaders.insert(asm_ins);
                        } else {
                            next_asm_ins_is_leader = true;
                        }

                        // The target instruction of a jump instruction is a leader.
                        leaders.insert(target_asm_ins);
                    }
                    BranchTargetAddress::Indirect(_register) => {
                        let target_address = extract_target_address_from_gem5_trace(
                            trace_path,
                            indirect_branch_in_trace_regex,
                            &asm_ins.address(),
                        );

                        // TODO FROM HERE
                        if let Some(target_address) = target_address {
                            let target_asm_ins =
                                find_asm_ins_by_address(subroutines, &target_address).expect(
                                    "Error: No insturciton with the given address was found.",
                                );

                            // If this instruction is followed by a jump instruction (next_asm_ins_is_leader == true), it is a leader.
                            if next_asm_ins_is_leader {
                                leaders.insert(asm_ins);
                            } else {
                                next_asm_ins_is_leader = true;
                            }

                            // The target instruction of a jump instruction is a leader.
                            leaders.insert(target_asm_ins);
                        } else {
                            // OR TODO FROM HERE
                            // "Error: No target address was found for the given branch instruction address.",
                        }
                    }
                },
                None => {
                    if next_asm_ins_is_leader {
                        leaders.insert(asm_ins);
                        next_asm_ins_is_leader = false;
                    }
                }
            }
        }
    }

    leaders
}

pub fn extract_basic_blocks(
    subroutines: &[Subroutine],
    leaders: HashSet<AssemblyInstruction>,
) -> Vec<BasicBlock> {
    let mut basic_blocks = Vec::new();
    let mut leader_iter = leaders.into_iter();
    let mut start_ins = leader_iter.next().unwrap();
    let mut prev_asm_ins = start_ins.clone();
    let mut current_leader = leader_iter.next().unwrap();

    'outer: for subroutine in subroutines {
        for asm_ins in &subroutine.assembly_instructions {
            if *asm_ins == current_leader {
                basic_blocks.push(BasicBlock {
                    starting_instruction: start_ins,
                    ending_instruction: prev_asm_ins,
                });
                start_ins = asm_ins.clone();

                if let Some(leader) = leader_iter.next() {
                    current_leader = leader;
                } else {
                    break 'outer;
                }
            }

            prev_asm_ins = asm_ins.clone();
        }
    }

    basic_blocks
}

/// ## Returns
/// - `Some(BranchTargetAddress)`: If the `AssemblyInstruction` is a `ControlFlowInstruciton`
/// - `None`: Otherwise
pub fn extract_branch_target_address(
    asm_ins: &AssemblyInstruction,
    indirect_branch_regex: &Regex,
) -> Option<BranchTargetAddress> {
    match &asm_ins.instruction_type {
        instruction::InstructionType::Cti(cti_data) => match &cti_data.branch_type {
            instruction::BranchType::ConditionalBranch(cb) => match cb {
                _ => Some(BranchTargetAddress::Direct(
                    asm_ins.target_address().unwrap(),
                )),
            },
            instruction::BranchType::UnconditionalBranch(ub) => match ub {
                instruction::UnconditionalBranch::Ret
                | instruction::UnconditionalBranch::Br
                | instruction::UnconditionalBranch::Blr => Some(BranchTargetAddress::Indirect(
                    asm_ins.target_address().unwrap(),
                )),
                _ => Some(BranchTargetAddress::Direct(
                    asm_ins.target_address().unwrap(),
                )),
            },
        },
        instruction::InstructionType::Ncti => None,
    }
}

fn extract_target_address_from_gem5_trace(
    trace_path: &str,
    indirect_branch_in_trace_regex: &Regex,
    asm_ins_address: &str,
) -> Option<String> {
    let trace = fs::read_to_string(trace_path).unwrap();

    for line in trace.lines() {
        if let Some(cap) = indirect_branch_in_trace_regex.captures(line) {
            dbg!(&asm_ins_address, cap.get(1).unwrap().as_str());
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
