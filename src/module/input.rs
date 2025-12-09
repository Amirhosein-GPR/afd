use std::{env, fs, process::Command};

use clap::Parser;
use regex::{Captures, Regex};

use crate::module::{
    config::{
        ASSEMBLY_EXT_CLEANED, ASSEMBLY_EXT_ORGINAL, ASSEMBLY_EXT_UNCOMMENTED, BINARY_EXT, TRACE_EXT,
    },
    regex::RegexContainer,
};

/// Represents cli arguments of the program.
#[derive(Parser)]
#[command(
    name = "CFGC",
    version,
    about = "A Control Flow Graph (CFG) generator for AArch64 (Arm-64bit) programs"
)]
pub struct Cli {
    #[arg(short, long)]
    /// Path to the source files. If set, '--binary_path' option will be ignored.
    pub source_path: Option<String>,
    #[arg(short, long)]
    /// Path to the binary files. If no '--source_path' option is set this option will be used.
    pub binary_path: Option<String>,
}

pub enum FileType {
    Original,
    Uncommented,
    Cleaned,
}

pub struct InputManager {
    src_paths: Option<Vec<String>>,
    bin_paths: Vec<String>,
    asm_paths: Option<Vec<String>>,
    trace_paths: Option<Vec<String>>,
}

impl InputManager {
    pub fn new(src_file_regex: &Regex) -> Self {
        env::set_current_dir("../..").unwrap();

        if !fs::exists("workspace/gem5_branch_traces").unwrap() {
            fs::create_dir("workspace/gem5_branch_traces").unwrap();
        }

        if !fs::exists("workspace/cleaned_assemblies").unwrap() {
            fs::create_dir("workspace/cleaned_assemblies").unwrap();
        }

        let cli = Cli::parse();

        match cli.source_path.as_ref() {
            Some(source_path) => {
                let src_paths = Self::get_files_in_path(&source_path);
                let bin_paths = Self::create_bin_paths(&src_paths, src_file_regex);

                Self {
                    src_paths: Some(src_paths),
                    bin_paths,
                    asm_paths: None,
                    trace_paths: None,
                }
            }
            None => match cli.binary_path.as_ref() {
                Some(binary_path) => Self {
                    src_paths: None,
                    bin_paths: Self::get_files_in_path(&binary_path),
                    asm_paths: None,
                    trace_paths: None,
                },
                None => {
                    panic!(
                        "At least one of the options should be set! Run with --help for more information.",
                    )
                }
            },
        }
    }

    pub fn get_binary_paths(&self) -> &Vec<String> {
        &self.bin_paths
    }

    pub fn get_assembly_paths(
        &mut self,
        bin_file_regex: &Regex,
        file_type: FileType,
    ) -> Vec<String> {
        if self.asm_paths.is_none() {
            self.asm_paths = Some(Vec::new());

            for bin_path in &self.bin_paths {
                let asm_path = bin_file_regex.replace(&bin_path, |caps: &Captures<'_>| {
                    format!(
                        "{}{}{}{}",
                        &caps[1], "assemblies/original", &caps[2], ASSEMBLY_EXT_ORGINAL
                    )
                });
                self.asm_paths.as_mut().unwrap().push(asm_path.to_string());
            }
        }

        match file_type {
            FileType::Original => {
                Self::create_parent_dirs(self.asm_paths.as_ref().unwrap().first().unwrap());
                self.asm_paths.clone().unwrap()
            }
            FileType::Uncommented => {
                let mut ucmt_file_paths = Vec::new();
                for bin_path in &self.bin_paths {
                    let asm_path = bin_file_regex.replace(&bin_path, |caps: &Captures<'_>| {
                        format!(
                            "{}{}{}{}",
                            &caps[1], "assemblies/uncommented", &caps[2], ASSEMBLY_EXT_UNCOMMENTED
                        )
                    });
                    ucmt_file_paths.push(asm_path.to_string());
                }

                Self::create_parent_dirs(ucmt_file_paths.first().unwrap());
                ucmt_file_paths
            }
            FileType::Cleaned => {
                let mut cleaned_file_paths = Vec::new();
                for bin_path in &self.bin_paths {
                    let asm_path = bin_file_regex.replace(&bin_path, |caps: &Captures<'_>| {
                        format!(
                            "workspace/cleaned_assemblies{}{}",
                            &caps[2], ASSEMBLY_EXT_CLEANED
                        )
                    });
                    cleaned_file_paths.push(asm_path.to_string());
                }

