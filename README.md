# miniRobot
全平台机器本地代理节点。应用场景：
- 即时查询主机信息：推荐 `minirobot_info`
- 本地任务执行和主机监控：推荐 `minirobot_manage`
- 管理远端机器和节点间通信等：推荐 `minirobot`

基础功能支撑作为机器代理节点，可以作为测试系统执行器、分布式节点通信、后台进程等。

## 功能

### 1.本地功能

#### 1.1.查询
本地查询功能以`minirobot_info`工具的形式提供，可单独使用。

##### 1.1.1.工具部署
提供`Linux`环境部署指导。工具仅做查询，因查询系统所有进程信息，设置`root`属主，普通用户执行亦可。

- `Linux`系统
```bash
sudo cp minirobot_info /usr/local/bin/
sudo chown root:root /usr/local/bin/minirobot_info
sudo 4755 /usr/local/bin/minirobot_info # setuid位，执行以文件所有者权限执行
```

##### 1.1.2.本机信息
- [x] 即时采集主机信息
- [x] 结果支持`json`输出

本机采集信息清单：
- `Hostname`
- `OS`:
    - `type`
    - `name`
    - `version`
    - `arch`
- `Cpu`: `socket`、`core`、`thread`
- `Memory`: `Total`、`Swap`
- `Disk`: `Block devices`、`Partition size info`
- `NIC`: `active nic info`
    - `Name`
    - `MAC`
    - `Status`
    - `IPv4 address`
    - `IPv6 address`

![采集本机信息](https://cdn.jsdelivr.net/gh/gh503/CDN@latest/shotimg/host_info.png)

##### 1.1.3.本地进程
- [x] 即时查询主机所有进程并返回进程信息
- [x] 支持按`PID`和命令关键词过滤
- [x] 支持反向滤除
- [x] 结果支持`json`输出

进程信息清单：
- PID
- Exec_Path
- Command
- Full Command

![本地进程过滤](https://cdn.jsdelivr.net/gh/gh503/CDN@latest/shotimg/process_filter.png)

#### 1.2.管理
本地管理功能以工具`minirobot_manage`形式提供，可单独使用。

##### 1.2.1.工具部署
工具推荐以系统服务的形式部署运行，最高用户权限。

##### 1.2.1.本地任务管理
- [ ] 对提交到本地的任务（远端节点或者本地触发）进行队列处理
- [ ] 消费任务队列，解析任务信息，更新并存储任务日志
- [ ] 执行本机命令，获取命令结果、返回信息
- [ ] 支持`json`输出

#### 1.2.2.主机资源监控
- [ ] 磁盘不足监控
- [ ] 新增开放端口监控
- [ ] 任务异常监控
- [ ] 系统气泡告警

### 2.网络功能
网络功能以工具`minirobot`形式提供。可单独使用。

#### 2.1.工具部署
工具推荐以系统服务的形式部署运行，普通用户如`www-data`用户运行即可，不可以最高用户权限运行。

#### 2.2.数据加解密
- [x] 支持`AES256`、`ChaCha20`加解密和签名、验证

#### 2.3.用户认证
- [x] 密码认证

#### 2.4.主机管理
- [ ] 支持配置远程主机
- [ ] 支持远程主机的`ssh`登入
- [ ] 支持远程主机的`minirobot`服务部署
- [ ] 支持远程主机命令执行、结果和输出的读取

#### 2.5.节点管理
- [ ] 支持手动配置节点
- [ ] 支持自动配置节点
- [ ] 探测机器所在局域网内活跃的主机`minirobot`对等节点
- [ ] 支持节点状态测试
- [ ] 支持节点`gRPC`通信
- [ ] 支持`websocket`通信

## 被集成
系统支持以库的形式调用，提供`libminirobot.rlib`。包名为`minirobot`。

```rust
use minirobot;
use minirobot::local;
use minirobot::network;
use minirobot::security;
```

## 测试
### 1.单元测试
- [x] 基于`Cargo`内置功能编写，执行`cargo test`自动触发单元测试。

### 2.集成测试
- [x] 使用`Python3`内置`unittest`测试框架。

用例管理在`tests/`目录下，当前集成测试有`integration_test/`。执行流程：
```txt
integration_test/
global/setup ->
    testsuite/setup
        testmodule/setup
            testcase1
            testcase2
            ...
        testmodule/teardown
    testsuite/teardown
global/teardown
```

不打印详细：
![](https://cdn.jsdelivr.net/gh/gh503/CDN@latest/shotimg/integration_test.png)

打印详细：
![](https://cdn.jsdelivr.net/gh/gh503/CDN@latest/shotimg/integration_test_details.png)

## 项目开源计划
- [x] `Cargo`包平台: [minirobot](https://crates.io/crates/minirobot)
- [ ] `CompassCI`平台测试
- [ ] `openEuler`社区
- [ ] `Linux`社区

## 参与贡献
若有兴趣，非常欢迎`fork`项目参与贡献。
