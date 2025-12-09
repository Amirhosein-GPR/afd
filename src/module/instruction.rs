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
    BDotEQ,
    BDotNE,
    BDotCS,
    BDotCC,
    BDotMI,
    BDotPL,
    BDotVS,
    BDotVC,
    BDotHI,
    BDotLS,
    BDotGE,
    BDotLT,
    BDotGT,
    BDotLE,
    BDotAL,
    BDotNV,

    BCDotEQ,
    BCDotNE,
    BCDotCS,
    BCDotCC,
    BCDotMI,
    BCDotPL,
    BCDotVS,
    BCDotVC,
    BCDotHI,
    BCDotLS,
    BCDotGE,
    BCDotLT,
    BCDotGT,
    BCDotLE,
    BCDotAL,
    BCDotNV,

    CBZ,
    CBNZ,
    TBZ,
    TBNZ,
}

impl ConditionalBranch {
    /// Returns the corresponding string values for each type of conditional branch instructions.
    pub fn value(&self) -> &str {
        match self {
            ConditionalBranch::BDotEQ => "b.eq",
            ConditionalBranch::BDotNE => "b.ne",
            ConditionalBranch::BDotCS => "b.cs",
            ConditionalBranch::BDotCC => "b.cc",
            ConditionalBranch::BDotMI => "b.mi",
            ConditionalBranch::BDotPL => "b.pl",
            ConditionalBranch::BDotVS => "b.vs",
            ConditionalBranch::BDotVC => "b.vc",
            ConditionalBranch::BDotHI => "b.hi",
            ConditionalBranch::BDotLS => "b.ls",
            ConditionalBranch::BDotGE => "b.ge",
            ConditionalBranch::BDotLT => "b.lt",
            ConditionalBranch::BDotGT => "b.gt",
            ConditionalBranch::BDotLE => "b.le",
            ConditionalBranch::BDotAL => "b.al",
            ConditionalBranch::BDotNV => "b.nv",
            ConditionalBranch::BCDotEQ => "bc.eq",
            ConditionalBranch::BCDotNE => "bc.ne",
            ConditionalBranch::BCDotCS => "bc.cs",
            ConditionalBranch::BCDotCC => "bc.cc",
            ConditionalBranch::BCDotMI => "bc.mi",
            ConditionalBranch::BCDotPL => "bc.pl",
            ConditionalBranch::BCDotVS => "bc.vs",
            ConditionalBranch::BCDotVC => "bc.vc",
            ConditionalBranch::BCDotHI => "bc.hi",
            ConditionalBranch::BCDotLS => "bc.ls",
            ConditionalBranch::BCDotGE => "bc.ge",
            ConditionalBranch::BCDotLT => "bc.lt",
            ConditionalBranch::BCDotGT => "bc.gt",
            ConditionalBranch::BCDotLE => "bc.le",
            ConditionalBranch::BCDotAL => "bc.al",
            ConditionalBranch::BCDotNV => "bc.nv",
            ConditionalBranch::CBZ => "cbz",
            ConditionalBranch::CBNZ => "cbnz",
            ConditionalBranch::TBZ => "tbz",
            ConditionalBranch::TBNZ => "tbnz",
        }
    }
}

/// Represents unconditional branch instrutions in ARM architecture.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum UnconditionalBranch {
    DirectBranch(DirectBranch),
    IndirectBranch(IndirectBranch),
}

impl UnconditionalBranch {
    /// Returns the corresponding string values for each type of unconditional branch instructions.
    pub fn value(&self) -> &str {
        match self {
            UnconditionalBranch::DirectBranch(direct_branch) => direct_branch.value(),
            UnconditionalBranch::IndirectBranch(indirect_branch) => indirect_branch.value(),
        }
    }
}

/// Represents direct unconditional branch instrutions in ARM architecture.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum DirectBranch {
    B,
    BL,
    RET,
}

impl DirectBranch {
    /// Returns the corresponding string values for each type of direct unconditional branch instructions.
    pub fn value(&self) -> &str {
        match self {
            DirectBranch::B => "b",
            DirectBranch::BL => "bl",
            DirectBranch::RET => "ret",
        }
    }
}

