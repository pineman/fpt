use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

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
    enabled: Option<bool>,
}

// build script's entry point
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let destination = dbg!(Path::new(&out_dir).join("tests.rs"));
    let mut test_file = File::create(destination).unwrap();

    write_header(&mut test_file);
    write_test(&mut test_file, "tests/rom_tests.json");
}

fn write_test(test_file: &mut File, directory: &str) {
    let source = dbg!(std::fs::read_to_string(directory).unwrap());
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
        )
        .unwrap();
    }
}

fn write_header(test_file: &mut File) {
    write!(test_file, include_str!("./tests/templates/header")).unwrap();
}
