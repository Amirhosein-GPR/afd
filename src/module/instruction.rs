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
    BDotAl,
    BDotNv,

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
    BCDotAl,
    BCDotNv,

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
            ConditionalBranch::BDotAl => "b.al",
            ConditionalBranch::BDotNv => "b.nv",
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
            ConditionalBranch::BCDotAl => "bc.al",
            ConditionalBranch::BCDotNv => "bc.nv",
            ConditionalBranch::Cbz => "cbz",
            ConditionalBranch::Cbnz => "cbnz",
            ConditionalBranch::Tbz => "tbz",
            ConditionalBranch::Tbnz => "tbnz",
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
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum InstructionType {
    Cti(CtiData),
    Ncti,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct CtiData {
    pub target_address: String,
    pub branch_type: BranchType,
    pub reachable: bool,
}

/// Represents a control transfaer instruction (aka, jump instruction) in an assembly file.
///
/// It contains some useful information about each CTI.
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct AssemblyInstruction {
    raw_content: String,
    pub line_number: u32,
    pub address: String,
    pub instruction_type: InstructionType,
}

impl AssemblyInstruction {
    pub fn formatted_string(&self) -> String {
        match &self.instruction_type {
            InstructionType::Cti(cti_data) => {
                format!(
                    "Line Number: {}, Address: {}, Type: [CTI] ==> Target Address: {:?}, Branch Type: {:?}, Reachable: {:?}",
                    self.line_number,
                    self.address,
                    cti_data.target_address,
                    cti_data.branch_type,
                    cti_data.reachable
                )
            }
            InstructionType::Ncti => {
                format!(
                    "Line Number: {}, Address: {}, Type: [NCTI]",
                    self.line_number, self.address,
                )
            }
        }
    }

    pub fn raw_content(&self) -> &str {
        &self.raw_content
    }

    pub fn address(&self) -> String {
        self.address.clone()
    }

    pub fn target_address(&self) -> Option<String> {
        match &self.instruction_type {
            InstructionType::Cti(cti_data) => Some(cti_data.target_address.clone()),
            InstructionType::Ncti => None,
        }
    }
}

pub enum BranchTargetAddress {
    Direct(String),
    Indirect(String),
}

/// Used to detect 2 main types of instructions (branch and non-branch) in an assembly file.
pub fn determine_instruction_type(
    assembly_line: &str,
    line_number: u32,
    regex_container: &RegexContainer,
) -> AssemblyInstruction {
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
            f if f == ConditionalBranch::BDotAl.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BDotAl))
            }
            f if f == ConditionalBranch::BDotNv.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BDotNv))
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
            f if f == ConditionalBranch::BCDotAl.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotAl))
            }
            f if f == ConditionalBranch::BCDotNv.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotNv))
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
            _ => None,
        }
    } else {
        None
    };

    if let Some(branch_type) = branch_type {
        AssemblyInstruction {
            raw_content: assembly_line.to_string(),
            line_number,
            address: extract_address(assembly_line),
            instruction_type: InstructionType::Cti(CtiData {
                target_address: extract_target_address(
                    assembly_line,
                    &regex_container.target_address,
                ),
                branch_type,
                reachable: true,
            }),
        }
    } else {
        AssemblyInstruction {
            raw_content: assembly_line.to_string(),
            line_number,
            address: extract_address(assembly_line),
            instruction_type: InstructionType::Ncti,
        }
    }
}

fn extract_address(line: &str) -> String {
    line.split(":\t").next().unwrap().to_string()
}

fn extract_target_address(line: &str, target_address_regex: &Regex) -> String {
    // REMOVE LATER
    // println!("L: {line}");
    let target_address = target_address_regex
        .captures(line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string();
    // REMOVE LATER
    // println!("Captured: {target_address}");

    target_address
}
