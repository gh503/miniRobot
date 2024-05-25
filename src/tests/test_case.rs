use std::fs;

// 测试用例结构体
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TestCase {
    name: String,
    module: String,
    suite: String,
}

impl TestCase {
    /// 创建一个新的测试用例
    pub fn new(name: &str, module: &str, suite: &str) -> Option<Self> {
        // 构建测试用例的文件路径
        let file_path = format!("tests/{}/{}_test.rs", suite, module);

        // 检查文件是否存在
        if fs::metadata(&file_path).is_ok() {
            Some(Self {
                name: name.to_string(),
                module: module.to_string(),
                suite: suite.to_string(),
            })
        } else {
            // 检测失败需要处理
            None
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// 执行测试用例
    pub fn run(&self) {
        println!("Running test case: {}", self.name);

        // 执行全局设置函数
        if let Err(e) = call_function("tests/global_test.rs", "setup") {
            println!("Error in global setup: {:?}", e);
            return;
        }

        // 执行模块级别的设置函数
        let module_test_file = format!("tests/{}/module_test.rs", self.suite);
        if let Err(e) = call_function(&module_test_file, "setup") {
            println!("Error in module setup: {:?}", e);
            return;
        }

        // 执行测试用例级别的设置函数
        let test_case_setup_file = format!("tests/{}/{}_test.rs", self.suite, self.module);
        if let Err(e) = call_function(&test_case_setup_file, "setup") {
            println!("Error in test case setup: {:?}", e);
            return;
        }

        // 执行实际的测试用例
        let test_case_function = format!("test_{}", self.name);
        if let Err(e) = call_function(&test_case_setup_file, &test_case_function) {
            println!("Error in test case: {:?}", e);
        }

        // 执行测试用例级别的清理函数
        if let Err(e) = call_function(&test_case_setup_file, "teardown") {
            println!("Error in test case teardown: {:?}", e);
        }

        // 执行模块级别的清理函数
        if let Err(e) = call_function(&module_test_file, "teardown") {
            println!("Error in module teardown: {:?}", e);
        }

        // 执行全局清理函数
        if let Err(e) = call_function("tests/global_test.rs", "teardown") {
            println!("Error in global teardown: {:?}", e);
        }
    }
}


pub fn call_function(file_path: &str, function_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 这里可以使用动态加载库的方式调用函数或者使用子进程执行特定的测试文件和函数
    let status = std::process::Command::new("cargo")
        .arg("test")
        .arg("--")
        .arg(format!("{}::{}", file_path.replace(".rs", ""), function_name))
        .status()?;

    if !status.success() {
        return Err(format!("Function {} in file {} failed", function_name, file_path).into());
    }
    Ok(())
}
