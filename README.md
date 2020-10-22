# qatrader-rs

rust version for qatrader, for a high performance with limited resources




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
    qatrader-rs.exe --account=150083 --password=980817 --broker=simnow --wsuri=ws://192.168.2.124:7988 --database_ip=mongodb://localhost:27017 --eventmq_ip=amqp://admin:admin@192.168.2.125:5672/ --log_level=debug
    ```

2.
    ```
    qatrader-rs.exe -c conf\boot.toml
    ```
   boot.toml
    ```toml
    [common]
    account= "150083"
    password= "980817"
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