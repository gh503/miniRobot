use std::process::{Command, Output};
use std::fs;
use std::str;

fn run_python_script(script_path: &str, args: &[&str]) -> Result<(Output, String), String> {
    let output = Command::new("python3")
        .arg(script_path)
        .args(args)
        .output()
        .expect(&format!("Failed to execute {} with arguments {:?}", script_path, args));
    
    let stdout = match str::from_utf8(&output.stdout) {
        Ok(v) => v.to_string(),
        Err(e) => return Err(format!("Failed to parse stdout: {}", e)),
    };

    let stderr = match str::from_utf8(&output.stderr) {
        Ok(v) => v.to_string(),
        Err(e) => return Err(format!("Failed to parse stderr: {}", e)),
    };

    if !output.status.success() {
        return Err(format!("Script {} with arguments {:?} failed: {}\n{}", script_path, args, stdout, stderr));
    }

    Ok((output, stdout))
}

fn run_test_module(module_path: &str) -> Result<(Output, String), String> {
    let output = Command::new("python3")
        .arg("-m")
        .arg("unittest")
        .arg("discover")
        .arg("-s")
        .arg(module_path)
        .arg("-p")
        .arg("*.py")
        .output()
        .expect("Failed to discover tests");

    let stdout = match str::from_utf8(&output.stdout) {
        Ok(v) => v.to_string(),
        Err(e) => return Err(format!("Failed to parse stdout: {}", e)),
    };

    let stderr = match str::from_utf8(&output.stderr) {
        Ok(v) => v.to_string(),
        Err(e) => return Err(format!("Failed to parse stderr: {}", e)),
    };

    if !output.status.success() {
        return Err(format!("Test module {} failed: {}\n{}", module_path, stdout, stderr));
    }

    Ok((output, stdout))
}

fn run_test_suite(suite_path: &str) -> Result<(), String> {
    // Run the suite setup
    match run_python_script(&format!("{}/testsuite.py", suite_path), &["setup"]) {
        Ok((_, stdout)) => println!("Suite setup for {} succeeded:\n{}", suite_path, stdout),
        Err(e) => return Err(e),
    }

    match run_test_module(suite_path) {
        Ok((_, stdout)) => {
            println!("Test module {} succeeded:\n{}", suite_path, stdout);
        },
        Err(e) => {
            return Err(e);
        }
    }

    // Run the suite teardown
    match run_python_script(&format!("{}/testsuite.py", suite_path), &["teardown"]) {
        Ok((_, stdout)) => println!("Suite teardown for {} succeeded:\n{}", suite_path, stdout),
        Err(e) => return Err(e),
    }

    Ok(())
}

#[test]
fn integration_tests() {
    // Global setup
    match run_python_script("tests/integration_test/global.py", &["setup"]) {
        Ok((_, stdout)) => println!("Global setup succeeded:\n{}", stdout),
        Err(e) => panic!("{}", e),
    }

    for entry in fs::read_dir("tests/integration_test").expect("Tests directory not found") {
        let entry = entry.expect("Error reading directory entry");
        let path = entry.path();
        if path.is_dir() {
            if let Err(e) = run_test_suite(path.to_str().unwrap()) {
                panic!("{}", e);
            }
        }
    }

    // Global teardown
    match run_python_script("tests/integration_test/global.py", &["teardown"]) {
        Ok((_, stdout)) => println!("Global teardown succeeded:\n{}", stdout),
        Err(e) => panic!("{}", e),
    }
}
