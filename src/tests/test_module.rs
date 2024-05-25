use std::fs;
use crate::local::cpu::CpuInfo;
use crate::tests::test_case::{TestCase, call_function};
use crate::tests::test_order::TestOrder;
use rayon::ThreadPoolBuilder;

pub struct TestModule {
    name: String,
    suite: String,
    test_cases: Vec<TestCase>,
    parallel: usize,
    order: TestOrder,
}

impl TestModule {
    pub fn new(name: &str, suite: &str, parallel: Option<usize>, order: Option<TestOrder>) -> Option<Self> {
        let file_path = format!("tests/{}/{}_test.rs", suite, name);
        if fs::metadata(&file_path).is_ok() {
            let test_cases = discover_test_cases(name, suite);
            Some(Self {
                name: name.to_string(),
                suite: suite.to_string(),
                test_cases,
                parallel: parallel.unwrap_or(CpuInfo::new().get_cpu_thread_count().parse().unwrap()),
                order: order.unwrap_or(TestOrder::Sequential),
            })
        } else {
            None
        }
    }

    pub fn run(&self) {
        println!("Running test module: {}", self.name);

        if let Err(e) = call_function("tests/global_test.rs", "setup") {
            println!("Error in global setup: {:?}", e);
            return;
        }

        let module_test_file = format!("tests/{}/module_test.rs", self.suite);
        if let Err(e) = call_function(&module_test_file, "setup") {
            println!("Error in module setup: {:?}", e);
            return;
        }

        match self.order {
            TestOrder::Sequential => { for test_case in &self.test_cases {
                    test_case.run();
                }
            }
            TestOrder::Parallel => {
                let pool = ThreadPoolBuilder::new()
                    .num_threads(self.parallel)
                    .build()
                    .unwrap();
                pool.scope(|s| {
                    for test_case in &self.test_cases {
                        s.spawn(|_| {
                            test_case.run();
                        });
                    }
                });
            }
        }

        if let Err(e) = call_function(&module_test_file, "teardown") {
            println!("Error in module teardown: {:?}", e);
        }

        if let Err(e) = call_function("tests/global_test.rs", "teardown") {
            println!("Error in global teardown: {:?}", e);
        }
    }
}

pub fn discover_test_cases(module: &str, suite: &str) -> Vec<TestCase> {
    let mut test_cases = Vec::new();
    let file_path = format!("tests/{}/{}_test.rs", suite, module);
    if let Ok(content) = fs::read_to_string(&file_path) {
        for line in content.lines() {
            if line.trim().starts_with("fn test_") {
                let start = line.find("test_").unwrap();
                let end = line.find('(').unwrap();
                let name = &line[start + 5..end];
                if let Some(test_case) = TestCase::new(name, module, suite) {
                    test_cases.push(test_case);
                }
            }
        }
    }
    test_cases
}
