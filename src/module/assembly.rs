use std::fs;

use regex::Regex;

/// Represents a subroutine in an assembly source file.
///
/// It cotains some useful info about a subroutine in an assembly file.
pub struct Subroutine {
    pub definition_label: String,
    pub start_line: usize,
    pub end_line: usize,
    pub asm_insts: Vec<AssemblyInstruction>,
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

    // TODO: Might be able to increase performance of it later.
    pub fn get_main_subroutine_index(subroutines: &[Subroutine]) -> usize {
        for (i, subroutine) in subroutines.iter().enumerate() {
            if subroutine.definition_label.contains("<main>:") {
                return i;
            }
        }

        panic!("Main subroutine was not found!");
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct AssemblyInstructionId {
    pub subroutine_index: usize,
    pub asm_inst_index: usize,
}

/// Represents a control transfaer instruction (aka, jump instruction) in an assembly file.
///
/// It contains some useful information about each CTI.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct AssemblyInstruction {
    pub id: AssemblyInstructionId,
    pub source_line_number: usize,
    pub asm_line_number: usize,
    pub content: String,
    pub inst_type: AssemblyInstructionType,
}

type TargetId = AssemblyInstructionId;
type InstructionName = String;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum AssemblyInstructionType {
    CtiConditional(InstructionName, Option<TargetId>),
    CtiUnconditional(InstructionName, Option<TargetId>),
    Ncti(InstructionName),
}

impl AssemblyInstruction {
    pub fn new(
        id: AssemblyInstructionId,
        source_line_number: usize,
        asm_line_number: usize,
        content: &str,
        instruction_name_regex: &Regex,
    ) -> Self {
        let instruction_name = instruction_name_regex
            .captures(content)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();

        let inst_type = match instruction_name {
            "b.eq" | "b.ne" | "b.cs" | "b.cc" | "b.mi" | "b.pl" | "b.vs" | "b.vc" | "b.hi"
            | "b.ls" | "b.ge" | "b.lt" | "b.gt" | "b.le" | "bc.eq" | "bc.ne" | "bc.cs"
            | "bc.cc" | "bc.mi" | "bc.pl" | "bc.vs" | "bc.vc" | "bc.hi" | "bc.ls" | "bc.ge"
            | "bc.lt" | "bc.gt" | "bc.le" | "cbz" | "cbnz" | "tbz" | "tbnz" => {
                AssemblyInstructionType::CtiConditional(instruction_name.to_string(), None)
            }
            "b" | "bl" | "ret" | "br" | "blr" | "b.al" | "bc.al" | "b.nv" | "bc.nv" => {
                AssemblyInstructionType::CtiUnconditional(instruction_name.to_string(), None)
            }
            _ => AssemblyInstructionType::Ncti(instruction_name.to_string()),
        };

        Self {
            id,
            source_line_number,
            asm_line_number,
            content: content.to_string(),
            inst_type,
        }
    }

    pub fn branch_condition(&self) -> Result<String, String> {
        match &self.inst_type {
            AssemblyInstructionType::CtiConditional(inst_name, _ta)
            | AssemblyInstructionType::CtiUnconditional(inst_name, _ta) => {
                Ok(match inst_name.as_str() {
                    "b.eq" => "Z == 1",
                    "b.ne" => "Z == 0",
                    "b.cs" => "C == 1",
                    "b.cc" => "C == 0",
                    "b.mi" => "N == 1",
                    "b.pl" => "N == 0",
                    "b.vs" => "V == 1",
                    "b.vc" => "V == 0",
                    "b.hi" => "C == 1 and Z == 0",
                    "b.ls" => "C == 0 or Z == 1",
                    "b.ge" => "N == V",
                    "b.lt" => "N != V",
                    "b.gt" => "Z == 0 and N == V",
                    "b.le" => "Z == 1 or N != V",
                    "bc.eq" => "Z == 1",
                    "bc.ne" => "Z == 0",
                    "bc.cs" => "C == 1",
                    "bc.cc" => "C == 0",
                    "bc.mi" => "N == 1",
                    "bc.pl" => "N == 0",
                    "bc.vs" => "V == 1",
                    "bc.vc" => "V == 0",
                    "bc.hi" => "C == 1 and Z == 0",
                    "bc.ls" => "C == 0 or Z == 1",
                    "bc.ge" => "N == V",
                    "bc.lt" => "N != V",
                    "bc.gt" => "Z == 0 and N == V",
                    "bc.le" => "Z == 1 or N != V",
                    "cbz" => "register == 0",
                    "cbnz" => "register != 0",
                    "tbz" => "selected bit == 0",
                    "tbnz" => "selected bit == 1",
                    _ => "true",
                }
                .to_string())
            }

            AssemblyInstructionType::Ncti(_inst_name) => Err(format!(
                "NCTI Instruction: \"{}\" has no branch condition!",
                self.content
            )),
        }
    }

