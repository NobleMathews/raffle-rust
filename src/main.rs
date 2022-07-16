#![allow(unused)]

use std::path::PathBuf;
use std::process::Command;
use std::fs;
use std::io::{BufWriter, Write};
use clap::Parser;
use anyhow::{Context, Result};
use regex::Regex;

#[derive(Parser)]
struct Cli {
    /// The build pattern to use on project
    build_pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    analysis_path: PathBuf,
    /// Path to clang-build directory
    #[clap(short = 'c', long = "clang-build-dir", default_value_t = String::from("/usr/local/bin/"))]
    clang_build_dir: String
}

/// Use build pattern on project and return list of targets
fn static_targets() -> Result<()> {
    let args = Cli::parse();
    let project_path:&PathBuf = &args.analysis_path;
    let cpp_flags = shell_words::split(&args.build_pattern).expect("failed to parse build command");
    let output = Command::new(PathBuf::from(args.clang_build_dir).join("scan-build"))
                     .args(cpp_flags)
                     .arg(project_path)
                     .output()
                     .expect("failed to execute process");
    let md = fs::metadata(project_path).unwrap();
    assert!(output.status.success());
    println!("Build status: {}", output.status);
    // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    let err_str = String::from_utf8_lossy(&output.stderr);
    let re = Regex::new(r"((?m)^[^.\s]+\.[^:]+):(\d+):(\d+)").unwrap();
    if md.is_dir(){
        fs::create_dir_all(project_path.join("obj-aflgo").join("temp"))?;
        let f = fs::File::create(project_path.join("obj-aflgo").join("temp").join("BBtargets.txt")).expect("unable to create file");
        let mut f = BufWriter::new(f);
        for cap in re.captures_iter(&err_str) {
            write!(f, "{}:{}", &cap[1], &cap[2]).expect("unable to write");
            // println!("File: {} Line: {} Column: {}", &cap[1], &cap[2], &cap[3]);
        }
    }
    else{
        let project_folder = project_path.parent().unwrap();
        fs::create_dir_all(project_folder.join("obj-aflgo").join("temp"))?;
        let f = fs::File::create(project_folder.join("obj-aflgo").join("temp").join("BBtargets.txt")).expect("unable to create file");
        let mut f = BufWriter::new(f);
        for cap in re.captures_iter(&err_str) {
            write!(f, "{}:{}", &cap[1], &cap[2]).expect("unable to write");
            // println!("File: {} Line: {} Column: {}", &cap[1], &cap[2], &cap[3]);
        }
    }
    // let content = std::fs::read_to_string(path)
    //     .with_context(|| format!("could not read file `{}`", path))?;
    // println!("file content: {}", content);
    Ok(())
}


fn main() {
    static_targets();
}
