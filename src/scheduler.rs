use websocket::{OwnedMessage, ClientBuilder, WebSocketError};
use std::thread;
use crossbeam_channel::{unbounded, Sender, Receiver};
use log::{error, info, warn};
use chrono::Local;
use actix::prelude::*;

use crate::qaeventmq::QAEventMQ;
use crate::qawebsocket::QAWebSocket;
use crate::config::CONFIG;
use crate::qatrader::QATrader;
use std::time::Duration;

pub enum Event {
    RESTART
}

pub struct Scheduler {
    pub s_c: (Sender<Event>, Receiver<Event>),
    pub ws_channel: (Sender<OwnedMessage>, Receiver<OwnedMessage>),
    pub db_channel: (Sender<String>, Receiver<String>),
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            s_c: unbounded(),
            ws_channel: unbounded(),
            db_channel: unbounded(),
        }
    }

    pub fn start_mq_loop(&mut self) {
        let ws_send = self.ws_channel.0.clone();
        let mq_loop = thread::spawn(move || {
            let client = QAEventMQ {
                amqp: CONFIG.common.eventmq_ip.clone(),
                exchange: "QAORDER_ROUTER".to_string(),
                routing_key: CONFIG.common.account.clone(),
            };
            client.consume_direct(ws_send)
        });
    }

    pub fn start_ws_loop(&mut self) -> Result<(), WebSocketError> {
        let (sender, receiver) = QAWebSocket::connect(&CONFIG.common.wsuri)?;

        let ws_rece = self.ws_channel.1.clone();
        let ws_send = self.ws_channel.0.clone();
        let s_c_send1 = self.s_c.0.clone();
        let s_c_send2 = self.s_c.0.clone();
        let db_send_1 = self.db_channel.0.clone();

        let send_loop = thread::spawn(move || {
            QAWebSocket::send_loop(sender, ws_rece, s_c_send1)
        });

        let receive_loop = thread::spawn(move || {
            QAWebSocket::receive_loop(receiver, ws_send, db_send_1, s_c_send2)
        });

        QAWebSocket::login(self.ws_channel.0.clone());
        Ok(())
    }


    pub fn start_trader_loop(&mut self) {
        let ws_send = self.ws_channel.0.clone();
        let db_rx = self.db_channel.1.clone();
        let trade_loop = thread::spawn(move || {
            let mut qatrade = QATrader::new(ws_send, CONFIG.common.account.clone(), CONFIG.common.password.clone(),
                                            CONFIG.common.wsuri.clone(), CONFIG.common.broker.clone(), CONFIG.common.portfolio.clone(),
                                            CONFIG.common.eventmq_ip.clone(), CONFIG.common.ping_gap.clone(),
                                            CONFIG.common.bank_password.clone(), CONFIG.common.capital_password.clone(),
                                            CONFIG.common.taskid.clone(),
            );
            loop {
                if let Ok(data) = db_rx.recv() {
                    qatrade.sync(data);
                };
            }
        });
    }


    pub fn wait(&mut self) {
        match self.s_c.1.try_recv() {
            Ok(event) => {
                match event {
                    Event::RESTART => {
                        warn!("WebSocket Try Reconnecting");
                        if let Err(e) = self.start_ws_loop() {
                            error!("{:?}", e);
                            self.s_c.0.send(Event::RESTART);
                        }
                    }
                    _ => ()
                }
            }
            _ => ()
        }
    }

    pub fn ping(&mut self) {
        let msg = OwnedMessage::Ping(format!("ping-{}", CONFIG.common.account).as_bytes().to_vec());
        self.ws_channel.0.send(msg);
    }
}

impl Actor for Scheduler {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_secs(1), |act, ctx| {
            act.wait();
        });
        ctx.run_interval(Duration::from_secs(CONFIG.common.ping_gap as u64), |act, ctx| {
            act.ping();
        });
    }
}