    pub fn branch_condition_complement(&self) -> Result<String, String> {
        match &self.inst_type {
            AssemblyInstructionType::CtiConditional(inst_name, _ta)
            | AssemblyInstructionType::CtiUnconditional(inst_name, _ta) => {
                Ok(match inst_name.as_str() {
                    "b.eq" => "Z != 1",
                    "b.ne" => "Z != 0",
                    "b.cs" => "C != 1",
                    "b.cc" => "C != 0",
                    "b.mi" => "N != 1",
                    "b.pl" => "N != 0",
                    "b.vs" => "V != 1",
                    "b.vc" => "V != 0",
                    "b.hi" => "C != 1 or Z != 0",
                    "b.ls" => "C != 0 and Z != 1",
                    "b.ge" => "N != V",
                    "b.lt" => "N == V",
                    "b.gt" => "Z != 0 or N != V",
                    "b.le" => "Z != 1 and N == V",
                    "bc.eq" => "Z != 1",
                    "bc.ne" => "Z != 0",
                    "bc.cs" => "C != 1",
                    "bc.cc" => "C != 0",
                    "bc.mi" => "N != 1",
                    "bc.pl" => "N != 0",
                    "bc.vs" => "V != 1",
                    "bc.vc" => "V != 0",
                    "bc.hi" => "C != 1 or Z != 0",
                    "bc.ls" => "C != 0 and Z != 1",
                    "bc.ge" => "N != V",
                    "bc.lt" => "N == V",
                    "bc.gt" => "Z != 0 or N != V",
                    "bc.le" => "Z != 1 and N == V",
                    "cbz" => "register != 0",
                    "cbnz" => "register == 0",
                    "tbz" => "selected bit != 0",
                    "tbnz" => "selected bit != 1",
                    _ => "false",
                }
                .to_string())
            }

            AssemblyInstructionType::Ncti(_inst_name) => Err(format!(
                "NCTI Instruction: \"{}\" has no complement branch condition!",
                self.content
            )),
        }
    }
    // TODO: Might be able to increase performance of it later.
    pub fn get_address(&self) -> &str {
        self.content.split_once(":").unwrap().0
    }

    pub fn get_target_id(&self) -> Result<AssemblyInstructionId, String> {
        match &self.inst_type {
            AssemblyInstructionType::CtiConditional(_inst_name, ta) => {
                if let Some(ta) = ta {
                    return Ok(ta.clone());
                } else {
                    Err(format!(
                        "Target Address is not previously extracted for the assembly instruction: \"{}\"",
                        self.content
                    ))
                }
            }
            AssemblyInstructionType::CtiUnconditional(_inst_name, ta) => {
                if let Some(ta) = ta {
                    return Ok(ta.clone());
                } else {
                    Err(format!(
                        "Target Address is not previously extracted for the assembly instruction: \"{}\"",
                        self.content
                    ))
                }
            }
            AssemblyInstructionType::Ncti(_inst_name) => {
                return Err(format!(
                    "NCTI (Non Control Transfer Instruction) has no target address: \"{}\"",
                    self.content
                ));
            }
        }
    }

    fn extract_target_address_from_gem5_trace(
        &self,
        trace_path: &str,
        indirect_branch_in_trace_regex: &Regex,
    ) -> Result<String, String> {
        let trace = fs::read_to_string(trace_path).unwrap();

        for line in trace.lines() {
            if let Some(cap) = indirect_branch_in_trace_regex.captures(line) {
                if cap.get(1).unwrap().as_str() == self.get_address() {
                    return Ok(cap.get(2).unwrap().as_str().to_string());
                }
            }
        }

        Err(format!(
            "No dynamic target address was found for the assembly instruction: \"{}\" on line: {}",
            self.content, self.asm_line_number
        ))
    }

