use std::collections::{HashMap, HashSet};
use std::fs;
use crate::tests::test_case::TestCase;
use crate::tests::test_module::discover_test_cases;
use crate::tests::test_order::TestOrder;
use crate::local::cpu::CpuInfo;
use rayon::prelude::*;
use rayon::iter::IntoParallelRefIterator;

/// 测试套结构体
pub struct TestSuite {
    name: String,
    modules: Vec<String>,
    test_cases: Vec<TestCase>,
    modules_test_cases: HashMap<String, Vec<TestCase>>,
    parallel: usize,
    order: TestOrder,
}

impl TestSuite {
    /// 创建一个新的测试套
    pub fn new(name: &str, parallel: Option<usize>, order: Option<TestOrder>) -> Option<Self> {
        // 查找测试套下的所有测试用例
        let suite_path = format!("tests/{}", name);
        let mut modules = Vec::new();
        let mut test_cases = Vec::new();
        let mut modules_test_cases = HashMap::new();
        if let Ok(entries) = fs::read_dir(&suite_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(file_name) = path.file_name() {
                            let file_name = file_name.to_string_lossy().to_string();
                            if file_name.ends_with("_test.rs") && file_name.ne("module_test.rs") {
                                let module_name = file_name.trim_end_matches("_test.rs").to_string();
                                modules.push(module_name.clone());
                                let mod_test_cases = discover_test_cases(&module_name, &name);
                                test_cases.append(&mut mod_test_cases.clone());
                                modules_test_cases.insert(module_name, mod_test_cases);
                            }
                        }
                    }
                }
            }
            if modules.is_empty() || test_cases.is_empty() {
                None
            } else {
                Some(Self {
                    name: name.to_string(),
                    modules,
                    test_cases,
                    modules_test_cases,
                    parallel: parallel.unwrap_or(CpuInfo::new().get_cpu_thread_count().parse().unwrap()),
                    order: order.unwrap_or(TestOrder::Sequential),
                })
            }
        } else {
            None
        }
    }

    /// 运行筛选后的测试用例
    pub fn run_filtered(&self, module_names: &HashSet<String>, test_case_names: &HashSet<String>) {
        println!("Running test suite: {}", self.name);
        
        let filtered_modules: Vec<&String> = if module_names.is_empty() {
            self.modules.iter().collect()
        } else {
            self.modules.iter().filter(|m| module_names.contains(*m)).collect()
        };

        let filtered_test_cases: Vec<&TestCase> = if test_case_names.is_empty() {
            self.test_cases.iter().collect()
        } else {
            self.test_cases.iter().filter(|tc| test_case_names.contains(tc.get_name())).collect()
        };

        match self.order {
            TestOrder::Sequential => {
                for module_name in filtered_modules {
                    if let Some(test_cases) = self.modules_test_cases.get(module_name) {
                        for test_case in test_cases {
                            if filtered_test_cases.contains(&test_case) {
                                test_case.run();
                            }
                        }
                    }
                }
            }
            TestOrder::Parallel => {
                rayon::ThreadPoolBuilder::new()
                    .num_threads(self.parallel)
                    .build_global()
                    .unwrap();
                filtered_test_cases.par_iter().for_each(|test_case| {
                    test_case.run();
                });
            }
        }
    }
}
