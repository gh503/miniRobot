#[cfg(test)]
mod global_test {
    pub fn setup() {
        // 全局初始化代码
        println!("全局初始化");
    }

    pub fn teardown() {
        // 全局清理代码
        println!("全局清理");
    }
}