/// Represents indirect unconditional branch instrutions in ARM architecture.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum IndirectBranch {
    BR,
    BLR,
    BLRAA,
    BLRAAZ,
    BLRAB,
    BLRABZ,
}

impl IndirectBranch {
    /// Returns the corresponding string values for each type of indirect unconditional branch instructions.
    pub fn value(&self) -> &str {
        match self {
            IndirectBranch::BR => "br",
            IndirectBranch::BLR => "blr",
            IndirectBranch::BLRAA => "blraa",
            IndirectBranch::BLRAAZ => "blraaz",
            IndirectBranch::BLRAB => "blrab",
            IndirectBranch::BLRABZ => "blrabz",
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum AssemblyInstruction {
    CTI(ControlTransferInstruction),
    NCTI(NonControlTransferInstruction),
}

impl AssemblyInstruction {
    pub fn formatted_string(&self) -> String {
        match self {
            AssemblyInstruction::CTI(control_transfer_instruction) => {
                control_transfer_instruction.formatted_string()
            }
            AssemblyInstruction::NCTI(non_control_transfer_instruction) => {
                non_control_transfer_instruction.formatted_string()
            }
        }
    }

    pub fn raw_content(&self) -> &str {
        match self {
            AssemblyInstruction::CTI(control_transfer_instruction) => {
                control_transfer_instruction.raw_content()
            }
            AssemblyInstruction::NCTI(non_control_transfer_instruction) => {
                non_control_transfer_instruction.raw_content()
            }
        }
    }

    pub fn address(&self) -> String {
        match self {
            AssemblyInstruction::CTI(cti) => cti.address.clone(),
            AssemblyInstruction::NCTI(ncti) => ncti.address.clone(),
        }
    }

    pub fn target_address(&self) -> Option<String> {
        match self {
            AssemblyInstruction::CTI(cti) => Some(cti.target_address.clone()),
            AssemblyInstruction::NCTI(_ncti) => None,
        }
    }
}

/// Represents a control transfaer instruction (aka, jump instruction) in an assembly file.
///
/// It contains some useful information about each CTI.
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ControlTransferInstruction {
    raw_content: String,
    pub line_number: u32,
    pub address: String,
    pub target_address: String,
    pub branch_type: BranchType,
}

impl ControlTransferInstruction {
    pub fn formatted_string(&self) -> String {
        format!(
            "Line Number: {}, Address: {}, Type: [CTI] ==> Target Address: {:?}, Branch Type: {:?}",
            self.line_number, self.address, self.target_address, self.branch_type
        )
    }

    pub fn raw_content(&self) -> &str {
        &self.raw_content
    }
}

/// Represents any intsructions other than control transfer ones.
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct NonControlTransferInstruction {
    raw_content: String,
    pub line_number: u32,
    pub address: String,
}

impl NonControlTransferInstruction {
    pub fn formatted_string(&self) -> String {
        format!(
            "Line Number: {}, Address: {}, Type: [NCTI]",
            self.line_number, self.address
        )
    }
    pub fn raw_content(&self) -> &str {
        &self.raw_content
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
            f if f == ConditionalBranch::BDotEQ.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BDotEQ))
            }
            f if f == ConditionalBranch::BDotNE.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BDotNE))
            }
            f if f == ConditionalBranch::BDotCS.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BDotCS))
            }
            f if f == ConditionalBranch::BDotCC.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BDotCC))
            }
            f if f == ConditionalBranch::BDotMI.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BDotMI))
            }
            f if f == ConditionalBranch::BDotPL.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BDotPL))
            }
            f if f == ConditionalBranch::BDotVS.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BDotVS))
            }
            f if f == ConditionalBranch::BDotVC.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BDotVC))
            }
            f if f == ConditionalBranch::BDotHI.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BDotHI))
            }
            f if f == ConditionalBranch::BDotLS.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BDotLS))
            }
            f if f == ConditionalBranch::BDotGE.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BDotGE))
            }
            f if f == ConditionalBranch::BDotLT.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BDotLT))
            }
            f if f == ConditionalBranch::BDotGT.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BDotGT))
            }
            f if f == ConditionalBranch::BDotLE.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BDotLE))
            }
            f if f == ConditionalBranch::BDotAL.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BDotAL))
            }
            f if f == ConditionalBranch::BDotNV.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BDotNV))
            }
            f if f == ConditionalBranch::BCDotEQ.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotEQ))
            }
            f if f == ConditionalBranch::BCDotNE.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotNE))
            }
            f if f == ConditionalBranch::BCDotCS.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotCS))
            }
            f if f == ConditionalBranch::BCDotCC.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotCC))
            }
            f if f == ConditionalBranch::BCDotMI.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotMI))
            }
            f if f == ConditionalBranch::BCDotPL.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotPL))
            }
            f if f == ConditionalBranch::BCDotVS.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotVS))
            }
            f if f == ConditionalBranch::BCDotVC.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotVC))
            }
            f if f == ConditionalBranch::BCDotHI.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotHI))
            }
            f if f == ConditionalBranch::BCDotLS.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotLS))
            }
            f if f == ConditionalBranch::BCDotGE.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotGE))
            }
            f if f == ConditionalBranch::BCDotLT.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotLT))
            }
            f if f == ConditionalBranch::BCDotGT.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotGT))
            }
            f if f == ConditionalBranch::BCDotLE.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotLE))
            }
            f if f == ConditionalBranch::BCDotAL.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotAL))
            }
            f if f == ConditionalBranch::BCDotNV.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::BCDotNV))
            }
            f if f == ConditionalBranch::CBZ.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::CBZ))
            }
            f if f == ConditionalBranch::CBNZ.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::CBNZ))
            }
            f if f == ConditionalBranch::TBZ.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::TBZ))
            }
            f if f == ConditionalBranch::TBNZ.value() => {
                Some(BranchType::ConditionalBranch(ConditionalBranch::TBNZ))
            }
            f if f == DirectBranch::B.value() => Some(BranchType::UnconditionalBranch(
                UnconditionalBranch::DirectBranch(DirectBranch::B),
            )),
            f if f == DirectBranch::BL.value() => Some(BranchType::UnconditionalBranch(
                UnconditionalBranch::DirectBranch(DirectBranch::BL),
            )),
            f if f == DirectBranch::RET.value() => Some(BranchType::UnconditionalBranch(
                UnconditionalBranch::DirectBranch(DirectBranch::RET),
            )),
            f if f == IndirectBranch::BR.value() => Some(BranchType::UnconditionalBranch(
                UnconditionalBranch::IndirectBranch(IndirectBranch::BR),
            )),
            f if f == IndirectBranch::BLR.value() => Some(BranchType::UnconditionalBranch(
                UnconditionalBranch::IndirectBranch(IndirectBranch::BLR),
            )),
            f if f == IndirectBranch::BLRAA.value() => Some(BranchType::UnconditionalBranch(
                UnconditionalBranch::IndirectBranch(IndirectBranch::BLRAA),
            )),
            f if f == IndirectBranch::BLRAAZ.value() => Some(BranchType::UnconditionalBranch(
                UnconditionalBranch::IndirectBranch(IndirectBranch::BLRAAZ),
            )),
            f if f == IndirectBranch::BLRAB.value() => Some(BranchType::UnconditionalBranch(
                UnconditionalBranch::IndirectBranch(IndirectBranch::BLRAB),
            )),
            f if f == IndirectBranch::BLRABZ.value() => Some(BranchType::UnconditionalBranch(
                UnconditionalBranch::IndirectBranch(IndirectBranch::BLRABZ),
            )),
            _ => None,
        }
    } else {
        None
    };

    if let Some(branch_type) = branch_type {
        AssemblyInstruction::CTI(ControlTransferInstruction {
            raw_content: assembly_line.to_string(),
            line_number,
            address: extract_address(assembly_line),
            target_address: extract_target_address(assembly_line, &regex_container.target_address),
            branch_type,
        })
    } else {
        AssemblyInstruction::NCTI(NonControlTransferInstruction {
            raw_content: assembly_line.to_string(),
            line_number,
            address: extract_address(assembly_line),
        })
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
