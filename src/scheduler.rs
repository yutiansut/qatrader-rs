use websocket::{OwnedMessage, ClientBuilder, WebSocketError};
use std::thread;
use log::{error, info, warn};
use chrono::Local;
use actix::prelude::*;
use websocket::sender::Writer;
use std::net::TcpStream;
use crate::qaeventmq::QAEventMQ;
use crate::qawebsocket::QAWebSocket;
use crate::config::CONFIG;
use crate::qatrader::QATrader;
use std::time::Duration;
use std::sync::{Mutex, Arc};
use crate::xmsg::XReqLogin;


pub struct Scheduler {
    pub trader: QATrader,
    pub ws_sender: Option<Writer<TcpStream>>,
}

impl Scheduler {
    pub fn new() -> Self {
        let trader = QATrader::new(CONFIG.common.account.clone(), CONFIG.common.password.clone(),
                                   CONFIG.common.wsuri.clone(), CONFIG.common.broker.clone(), CONFIG.common.portfolio.clone(),
                                   CONFIG.common.eventmq_ip.clone(), CONFIG.common.ping_gap.clone(),
                                   CONFIG.common.bank_password.clone(), CONFIG.common.capital_password.clone(),
                                   CONFIG.common.taskid.clone());
        Self {
            trader,
            ws_sender: None,
        }
    }

    pub fn start_mq_loop(&mut self, addr: Addr<Scheduler>) {
        let mq_loop = thread::spawn(move || {
            let client = QAEventMQ {
                amqp: CONFIG.common.eventmq_ip.clone(),
                exchange: "QAORDER_ROUTER".to_string(),
                routing_key: CONFIG.common.account.clone(),
            };
            client.consume_direct(addr)
        });
    }

    pub fn start_ws_receive_loop(&mut self, addr: Addr<Scheduler>) -> Result<(), WebSocketError> {
        let (sender, receiver) = QAWebSocket::connect(&CONFIG.common.wsuri)?;
        self.ws_sender = Some(sender);

        let receive_loop = thread::spawn(move || {
            QAWebSocket::receive_loop(receiver, addr)
        });

        let account = CONFIG.common.account.clone();
        let password = CONFIG.common.password.clone();
        let broker = CONFIG.common.broker.clone();
        let login = XReqLogin {
            topic: "login".to_string(),
            aid: "req_login".to_string(),
            bid: broker.clone(),
            user_name: account.clone(),
            password: password.clone(),
        };
        let msg = serde_json::to_string(&login).unwrap();
        self.send_message(OwnedMessage::Text(msg));
        Ok(())
    }


    pub fn ping(&mut self) {
        let msg = format!("ping-{}", CONFIG.common.account).as_bytes().to_vec();
        self.send_message(OwnedMessage::Ping(msg));
    }

    pub fn send_message(&mut self, item: OwnedMessage) {
        if self.ws_sender.is_some() {
            QAWebSocket::send(self.ws_sender.as_mut().unwrap(), item);
        }
    }
}

impl Actor for Scheduler {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.trader.ws_sender = Some(ctx.address().clone());
        self.start_mq_loop(ctx.address().clone());

        if let Err(e) = self.start_ws_receive_loop(ctx.address().clone()) {
            error!("websocket {:?}", e);
            ctx.stop();
            std::process::exit(1);
        }

        ctx.run_interval(Duration::from_secs(CONFIG.common.ping_gap as u64), |act, ctx| {
            act.ping();
        });
    }
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct WSReStart;


impl Handler<WSReStart> for Scheduler {
    type Result = ();
    fn handle(&mut self, _: WSReStart, ctx: &mut Context<Self>) {
        self.ws_sender = None;
        if let Err(e) = self.start_ws_receive_loop(ctx.address().clone()) {
            error!("{:?} , 3s later Try Reconnecting", e);
            ctx.run_later(Duration::from_secs(3), |act, ctx| {
                ctx.address().do_send(WSReStart);
            });
        }
    }
}


#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct OwnedMessageWrap(pub OwnedMessage);


impl Handler<OwnedMessageWrap> for Scheduler {
    type Result = ();
    fn handle(&mut self, msg: OwnedMessageWrap, ctx: &mut Context<Self>) {
        self.send_message(msg.0);
    }
}


#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct SyncMessage(pub String);


impl Handler<SyncMessage> for Scheduler {
    type Result = ();
    fn handle(&mut self, msg: SyncMessage, ctx: &mut Context<Self>) {
        self.trader.parse(msg.0);
    }
}


#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct PongMessage;

impl Handler<PongMessage> for Scheduler {
    type Result = ();
    fn handle(&mut self, msg: PongMessage, ctx: &mut Context<Self>) {
        self.trader.sync();
    }
}