    //TODO REMOVE OR NOT?
    /// Returns the line number of the first assembly instruction in the subroutine.
    /// It can return None if the address is outside of the program.
    pub fn get_subroutine_index_from_address(
        &self,
        subroutines: &[Subroutine],
        address: &str,
    ) -> Result<usize, String> {
        for (i, subroutine) in subroutines.iter().enumerate() {
            for asm_line in &subroutine.asm_insts {
                if asm_line.get_address() == address {
                    return Ok(i);
                }
            }
        }

        Err(format!(
            "No subroutine was found with an assembly instruction line with the address: \"{}\"",
            address
        ))
    }

    pub fn get_target_ids(&self, subroutines: &[Subroutine]) -> Vec<AssemblyInstructionId> {
        match &self.inst_type {
            AssemblyInstructionType::CtiConditional(_inst_name, _ta) => {
                let target_inst_id = self.get_target_id().unwrap();
                let next_inst_id = self.next_asm_inst_id(subroutines);

                vec![target_inst_id, next_inst_id]
            }
            AssemblyInstructionType::CtiUnconditional(inst_name, _ta) => {
                let target_id = self.get_target_id().unwrap();

                if inst_name == "ret"
                    && let Some(next_id) = self.next_asm_inst_id_in_the_same_subroutine(subroutines)
                {
                    vec![target_id, next_id]
                } else {
                    vec![target_id]
                }
            }
            AssemblyInstructionType::Ncti(_inst_name) => vec![],
        }
    }

    pub fn next_asm_inst_id(&self, subroutines: &[Subroutine]) -> AssemblyInstructionId {
        if self.id.asm_inst_index + 1 < subroutines[self.id.subroutine_index].asm_insts.len() {
            AssemblyInstructionId {
                subroutine_index: self.id.subroutine_index,
                asm_inst_index: self.id.asm_inst_index + 1,
            }
        } else {
            panic!(
                "No next assembly instruction id was found! Current assembly instruction is: \"{}\"",
                self.content
            );
        }

        // if self.id.asm_inst_index + 1 < subroutines[self.id.subroutine_index].asm_lines.len() {
        //     AssemblyInstructionId {
        //         subroutine_index: self.id.subroutine_index,
        //         asm_inst_index: self.id.asm_inst_index + 1,
        //     }
        // } else if self.id.subroutine_index + 1 < subroutines.len() {
        //     AssemblyInstructionId {
        //         subroutine_index: self.id.subroutine_index + 1,
        //         asm_inst_index: 0,
        //     }
        // } else {
        //     panic!(
        //         "No next assembly instruction id was found! Current assembly instruction is: \"{}\"",
        //         self.content
        //     );
        // }
    }

    pub fn next_asm_inst_address(&self) -> String {
        let next_bb_adr = usize::from_str_radix(self.get_address(), 16).unwrap() + 4;

        format!("{:x}", next_bb_adr)
    }

    pub fn get_asm_inst_id_by_address(
        subroutines: &[Subroutine],
        address: &str,
    ) -> Result<AssemblyInstructionId, String> {
        for (i, subroutine) in subroutines.iter().enumerate() {
            for (j, asm_line) in subroutine.asm_insts.iter().enumerate() {
                if asm_line.get_address() == address {
                    return Ok(AssemblyInstructionId {
                        subroutine_index: i,
                        asm_inst_index: j,
                    });
                }
            }
        }

        Err(format!(
            "No asssembly instruction was found with the address: \"{}\"",
            address
        ))
    }

