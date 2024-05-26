use pyo3::prelude::*;
use pyo3::types::PyModule;
use rust_embed::RustEmbed;
use std::{collections::HashMap, path::PathBuf};

#[derive(RustEmbed)]
#[folder = "tests/"]
struct Asset;

fn setup() {
    println!("Global setup");
}

fn teardown() {
    println!("Global teardown");
}

fn execute_python_script(py: Python, script_content: &str) -> PyResult<()> {
    let _module = PyModule::from_code(py, script_content, "", "")?;
    Ok(())
}

fn run_function_if_exists(py: Python, script: &str, func_name: &str) -> PyResult<()> {
    let module = PyModule::from_code(py, script, "", "")?;
    if let Ok(func) = module.getattr(func_name) {
        func.call0()?;
    }
    Ok(())
}

#[test]
fn integration_test() {
    setup();

    Python::with_gil(|py| {
        let mut suite_scripts: HashMap<String, Vec<String>> = HashMap::new();

        // Categorize scripts by suite directory
        for file in Asset::iter() {
            let path: PathBuf = file.as_ref().into();
            if path.extension().and_then(|s| s.to_str()) == Some("py") {
                if let Some(parent) = path.parent().and_then(|p| p.to_str()) {
                    let path_str = path.to_str().unwrap().to_string();
                    suite_scripts.entry(parent.to_string()).or_insert_with(Vec::new).push(path_str);
                }
            }
        }

        // Execute scripts in each suite
        for (suite, scripts) in &suite_scripts {
            println!("Executing suite: {}", suite);

            // Run suite setup if module_test.py exists
            if let Some(module_test_path) = scripts.iter().find(|&s| s.ends_with("module_test.py")) {
                if let Some(script_content) = Asset::get(module_test_path) {
                    let script_content = std::str::from_utf8(script_content.data.as_ref()).unwrap();
                    run_function_if_exists(py, &script_content, "setup").expect("Failed to run suite setup");
                }
            }

            // Execute each script in the suite
            for script in scripts {
                if script.ends_with("module_test.py") {
                    continue; // module_test.py is only for suite setup/teardown
                }

                if let Some(script_content) = Asset::get(script) {
                    let script_content = std::str::from_utf8(script_content.data.as_ref()).unwrap();
                    println!("Executing script: {}", script);

                    run_function_if_exists(py, &script_content, "setup").expect("Failed to run script setup");
                    execute_python_script(py, &script_content).expect(&format!("Failed to execute script {:?}", script));
                    run_function_if_exists(py, &script_content, "teardown").expect("Failed to run script teardown");
                }
            }

            // Run suite teardown if module_test.py exists
            if let Some(module_test_path) = scripts.iter().find(|&s| s.ends_with("module_test.py")) {
                if let Some(script_content) = Asset::get(module_test_path) {
                    let script_content = std::str::from_utf8(script_content.data.as_ref()).unwrap();
                    run_function_if_exists(py, &script_content, "teardown").expect("Failed to run suite teardown");
                }
            }
        }
    });

    teardown();
}
