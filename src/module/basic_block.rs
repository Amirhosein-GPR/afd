use std::collections::{HashSet, VecDeque};

use crate::module::{
    assembly::{AssemblyInstruction, AssemblyInstructionId, AssemblyInstructionType, Subroutine},
    cfg::Edge,
};

pub struct BasicBlock {
    pub asm_inst_ids: Vec<AssemblyInstructionId>,
    pub edges: Vec<Edge>,
}

impl BasicBlock {
    pub fn new(asm_inst_ids: Vec<AssemblyInstructionId>, edges: Vec<Edge>) -> Self {
        Self {
            asm_inst_ids,
            edges,
        }
    }

    pub fn last_asm_instruction<'a>(
        &self,
        subroutines: &'a [Subroutine],
    ) -> &'a AssemblyInstruction {
        let last_asm_inst_id = self.asm_inst_ids.last().unwrap();
        &subroutines[last_asm_inst_id.subroutine_index].asm_insts[last_asm_inst_id.asm_inst_index]
    }
}

pub fn extract_leaders_ids(subroutines: &mut [Subroutine]) -> Vec<AssemblyInstructionId> {
    println!("=== Extracting Leaders ===");

    let mut leader_ids = HashSet::new();

    let main_subroutine_index = Subroutine::get_main_subroutine_index(subroutines);
    subroutines[main_subroutine_index].necessary = true;
    // First instruciton of the main subroutine is a leader.
    leader_ids.insert(AssemblyInstructionId {
        subroutine_index: main_subroutine_index,
        asm_inst_index: 0,
    });

    // nec_sub_ind = necessary_subroutines_indices
    let mut nec_sub_ind_hs = HashSet::new();
    let mut nec_sub_ind_vec = VecDeque::new();
    nec_sub_ind_hs.insert(main_subroutine_index);
    nec_sub_ind_vec.push_back(main_subroutine_index);

    let mut return_stack = Vec::new();
    let mut j = 0;

    while let Some(mut i) = nec_sub_ind_vec.pop_front() {
        'outer_loop: loop {
            while j < subroutines[i].asm_insts.len() {
                let asm_inst = &subroutines[i].asm_insts[j];

                if i == main_subroutine_index && j == subroutines[i].asm_insts.len() - 1 {
                    let mut leader_ids = leader_ids.into_iter().collect::<Vec<_>>();
                    leader_ids.sort_unstable();

                    return leader_ids;
                }

                let target_ids = asm_inst.get_target_ids(subroutines);

                for target_id in &target_ids {
                    leader_ids.insert(target_id.clone());
                }

                match &asm_inst.inst_type {
                    AssemblyInstructionType::CtiUnconditional(inst_name, _ta) => {
                        if inst_name == "bl" || inst_name == "blr" {
                            if nec_sub_ind_hs.insert(target_ids[0].subroutine_index) {
                                nec_sub_ind_vec.push_back(target_ids[0].subroutine_index);
                                subroutines[target_ids[0].subroutine_index].necessary = true;

                                return_stack.push(AssemblyInstructionId {
                                    subroutine_index: i,
                                    asm_inst_index: j + 1,
                                });

                                i = target_ids[0].subroutine_index;
                                j = target_ids[0].asm_inst_index;

                                continue 'outer_loop;
                            }
                        } else if inst_name == "ret" {
                            if let Some(return_asm_inst_id) = return_stack.pop() {
                                i = return_asm_inst_id.subroutine_index;
                                j = return_asm_inst_id.asm_inst_index;

                                continue 'outer_loop;
                            } else if i != main_subroutine_index {
                                panic!("No assembly instruciton return id is found!");
                            }
                        }
                    }
                    _ => {}
                }

                j += 1;
            }

            break;
        }
    }

    panic!(
        "Leader extraction failed! Couldn't analyze till the last assembly instruciton of the main subroutine!"
    );
}

pub fn extract_basic_blocks(
    subroutines: &[Subroutine],
    leader_ids: Vec<AssemblyInstructionId>,
) -> Vec<BasicBlock> {
    println!("=== Extracting Basic Blocks ===");

    let mut basic_blocks = Vec::new();
    let mut asm_inst_ids = Vec::new();

    for i in 0..leader_ids.len() {
        let mut current_asm_inst_id = leader_ids[i].clone();

        if i + 1 < leader_ids.len() {
            let next_leader_id = &leader_ids[i + 1];

            loop {
                for j in (current_asm_inst_id.asm_inst_index)
                    ..subroutines[current_asm_inst_id.subroutine_index]
                        .asm_insts
                        .len()
                {
                    if current_asm_inst_id.subroutine_index == next_leader_id.subroutine_index
                        && j == next_leader_id.asm_inst_index
                    {
                        basic_blocks.push(BasicBlock::new(asm_inst_ids, Vec::new()));
                        asm_inst_ids = Vec::new();
                        break;
                    } else {
                        asm_inst_ids.push(AssemblyInstructionId {
                            subroutine_index: current_asm_inst_id.subroutine_index,
                            asm_inst_index: j,
                        });
                    }
                }
                // It means that the above if condition (if current_leader_id...) has been evaluated to true and we have reached the next leader (leader_ids[i+1]).
                if asm_inst_ids.len() == 0 {
                    break;
                } else {
                    current_asm_inst_id = AssemblyInstructionId {
                        subroutine_index: current_asm_inst_id.subroutine_index + 1,
                        asm_inst_index: 0,
                    }
                }
            }
        } else {
            for j in (current_asm_inst_id.asm_inst_index)
                ..subroutines[current_asm_inst_id.subroutine_index]
                    .asm_insts
                    .len()
            {
                asm_inst_ids.push(AssemblyInstructionId {
                    subroutine_index: current_asm_inst_id.subroutine_index,
                    asm_inst_index: j,
                });
            }

            basic_blocks.push(BasicBlock::new(asm_inst_ids, Vec::new()));
            break;
        }
    }

    basic_blocks
}

pub fn find_basic_block_index_by_id(
    basic_blocks: &[BasicBlock],
    basic_block_id: &AssemblyInstructionId,
) -> Option<usize> {
    for (i, basic_block) in basic_blocks.iter().enumerate() {
        if basic_block_id == basic_block.asm_inst_ids.first().unwrap() {
            return Some(i);
        }
    }

    None
}
