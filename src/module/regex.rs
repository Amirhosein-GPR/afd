use regex::Regex;

use crate::module::config::BINARY_EXT;

pub struct RegexContainer {
    pub src_file_regex: Regex,
    pub bin_file_regex: Regex,
    pub branches: Regex,
    pub definition_label: Regex,
    pub target_label: Regex,
    pub target_address: Regex,
    pub gem5_trace_indirect_branch: Regex,
    pub jump_to_subroutine: Regex,
}

impl RegexContainer {
    pub fn new() -> Self {
        Self {
            src_file_regex: Regex::new(r"(benchmarks(?:\w|\/)*)sources((?:\w|\/)*)\.(?:c|cc)$").unwrap(),
            bin_file_regex: Regex::new(
                    format!(
                        r"(benchmarks(?:\w|\/)*)binaries((?:\w|\/)*){}$",
                        regex::escape(BINARY_EXT)
                    )
                    .as_str(),
                )
                .unwrap(),
            branches: Regex::new(r"\b\s+((?:b\.\w+)|(?:bc\.\w+)|(?:blrabz)|(?:blrab)|(?:blraaz)|(?:blraa)|(?:bl)|(?:blr)|(?:br)|(?:ret)|(?:b)|(?:cbnz)|(?:cbz)|(?:tbnz)|(?:tbz))\s+\b").unwrap(),
            definition_label: Regex::new(r"^[0-9a-f]+\s<[.\w]*>:$").unwrap(),
            target_label: Regex::new(r"<\w+>$").unwrap(),
            target_address: Regex::new(r"(\w+)(?:\s+<.+>)?$").unwrap(),
            gem5_trace_indirect_branch: Regex::new(r"Commit branch:.*PC:0x(\w+) .*Indirect.*target:0x(\w+)").unwrap(),
            jump_to_subroutine: Regex::new(r"^[0-9a-f]+:\s+[0-9a-f]+\s+(?:blr|bl)").unwrap()
        }
    }
}
