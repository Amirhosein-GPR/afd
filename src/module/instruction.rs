use regex::Regex;

use crate::module::regex::RegexContainer;

/// Represents the two main types of branch instrutions in ARM architecture.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BranchType {
    ConditionalBranch(ConditionalBranch),
    UnconditionalBranch(UnconditionalBranch),
}

impl BranchType {
    /// Returns the corresponding string values for each type of branch instructions.
    pub fn value(&self) -> &str {
        match self {
            BranchType::ConditionalBranch(conditional_branch) => conditional_branch.value(),
            BranchType::UnconditionalBranch(unconditional_branch) => unconditional_branch.value(),
        }
    }

    pub fn branch_condition(&self) -> String {
        match self {
            BranchType::ConditionalBranch(conditional_branch) => {
                conditional_branch.branch_condition().to_string()
            }
            BranchType::UnconditionalBranch(unconditional_branch) => {
                unconditional_branch.branch_condition().to_string()
            }
        }
    }

    pub fn branch_condition_complement(&self) -> String {
        match self {
            BranchType::ConditionalBranch(conditional_branch) => {
                conditional_branch.branch_condition_complement().to_string()
            }
            BranchType::UnconditionalBranch(unconditional_branch) => unconditional_branch
                .branch_condition_complement()
                .to_string(),
        }
    }
}

/// Represents conditional branch instrutions in ARM architecture.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ConditionalBranch {
    BDotEq,
    BDotNe,
    BDotCs,
    BDotCc,
    BDotMi,
    BDotPl,
    BDotVs,
    BDotVc,
    BDotHi,
    BDotLs,
    BDotGe,
    BDotLt,
    BDotGt,
    BDotLe,

    BCDotEq,
    BCDotNe,
    BCDotCs,
    BCDotCc,
    BCDotMi,
    BCDotPl,
    BCDotVs,
    BCDotVc,
    BCDotHi,
    BCDotLs,
    BCDotGe,
    BCDotLt,
    BCDotGt,
    BCDotLe,

    Cbz,
    Cbnz,
    Tbz,
    Tbnz,
}

impl ConditionalBranch {
    /// Returns the corresponding string values for each type of conditional branch instructions.
    pub fn value(&self) -> &str {
        match self {
            ConditionalBranch::BDotEq => "b.eq",
            ConditionalBranch::BDotNe => "b.ne",
            ConditionalBranch::BDotCs => "b.cs",
            ConditionalBranch::BDotCc => "b.cc",
            ConditionalBranch::BDotMi => "b.mi",
            ConditionalBranch::BDotPl => "b.pl",
            ConditionalBranch::BDotVs => "b.vs",
            ConditionalBranch::BDotVc => "b.vc",
            ConditionalBranch::BDotHi => "b.hi",
            ConditionalBranch::BDotLs => "b.ls",
            ConditionalBranch::BDotGe => "b.ge",
            ConditionalBranch::BDotLt => "b.lt",
            ConditionalBranch::BDotGt => "b.gt",
            ConditionalBranch::BDotLe => "b.le",
            ConditionalBranch::BCDotEq => "bc.eq",
            ConditionalBranch::BCDotNe => "bc.ne",
            ConditionalBranch::BCDotCs => "bc.cs",
            ConditionalBranch::BCDotCc => "bc.cc",
            ConditionalBranch::BCDotMi => "bc.mi",
            ConditionalBranch::BCDotPl => "bc.pl",
            ConditionalBranch::BCDotVs => "bc.vs",
            ConditionalBranch::BCDotVc => "bc.vc",
            ConditionalBranch::BCDotHi => "bc.hi",
            ConditionalBranch::BCDotLs => "bc.ls",
            ConditionalBranch::BCDotGe => "bc.ge",
            ConditionalBranch::BCDotLt => "bc.lt",
            ConditionalBranch::BCDotGt => "bc.gt",
            ConditionalBranch::BCDotLe => "bc.le",
            ConditionalBranch::Cbz => "cbz",
            ConditionalBranch::Cbnz => "cbnz",
            ConditionalBranch::Tbz => "tbz",
            ConditionalBranch::Tbnz => "tbnz",
        }
    }

