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

```


如果需要编译:


```
rustup install nightly
cargo +nightly build

```

运行：

```
cargo run --release conf\boot.toml

```

