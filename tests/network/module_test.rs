#[cfg(test)]
mod network_test {
    pub fn setup() {
        // 在这里执行测试前的准备工作，比如初始化数据、建立连接等
        println!("Setup for module tests");
    }

    pub fn teardown() {
        // 在这里执行测试后的清理工作，比如关闭连接、删除临时文件等
        println!("Teardown after module tests");
    }
}