    pub fn branch_condition(&self) -> &str {
        match self {
            ConditionalBranch::BDotEq => "Z == 1",
            ConditionalBranch::BDotNe => "Z == 0",
            ConditionalBranch::BDotCs => "C == 1",
            ConditionalBranch::BDotCc => "C == 0",
            ConditionalBranch::BDotMi => "N == 1",
            ConditionalBranch::BDotPl => "N == 0",
            ConditionalBranch::BDotVs => "V == 1",
            ConditionalBranch::BDotVc => "V == 0",
            ConditionalBranch::BDotHi => "C == 1 and Z == 0",
            ConditionalBranch::BDotLs => "C == 0 or Z == 1",
            ConditionalBranch::BDotGe => "N == V",
            ConditionalBranch::BDotLt => "N != V",
            ConditionalBranch::BDotGt => "Z == 0 and N == V",
            ConditionalBranch::BDotLe => "Z == 1 or N != V",
            ConditionalBranch::BCDotEq => "Z == 1",
            ConditionalBranch::BCDotNe => "Z == 0",
            ConditionalBranch::BCDotCs => "C == 1",
            ConditionalBranch::BCDotCc => "C == 0",
            ConditionalBranch::BCDotMi => "N == 1",
            ConditionalBranch::BCDotPl => "N == 0",
            ConditionalBranch::BCDotVs => "V == 1",
            ConditionalBranch::BCDotVc => "V == 0",
            ConditionalBranch::BCDotHi => "C == 1 and Z == 0",
            ConditionalBranch::BCDotLs => "C == 0 or Z == 1",
            ConditionalBranch::BCDotGe => "N == V",
            ConditionalBranch::BCDotLt => "N != V",
            ConditionalBranch::BCDotGt => "Z == 0 and N == V",
            ConditionalBranch::BCDotLe => "Z == 1 or N != V",
            ConditionalBranch::Cbz => "register == 0",
            ConditionalBranch::Cbnz => "register != 0",
            ConditionalBranch::Tbz => "selected bit == 0",
            ConditionalBranch::Tbnz => "selected bit == 1",
        }
    }

    pub fn branch_condition_complement(&self) -> &str {
        match self {
            ConditionalBranch::BDotEq => "Z != 1",
            ConditionalBranch::BDotNe => "Z != 0",
            ConditionalBranch::BDotCs => "C != 1",
            ConditionalBranch::BDotCc => "C != 0",
            ConditionalBranch::BDotMi => "N != 1",
            ConditionalBranch::BDotPl => "N != 0",
            ConditionalBranch::BDotVs => "V != 1",
            ConditionalBranch::BDotVc => "V != 0",
            ConditionalBranch::BDotHi => "C != 1 or Z != 0",
            ConditionalBranch::BDotLs => "C != 0 and Z != 1",
            ConditionalBranch::BDotGe => "N != V",
            ConditionalBranch::BDotLt => "N == V",
            ConditionalBranch::BDotGt => "Z != 0 or N != V",
            ConditionalBranch::BDotLe => "Z != 1 and N == V",
            ConditionalBranch::BCDotEq => "Z != 1",
            ConditionalBranch::BCDotNe => "Z != 0",
            ConditionalBranch::BCDotCs => "C != 1",
            ConditionalBranch::BCDotCc => "C != 0",
            ConditionalBranch::BCDotMi => "N != 1",
            ConditionalBranch::BCDotPl => "N != 0",
            ConditionalBranch::BCDotVs => "V != 1",
            ConditionalBranch::BCDotVc => "V != 0",
            ConditionalBranch::BCDotHi => "C != 1 or Z != 0",
            ConditionalBranch::BCDotLs => "C != 0 and Z != 1",
            ConditionalBranch::BCDotGe => "N != V",
            ConditionalBranch::BCDotLt => "N == V",
            ConditionalBranch::BCDotGt => "Z != 0 or N != V",
            ConditionalBranch::BCDotLe => "Z != 1 and N == V",
            ConditionalBranch::Cbz => "register != 0",
            ConditionalBranch::Cbnz => "register == 0",
            ConditionalBranch::Tbz => "selected bit != 0",
            ConditionalBranch::Tbnz => "selected bit != 1",
        }
    }
}

/// Represents unconditional branch instrutions in ARM architecture.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum UnconditionalBranch {
    B,
    Bl,
    Ret,
    Br,
    Blr,
    Blraa,
    Blraaz,
    Blrab,
    Blrabz,
    BDotAl,
    BCDotAl,
    BDotNv,
    BCDotNv,
}

impl UnconditionalBranch {
    /// Returns the corresponding string values for each type of unconditional branch instructions.
    pub fn value(&self) -> &str {
        match self {
            UnconditionalBranch::B => "b",
            UnconditionalBranch::Bl => "bl",
            UnconditionalBranch::Ret => "ret",
            UnconditionalBranch::Br => "br",
            UnconditionalBranch::Blr => "blr",
            UnconditionalBranch::Blraa => "blraa",
            UnconditionalBranch::Blraaz => "blraaz",
            UnconditionalBranch::Blrab => "blrab",
            UnconditionalBranch::Blrabz => "blrabz",
            UnconditionalBranch::BDotAl => "b.al",
            UnconditionalBranch::BCDotAl => "bc.al",
            UnconditionalBranch::BDotNv => "b.nv",
            UnconditionalBranch::BCDotNv => "bc.nv",
        }
    }