                Self::create_parent_dirs(cleaned_file_paths.first().unwrap());
                cleaned_file_paths
            }
        }
    }

    pub fn gem5_trace_paths(&mut self, bin_file_regex: &Regex) -> &Vec<String> {
        if self.trace_paths.is_none() {
            self.trace_paths = Some(Vec::new());

            for bin_path in &self.bin_paths {
                let trace_path = bin_file_regex.replace(&bin_path, |caps: &Captures<'_>| {
                    format!(
                        "{}{}{}",
                        "workspace/gem5_branch_traces", &caps[2], TRACE_EXT
                    )
                });
                self.trace_paths
                    .as_mut()
                    .unwrap()
                    .push(trace_path.to_string());
            }
        }

        self.trace_paths.as_ref().unwrap()
    }

    fn get_files_in_path(input_path: &str) -> Vec<String> {
        // Creating the absolute path from the program's relative path.
        let input_path = format!(
            "{}/{}",
            env::current_dir().unwrap().to_str().unwrap(),
            input_path
        );

        let meta_data = fs::metadata(&input_path).unwrap();

        let input_paths = if meta_data.is_file() {
            vec![input_path]
        } else if meta_data.is_dir() {
            fs::read_dir(input_path)
                .unwrap()
                .map(|e| e.unwrap().path().to_str().unwrap().to_string())
                .collect::<Vec<_>>()
        } else {
            panic!("Error: Invalid input directory or file path!");
        };

        input_paths
    }

    fn create_bin_paths(src_paths: &[String], src_file_regex: &Regex) -> Vec<String> {
        let mut bin_paths = Vec::new();
        for src_path in src_paths {
            let bin_path = src_file_regex.replace(&src_path, |caps: &Captures<'_>| {
                format!("{}{}{}{}", &caps[1], "binaries", &caps[2], BINARY_EXT)
            });

            bin_paths.push(bin_path.to_string());
        }

        Self::create_parent_dirs(bin_paths.first().unwrap());

        bin_paths
    }

    fn create_parent_dirs(file_path: &str) {
        let parent_dir_path = file_path.rsplit_once('/').unwrap().0;
        if !fs::exists(parent_dir_path).unwrap() {
            fs::create_dir_all(parent_dir_path).unwrap();
        }
    }
}

/// Processes the input files of the program.
///
/// Compiles source files (and/or) disassembles binaries to extract assembly instructions of each input program, and then, simulates the running of each program's binary.
/// - Compiling is done with the help of 'aarch64-linux-gnu-gcc' cross compiler.
/// - Disassembling is done with the help of 'aarch64-linux-gnu-objdump' disassembler.
/// - Simulation is done with the help of 'gem5' simulator.
pub fn process_input_files(input_manager: &mut InputManager, regex_container: &RegexContainer) {
    if input_manager.src_paths.is_some() {
        gcc_compile_source_files(
            input_manager.src_paths.as_ref().unwrap(),
            input_manager.get_binary_paths(),
        );
    }

    let asm_path =
        input_manager.get_assembly_paths(&regex_container.bin_file_regex, FileType::Original);
    let uasm_path =
        input_manager.get_assembly_paths(&regex_container.bin_file_regex, FileType::Uncommented);

    objdump_binary_files(input_manager.get_binary_paths(), &asm_path);

    create_uncommented_files(&asm_path, &uasm_path);

    gem5_simulate(input_manager.get_binary_paths());
}

