use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Suite {
    tests: Vec<Test>,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Test {
    id: u32,
    path: String,
    termination_address: String,
    passing: Option<bool>,
    enabled: Option<bool>,
}

fn generate_rom_tests() {
    println!("cargo:rerun-if-changed=tests");
    let out_dir = env::var("OUT_DIR").unwrap();
    let destination = Path::new(&out_dir).join("tests.rs");
    let mut test_file = File::create(destination).unwrap();

    write_header(&mut test_file);
    write_test(&mut test_file, "tests/rom_tests.json");
}

fn write_test(test_file: &mut File, directory: &str) {
    let source = std::fs::read_to_string(directory).unwrap();
    let suite: Suite = serde_json::from_str(&source).unwrap();

    for test in suite.tests {
        if test.enabled.is_some() && !test.enabled.unwrap() {
            continue;
        }
        let test_name = format!("{}_{}", suite.name, test.id);

        write!(
            test_file,
            include_str!("./tests/templates/test"),
            name = test_name,
            path = test.path,
            termination_address = test.termination_address,
            passing = if test.passing.unwrap_or(true) {
                "true"
            } else {
                "false"
            },
        )
        .unwrap();
    }
}

fn write_header(test_file: &mut File) {
    write!(test_file, include_str!("./tests/templates/header")).unwrap();
}

fn run_cmd(cmd: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");
    if !output.status.success() {
        panic!("Command {} failed with exit code: {}", cmd, output.status);
    }
    String::from_utf8(output.stdout).expect("Failed to convert output to string")
}

fn fetch_mooneye_test_roms() {
    println!("cargo:rerun-if-changed=build.rs");
    run_cmd("curl -L --create-dirs --output-dir ../target/tmp -O https://gekkio.fi/files/mooneye-test-suite/mts-20240127-1204-74ae166/mts-20240127-1204-74ae166.tar.xz");
    run_cmd("mkdir -p ../target/test_roms");
    run_cmd("tar -xvf ../target/tmp/mts-20240127-1204-74ae166.tar.xz -C ../target/test_roms");
    run_cmd("rm -rf ../target/test_roms/mooneye");
    run_cmd("mv ../target/test_roms/mts-20240127-1204-74ae166 ../target/test_roms/mooneye");
}

fn main() {
    generate_rom_tests();
    fetch_mooneye_test_roms();
}