    pub fn branch_condition(&self) -> &str {
        "true"
    }

    pub fn branch_condition_complement(&self) -> &str {
        "false"
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum InstructionType {
    Cti(CtiData),
    Ncti,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct CtiData {
    pub target_address: TargetAddress,
    pub branch_type: BranchType,
    pub reachable: bool,
}

/// Represents a control transfaer instruction (aka, jump instruction) in an assembly file.
///
/// It contains some useful information about each CTI.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct AssemblyInstruction {
    pub raw_content: String,
    pub line_number: u32,
    pub address: String,
    pub instruction_type: InstructionType,
}

impl AssemblyInstruction {
    pub fn new(assembly_line: &str, line_number: u32, regex_container: &RegexContainer) -> Self {
        let branch_type = if let Some(found) = regex_container.branches.captures(assembly_line) {
            match found.get(1).unwrap().as_str() {
                f if f == ConditionalBranch::BDotEq.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BDotEq))
                }
                f if f == ConditionalBranch::BDotNe.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BDotNe))
                }
                f if f == ConditionalBranch::BDotCs.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BDotCs))
                }
                f if f == ConditionalBranch::BDotCc.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BDotCc))
                }
                f if f == ConditionalBranch::BDotMi.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BDotMi))
                }
                f if f == ConditionalBranch::BDotPl.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BDotPl))
                }
                f if f == ConditionalBranch::BDotVs.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BDotVs))
                }
                f if f == ConditionalBranch::BDotVc.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BDotVc))
                }
                f if f == ConditionalBranch::BDotHi.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BDotHi))
                }
                f if f == ConditionalBranch::BDotLs.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BDotLs))
                }
                f if f == ConditionalBranch::BDotGe.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BDotGe))
                }
                f if f == ConditionalBranch::BDotLt.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BDotLt))
                }
                f if f == ConditionalBranch::BDotGt.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BDotGt))
                }
                f if f == ConditionalBranch::BDotLe.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BDotLe))
                }
                f if f == ConditionalBranch::BCDotEq.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotEq))
                }
                f if f == ConditionalBranch::BCDotNe.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotNe))
                }
                f if f == ConditionalBranch::BCDotCs.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotCs))
                }
                f if f == ConditionalBranch::BCDotCc.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotCc))
                }
                f if f == ConditionalBranch::BCDotMi.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotMi))
                }
                f if f == ConditionalBranch::BCDotPl.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotPl))
                }
                f if f == ConditionalBranch::BCDotVs.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotVs))
                }
                f if f == ConditionalBranch::BCDotVc.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotVc))
                }
                f if f == ConditionalBranch::BCDotHi.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotHi))
                }
                f if f == ConditionalBranch::BCDotLs.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotLs))
                }
                f if f == ConditionalBranch::BCDotGe.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotGe))
                }
                f if f == ConditionalBranch::BCDotLt.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotLt))
                }
                f if f == ConditionalBranch::BCDotGt.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotGt))
                }
                f if f == ConditionalBranch::BCDotLe.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotLe))
                }
                f if f == ConditionalBranch::Cbz.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::Cbz))
                }
                f if f == ConditionalBranch::Cbnz.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::Cbnz))
                }
                f if f == ConditionalBranch::Tbz.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::Tbz))
                }
                f if f == ConditionalBranch::Tbnz.value() => {
                    Some(BranchType::ConditionalBranch(ConditionalBranch::Tbnz))
                }
                f if f == UnconditionalBranch::B.value() => {
                    Some(BranchType::UnconditionalBranch(UnconditionalBranch::B))
                }
                f if f == UnconditionalBranch::Bl.value() => {
                    Some(BranchType::UnconditionalBranch(UnconditionalBranch::Bl))
                }
                f if f == UnconditionalBranch::Ret.value() => {
                    Some(BranchType::UnconditionalBranch(UnconditionalBranch::Ret))
                }
                f if f == UnconditionalBranch::Br.value() => {
                    Some(BranchType::UnconditionalBranch(UnconditionalBranch::Br))
                }
                f if f == UnconditionalBranch::Blr.value() => {
                    Some(BranchType::UnconditionalBranch(UnconditionalBranch::Blr))
                }
                f if f == UnconditionalBranch::Blraa.value() => {
                    Some(BranchType::UnconditionalBranch(UnconditionalBranch::Blraa))
                }
                f if f == UnconditionalBranch::Blraaz.value() => {
                    Some(BranchType::UnconditionalBranch(UnconditionalBranch::Blraaz))
                }
                f if f == UnconditionalBranch::Blrab.value() => {
                    Some(BranchType::UnconditionalBranch(UnconditionalBranch::Blrab))
                }
                f if f == UnconditionalBranch::Blrabz.value() => {
                    Some(BranchType::UnconditionalBranch(UnconditionalBranch::Blrabz))
                }
                f if f == UnconditionalBranch::BDotAl.value() => {
                    Some(BranchType::UnconditionalBranch(UnconditionalBranch::BDotAl))
                }
                f if f == UnconditionalBranch::BCDotAl.value() => Some(
                    BranchType::UnconditionalBranch(UnconditionalBranch::BCDotAl),
                ),
                f if f == UnconditionalBranch::BDotNv.value() => {
                    Some(BranchType::UnconditionalBranch(UnconditionalBranch::BDotNv))
                }
                f if f == UnconditionalBranch::BCDotNv.value() => Some(
                    BranchType::UnconditionalBranch(UnconditionalBranch::BCDotNv),
                ),
                _ => None,
            }
        } else {
            None
        };

        if let Some(branch_type) = branch_type {
            Self {
                raw_content: assembly_line.to_string(),
                line_number,
                address: extract_address(assembly_line),
                instruction_type: InstructionType::Cti(CtiData {
                    target_address: extract_target_address(
                        assembly_line,
                        &regex_container.target_address,
                        &branch_type,
                    ),
                    branch_type,
                    reachable: true,
                }),
            }
        } else {
            Self {
                raw_content: assembly_line.to_string(),
                line_number,
                address: extract_address(assembly_line),
                instruction_type: InstructionType::Ncti,
            }
        }
    }

    pub fn formatted_string(&self) -> String {
        match &self.instruction_type {
            InstructionType::Cti(cti_data) => {
                format!(
                    "Line Number: {}, Address: {}, Type: [CTI] ==> Target Address: {:?}, Branch Type: {:?}",
                    self.line_number, self.address, cti_data.target_address, cti_data.branch_type,
                )
            }
            InstructionType::Ncti => {
                format!(
                    "Line Number: {}, Address: {}, Type: [NCTI]",
                    self.line_number, self.address
                )
            }
        }
    }

    pub fn cti_condtions(&self) -> (&str, &str, &str) {
        match &self.instruction_type {
            InstructionType::Cti(cti_data) => (
                cti_data.branch_type.value(),
                cti_data.target_address.value(),
                self.address.as_str(),
            ),
            InstructionType::Ncti => panic!("Error: cti_conditions() got called on NCTI type!"),
        }
    }

    pub fn raw_content(&self) -> &str {
        &self.raw_content
    }

    pub fn address(&self) -> String {
        self.address.clone()
    }

    pub fn get_target_address(&self) -> Option<TargetAddress> {
        match &self.instruction_type {
            InstructionType::Cti(cti_data) => Some(cti_data.target_address.clone()),
            InstructionType::Ncti => None,
        }
    }

    pub fn set_direct_target_address(&mut self, target_address: String) {
        match &mut self.instruction_type {
            InstructionType::Cti(cti_data) => {
                cti_data.target_address = TargetAddress::Direct(target_address);
            }
            InstructionType::Ncti => {
                panic!("Error: NCTIs don't have target address.")
            }
        }
    }

    pub fn flag_as_unreachable(&mut self) {
        match &mut self.instruction_type {
            InstructionType::Cti(cti_data) => {
                cti_data.reachable = false;
            }
            InstructionType::Ncti => {
                panic!("Error: NCTIs don't have target address.")
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum TargetAddress {
    Direct(String),
    Indirect(String),
}

impl TargetAddress {
    pub fn value(&self) -> &str {
        match self {
            TargetAddress::Direct(string) => string.as_str(),
            TargetAddress::Indirect(string) => string.as_str(),
        }
    }
}

fn extract_address(line: &str) -> String {
    line.split(":\t").next().unwrap().to_string()
}

fn extract_target_address(
    line: &str,
    target_address_regex: &Regex,
    branch_type: &BranchType,
) -> TargetAddress {
    let target_address_string = target_address_regex
        .captures(line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string();

    match branch_type {
        BranchType::ConditionalBranch(cb) => match cb {
            _ => TargetAddress::Direct(target_address_string),
        },
        BranchType::UnconditionalBranch(ub) => match ub {
            UnconditionalBranch::Ret | UnconditionalBranch::Br | UnconditionalBranch::Blr => {
                TargetAddress::Indirect(target_address_string)
            }
            _ => TargetAddress::Direct(target_address_string),
        },
    }
}
