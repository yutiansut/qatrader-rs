# qatrader-rs

rust version for qatrader, for a high performance with limited resources




```toml
mongodb = "0.3.11"
tokio = { version = "0.2", features = ["full"] }
lapin = "^0.28"
log = "^0.4"
tokio-tungstenite = "*"
env_logger = "^0.7" 
serde_json = "1.0"
rayon = "1.1"
ndarray = "0.13.0"
```


如果需要编译:


```
rustup install nightly
cargo +nightly build

```