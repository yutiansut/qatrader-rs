extern crate websocket;

use std::io::stdin;
use std::sync::mpsc::channel;
use std::thread;
use std::collections::HashMap;
use toml::Value;
use websocket::client::ClientBuilder;
use websocket::{Message, OwnedMessage};
use log::{error, info, warn};
use qatrade_rs::qawebsocket::QAWebSocket;
use qatrade_rs::qaeventmq::QAEventMQ;
use qatrade_rs::config::CONFIG;
use qatrade_rs::qatrader::QATrader;


fn main() {
    log4rs::init_file("conf/log4rs.yaml", Default::default()).unwrap();
    let user_name: String = CONFIG.common.user_name.clone();
    let password: String = CONFIG.common.password.clone();
    let broker: String = CONFIG.common.broker.clone();
    let wsuri: String = CONFIG.common.wsuri.clone();
    info!("Connecting to {}", wsuri);

    let client = match ClientBuilder::new(&wsuri)
        .unwrap()
        .add_protocol("rust-websocket")
        .connect_insecure() {
        Ok(c) => {
            info!("Successfully connected");
            c
        }
        Err(e) => {
            error!("{:?}", e);
            std::process::exit(1);
        }
    };

    let (receiver, sender) = client.split().unwrap();
    let (ws_send, rx) = channel();
    let (db_send, db_rx) = channel();

    let ws_send_1 = ws_send.clone();
    let ws_send_2 = ws_send.clone();
    let ws_send_3 = ws_send.clone();
    let ws_send_4 = ws_send.clone();
    let db_send_1 = db_send.clone();


    let mq_loop = thread::spawn(move || {
        let client = QAEventMQ {
            amqp: CONFIG.mq.uri.clone(),
            exchange: CONFIG.mq.exchange.clone(),
            routing_key: CONFIG.mq.routing_key.clone(),
        };
        client.consume_direct(ws_send_1)
    });

    let send_loop = thread::spawn(move || {
        QAWebSocket::send_loop(sender, rx)
    });

    let receive_loop = thread::spawn(move || {
        // Receive loop
        QAWebSocket::receive_loop(receiver, ws_send_2, db_send_1)
    });

    // login
    QAWebSocket::on_open(ws_send_3);
    let mut qatrade = QATrader::new(ws_send_4, user_name, password, wsuri, broker, "default".to_string(),
                                    CONFIG.mq.uri.clone(), 5, "".to_string(), "".to_string(), "".to_string(),
    );
    loop {
        if let Ok(data) = db_rx.recv() {
            qatrade.sync(data);
        };
    }
    // We're exiting

    info!("Waiting for child threads to exit");

    let _ = send_loop.join();
    let _ = receive_loop.join();

    info!("Exited");
}