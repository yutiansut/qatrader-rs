# qatrader-rs

rust version for qatrader, for a high performance with limited resources


支持单机上万账户并发的 qatrader rust版本

支持realtime 模拟盘版本 / ctp实盘版本 / 以及支持qifi协议的自建网关版本

本项目需要配合QATRADEG使用(https://github.com/yutiansut/QAtradeG)



```toml
websocket = "0.26.2"

uuid = { version = "0.8", features = ["serde", "v4"] }
# 序列化
regex = "1.3.6"
serde_json = "1.0"
serde_derive = "1.0"
serde = { version = "1.0", features = ["derive"] } # 序列化
#
mongodb = "0.9.1"
bson = "0.14.0"
amiquip = "0.3"
log = "0.4"
# 配置日志
clap = "2.33"
toml = "0.5"
log4rs="0.12"
env_logger = "0.7"
lazy_static = "1.4.0"
chrono = { version = "0.4", features = ["serde"] } # datetime

qifi-rs = {git="https://github.com/QUANTAXIS/qifi-rs.git"}
crossbeam-channel = "0.5.0"

```


如果需要编译:


```
rustup install nightly
cargo +nightly build

```

运行：

1.
    ```
    qatrader-rs.exe --account=xxxxx --password=xxxxx --broker=simnow --wsuri=ws://192.168.2.124:7988 --database_ip=mongodb://localhost:27017 --eventmq_ip=amqp://admin:admin@192.168.2.125:5672/ --log_level=debug
    ```

2.
    ```
    qatrader-rs.exe -c conf\boot.toml
    ```
   boot.toml
    ```toml
    [common]
    account= ""
    password= ""
    broker= "simnow"
    wsuri= "ws://192.168.2.124:7988"
    eventmq_ip="amqp://admin:admin@192.168.2.125:5672/"
    database_ip="mongodb://localhost:27017"
    ping_gap=5
    taskid=""
    portfolio="default"
    bank_password=""
    capital_password=""
    appid=""
    log_level="debug"
    ``` 


辅助运维脚本:  gen_toml.py

python gen_toml.py --account_cookie 你的账户 --password 你的密码 --broker 你在 qatradeG 预设好的 broker --wsuri QAtradeG的 websocket --eventmq_ip 你下单的 rabbitmq 的 amqp 协议地址  --database_ip mongodb 协议地址






关于 BROKER,  

- QUANTAXIS 是一个单独的本地 sim 账户, 他和正常的 simnow 功能完全一致, 在 simnow 失效的时候 你可以用这个账户来模拟 simnow
- simnow 是上期技术的官方 sim 账户
- 你自由配置的账户, 符合 CTP 接口即可(主席次席均支持)  具体查看 QATRADEG 的配置