    pub fn next_asm_inst_id_in_the_same_subroutine(
        &self,
        subroutines: &[Subroutine],
    ) -> Option<AssemblyInstructionId> {
        let asm_inst_id =
            AssemblyInstruction::get_asm_inst_id_by_address(subroutines, self.get_address())
                .unwrap();

        let next_asm_inst_id = AssemblyInstructionId {
            subroutine_index: asm_inst_id.subroutine_index,
            asm_inst_index: asm_inst_id.asm_inst_index + 1,
        };

        if subroutines[next_asm_inst_id.subroutine_index]
            .asm_insts
            .get(next_asm_inst_id.asm_inst_index)
            .is_some()
        {
            Some(next_asm_inst_id)
        } else {
            None
        }
    }
}

pub fn compute_target_ids(
    subroutines: &mut [Subroutine],
    trace_path: &str,
    gem5_trace_indirect_branch_regex: &Regex,
) {
    let main_sub_index = Subroutine::get_main_subroutine_index(&subroutines);
    let mut i = main_sub_index;
    let mut j = 0;
    let mut return_stack = Vec::new();

    'main_loop: loop {
        while i < subroutines.len() {
            while j < subroutines[i].asm_insts.len() {
                let asm_inst = &subroutines[i].asm_insts[j];
                println!("Current asm_inst: {}", asm_inst.content);

                let target_id = match &asm_inst.inst_type {
                    AssemblyInstructionType::CtiConditional(_inst_name, _ta) => {
                        let target_adr = asm_inst.content.split_whitespace().nth(3).unwrap();
                        let target_id = AssemblyInstruction::get_asm_inst_id_by_address(
                            subroutines,
                            target_adr,
                        )
                        .unwrap();

                        target_id
                    }
                    AssemblyInstructionType::CtiUnconditional(inst_name, _ta) => {
                        if inst_name == "bl" {
                            let target_adr = asm_inst.content.split_whitespace().nth(3).unwrap();
                            let target_id = AssemblyInstruction::get_asm_inst_id_by_address(
                                subroutines,
                                target_adr,
                            )
                            .unwrap();

                            let return_id = AssemblyInstructionId {
                                subroutine_index: i,
                                asm_inst_index: j + 1,
                            };

                            return_stack.push(return_id);

                            target_id
                        } else if inst_name == "blr" {
                            println!(
                                "Dynamic target address in the assembly instruction: \"{}\" was detected! Trying to resolve it.",
                                asm_inst.content
                            );

                            let target_address = asm_inst
                                .extract_target_address_from_gem5_trace(
                                    trace_path,
                                    gem5_trace_indirect_branch_regex,
                                )
                                .unwrap();
                            let target_id = AssemblyInstruction::get_asm_inst_id_by_address(
                                subroutines,
                                &target_address,
                            )
                            .unwrap();

                            let return_id = AssemblyInstructionId {
                                subroutine_index: i,
                                asm_inst_index: j + 1,
                            };

                            return_stack.push(return_id);

                            target_id
                        } else if inst_name == "br" {
                            println!(
                                "Dynamic target address in the assembly instruction: \"{}\" was detected! Trying to resolve it.",
                                asm_inst.content
                            );

                            let target_address = asm_inst
                                .extract_target_address_from_gem5_trace(
                                    trace_path,
                                    gem5_trace_indirect_branch_regex,
                                )
                                .unwrap();
                            let target_id = AssemblyInstruction::get_asm_inst_id_by_address(
                                subroutines,
                                &target_address,
                            )
                            .unwrap();

                            target_id
                        } else if inst_name == "ret" {
                            if let Some(return_id) = return_stack.pop() {
                                return_id
                            } else {
                                if i == main_sub_index && j == subroutines[i].asm_insts.len() - 1 {
                                    break 'main_loop;
                                } else {
                                    panic!(
                                        "Return stack is empty! We might have reached the end of the program! Assembly instruction: \"{}\"",
                                        asm_inst.content
                                    );
                                }
                            }
                        } else {
                            let target_adr = asm_inst.content.split_whitespace().nth(3).unwrap();
                            let target_id = AssemblyInstruction::get_asm_inst_id_by_address(
                                subroutines,
                                target_adr,
                            )
                            .unwrap();

                            target_id
                        }
                    }
                    AssemblyInstructionType::Ncti(_inst_name) => {
                        j += 1;
                        continue;
                    }
                };

                let asm_inst = &mut subroutines[i].asm_insts[j];
                match &mut asm_inst.inst_type {
                    AssemblyInstructionType::CtiConditional(_inst_name, ta) => {
                        *ta = Some(target_id)
                    }
                    AssemblyInstructionType::CtiUnconditional(inst_name, ta) => {
                        if inst_name == "bl" || inst_name == "blr" || inst_name == "ret" {
                            i = target_id.subroutine_index;
                            j = target_id.asm_inst_index;

                            *ta = Some(target_id);

                            continue 'main_loop;
                        } else {
                            *ta = Some(target_id);
                        }
                    }
                    AssemblyInstructionType::Ncti(_inst_name) => {
                        j += 1;
                        continue;
                    }
                }

                j += 1;
            }

            i += 1;
        }

        break;
    }
}