/// Runs a command and waits for it to finish it's execution.
fn run_command(cmd: &str, args: &[&str]) {
    Command::new(cmd)
        .args(args)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

/// Same as [`run_command`] but also set's a custom current dir before executing the command.
fn run_command_and_set_dir(cmd: &str, args: &[&str], dir: &str) {
    let current_dir = env::current_dir().unwrap().to_str().unwrap().to_string();

    Command::new(cmd)
        .args(args)
        .current_dir(format!("{}/{dir}", current_dir))
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

/// Same as [`run_command`] but also returns the output of the command execution.
fn run_command_and_get_output(cmd: &str, args: &[&str]) -> String {
    String::from_utf8(Command::new(cmd).args(args).output().unwrap().stdout).unwrap()
}

/// Compiles source file(s) in C language with the help of 'aarch64-linux-gnu-gcc' cross compiler.
fn gcc_compile_source_files(src_paths: &[String], bin_paths: &[String]) {
    println!("\n======== Compiling With 'aarch64-linux-gnu-gcc' Cross Compiler ========");
    println!("CMD ---> aarch64-linux-gnu-gcc [SOURCE_PATH] -static -o [BINARY_PATH]\n");

    for i in 0..src_paths.len() {
        println!("Compiling: '{}'", src_paths[i]);
        run_command(
            "aarch64-linux-gnu-gcc",
            &[&src_paths[i], "-static", "-o", &bin_paths[i]],
        );
        println!("Saved At: '{}'\n", bin_paths[i]);
    }

    println!("\n======================== Compilation Finished! ========================");
}

/// Disassembles aarch64 binay file(s) with the help of 'aarch64-linux-gnu-objdump' disassembler.
fn objdump_binary_files(bin_paths: &[String], asm_paths: &[String]) {
    println!("\n===== Disassembling With 'aarch64-linux-gnu-objdump' Disassembler =====");
    println!("CMD ---> aarch64-linux-gnu-objdump -d [BINARY_PATH] >> [DISASSEMBLED_PATH]\n");

    for i in 0..bin_paths.len() {
        println!("ObjDump disassembling: '{}'", bin_paths[i]);
        let output =
            run_command_and_get_output("aarch64-linux-gnu-objdump", &["-d", &bin_paths[i]]);
        fs::write(&asm_paths[i], output).unwrap();
        println!("Saved disassembled file at: '{}'\n", asm_paths[i]);
    }

    println!("\n======================= Disassembling Finished! =======================");
}

/// Creates the uncommented file equivalent from original assembly file.
fn create_uncommented_files(asm_paths: &[String], uasm_paths: &[String]) {
    for i in 0..asm_paths.len() {
        let asm_string = fs::read_to_string(&asm_paths[i]).unwrap();
        let mut uncommented_string = Vec::new();
        for asl in asm_string.lines() {
            let temp = asl.split("//").collect::<Vec<_>>();
            uncommented_string.push(temp[0].trim());
        }

        fs::write(&uasm_paths[i], uncommented_string.join("\n")).unwrap();
    }
}

/// Simulates the execution of the binary programs(s) with the help of `gem5` simulator.
fn gem5_simulate(binary_file_paths: &[String]) {
    println!("\n================== Simulating With 'gem5' Simulator ===================");
    println!(
        "CMD ---> ../programs/gem5/build/ALL/gem5.opt --debug-flags=Branch --debug-file=../gem5_branch_traces/[TRACE_FILE_NAME].trace ../programs/gem5_arm_script.py --binary [BINARY_FILE_PATH]\n"
    );

    let file_regex = Regex::new(r"(\w*)\.run$").unwrap();

    for bfp in binary_file_paths {
        let file_name = file_regex.captures(bfp).unwrap().get(1).unwrap().as_str();

        println!("gem5 simulating: '{bfp}'\n");
        run_command_and_set_dir(
            "../programs/gem5/build/ALL/gem5.opt",
            &[
                "--debug-flags=Branch",
                format!("--debug-file=../gem5_branch_traces/{file_name}.trace").as_str(),
                "../programs/gem5_arm_script.py",
                "--binary",
                bfp,
            ],
            "workspace",
        );
        println!(
            "\ngem5 saved trace result at: 'workspace/gem5_branch_traces/{file_name}.trace'\n"
        );
    }

    println!("\n======================== Simulation Finished! =========================");
}
