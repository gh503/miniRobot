# miniRobot
机器本地代理节点。应用场景：
- 随手即时查询主机信息：推荐 `mini_robot_info`
- 主机本地任务执行和节点间通信：推荐 `mini_robot`
- 测试远端节点：推荐 `mini_robot_test`

基础功能支撑作为机器代理节点，可以作为测试系统执行器、分布式节点通信、后台进程等。

## 关键功能

### 1.本地功能
#### 1.1.采集本机信息
>[x] 即时采集主机信息。查询结果支持json输出。
本机采集信息清单：
- Hostname
- OS:
    - type
    - name
    - version
    - arch
- Cpu: socket、core、thread
- Memory: Total、Swap
- Disk: Block devices、Partition size info
- NIC: all and active nic info
    - Name
    - MAC
    - Status
    - IPv4 address
    - IPv6 address

![采集本机信息](https://cdn.jsdelivr.net/gh/gh503/CDN@latest/shotimg/host_info.png)

#### 1.2.本地进程过滤
>[x] 即时查询主机所有进程并返回进程信息，并支持按PID和命令关键词过滤和反向滤除。查询结果支持json输出。
进程信息清单：
- PID
- Exec_Path
- Command
- Full Command

![本地进程过滤](https://cdn.jsdelivr.net/gh/gh503/CDN@latest/shotimg/process_filter.png)

#### 1.3.执行本地命令
>[ ] 执行本机命令，获取命令结果和返回信息。支持json输出。

### 2.网络功能
#### 2.1.网络加解密
>[x] 支持AES256、ChaCha20加解密和签名、验证。

#### 2.2.用户校验
>[x] 解密后合法用户认证。

#### 2.3.网络节点探测
>[ ] 探测机器局域网内活跃的主机mini_robot对等节点。

#### 2.4.网络通信
>[ ] 与可信对端通信。包括对等机器mini_robot节点和远端浏览器，其中浏览器通信用于支持节点的平台对接。

#### 2.3.网络节点管理
>[ ] 管理对等节点信息。

## 测试
### 1.单元测试
基于`Cargo`内置功能编写，执行`cargo test`自动触发单元测试。

### 2.集成测试
使用`Python3`内置`unittest`测试框架。
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
>[ ] CompassCI平台测试
>[ ] openEuler社区
>[ ] Linux社区

## 参与贡献
若有兴趣，非常欢迎fork项目参与贡献。
