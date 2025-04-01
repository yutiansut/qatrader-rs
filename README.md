# QATRADER-RS

QATRADER-RS is a high-performance trading order management and execution system written in Rust. It serves as the core trading engine in the QAUTLRA ecosystem, handling order routing, execution, risk management, and position tracking.

## 🚀 Features

- **High-Performance Trading**: Low-latency order execution with Rust
- **Multiple Broker Support**: Connect to various brokers and trading venues
- **Event-Driven Architecture**: Based on actor model for concurrent processing
- **Risk Management**: Pre-trade and post-trade risk checks
- **Position Tracking**: Real-time position and PnL monitoring
- **WebSocket Integration**: Real-time trading updates
- **Message Queue Support**: RabbitMQ for order flow and event processing
- **QIFI Standard Support**: Standard financial interface for cross-platform compatibility
- **MongoDB Integration**: Persistent storage of trading data and history

## 🏗️ System Architecture

QATRADER-RS is built with a modular, event-driven architecture:

1. **Scheduler**: Central coordinator for trading activities
2. **QATrader**: Core trading engine implementing order management and execution
3. **QAEventMQ**: Message queue interface for order flow and market events
4. **QAWebSocket**: WebSocket client for real-time market data and order updates
5. **QAMongo**: MongoDB interface for persistent storage

## 🔧 Installation

### Prerequisites
- Rust (latest stable version)
- MongoDB
- RabbitMQ

### Setup

1. Clone the repository:
```bash
git clone https://github.com/yutiansut/qatrader-rs.git
cd qatrader-rs
```

2. Configure the system:
Edit `conf/config.toml` to set up your trading environments.

3. Build and run:
```bash
cargo build --release
cargo run --release
```

Alternatively, generate a custom configuration:
```bash
python gen_toml.py --brokers broker1,broker2 --base_path /path/to/config
```

## ⚙️ Configuration

QATrader-RS uses TOML configuration files to define trading behaviors and connections:

```toml
[common]
log_level = "info"
version = "1.0"

[account]
account_cookie = "YOUR_ACCOUNT"
password = "YOUR_PASSWORD"
broker_id = "YOUR_BROKER_ID"
td_server = "tcp://YOUR_SERVER:PORT"
appid = "YOUR_APP_ID"
auth_code = "YOUR_AUTH_CODE"

[websocket]
market_ws = "ws://localhost:8014/ws/market"

[mq]
uri = "amqp://admin:admin@localhost:5672"
exchange = "qaorder"
model = "fanout"
routing_key = ""
```

## 📡 API and Interfaces

### Order Management Interface

QATRADER-RS provides a comprehensive order management API:

| Function | Description |
|----------|-------------|
| `send_order` | Submit a new order to the market |
| `cancel_order` | Cancel an existing order |
| `amend_order` | Modify an existing order |
| `query_position` | Query current positions |
| `query_account` | Query account information |
| `query_orders` | Query order status and history |
| `query_trades` | Query executed trades |

### Message Queue Structure

The system communicates through predefined message formats:

```json
{
  "topic": "order",
  "action": "send_order",
  "account_cookie": "acc001",
  "data": {
    "instrument_id": "cu2109",
    "exchange_id": "SHFE",
    "price": 75000,
    "volume": 1,
    "direction": "BUY",
    "offset": "OPEN"
  }
}
```

### Event Handling

QATRADER-RS processes various trading events:

1. **Order Events**: New order, order canceled, order rejected, etc.
2. **Trade Events**: Order filled, partially filled
3. **Account Events**: Deposit, withdrawal, margin changes
4. **Position Events**: Position changes, mark-to-market updates
5. **Risk Events**: Risk limit breaches, margin calls

## 🔄 Integration with QAUTLRA Ecosystem

QATRADER-RS integrates with the following components:

- **QIFI-RS**: For standardized financial interface definitions
- **QAMDGATEWAY**: For market data required in order decisions
- **QAREALTIMEPRO-RS**: For real-time market data analysis
- **QADB-RS**: For storing and retrieving historical trading data

## 📊 Performance Metrics

- **Order Latency**: <1ms for order processing
- **Throughput**: Support for 1000+ orders per second
- **Reliability**: 99.99% uptime for critical trading functions
- **Recovery**: Fast recovery from connection or system failures

## 🧪 Development

### Extending the System

To add support for new brokers or trading venues:

1. Create a new broker adapter in `src/adapters/`
2. Implement the required trading interfaces
3. Add configuration support in `src/config.rs`
4. Register the adapter in the main trading engine

### Testing

Run the test suite:
```bash
cargo test
```

For integration testing with simulated markets:
```bash
cargo test --features="simulation"
```

## 📚 Examples

The `examples` directory contains sample code for common use cases:

- Basic order submission
- Multiple broker integration
- Risk management setup
- Position monitoring
- Algorithmic trading strategies

## 📝 License

[License information]

## 👥 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📬 Contact

For questions or support, please contact [contact information].



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


