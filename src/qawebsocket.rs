use websocket::{OwnedMessage, Message,WebSocketError};
use websocket::receiver::Reader;
use websocket::sender::Writer;
use std::net::TcpStream;
use std::sync::mpsc::{Sender, Receiver};
use serde_json::Value;
use log::{warn, error, debug, info};
use crate::msg::{parse_message, RtnData};
use crate::xmsg::{XPeek, XReqLogin};
use crate::config::CONFIG;

pub struct QAWebSocket;

impl QAWebSocket {
    pub fn on_open(ws_send: Sender<OwnedMessage>) {
        let user_name = CONFIG.common.account_name.clone();
        let password = CONFIG.common.password.clone();
        let broker = CONFIG.common.broker.clone();
        let login = XReqLogin {
            topic: "login".to_string(),
            aid: "req_login".to_string(),
            bid: broker.clone(),
            user_name: user_name.clone(),
            password: password.clone(),
        };
        let msg = serde_json::to_string(&login).unwrap();
        if let Err(e) = ws_send.send(OwnedMessage::Text(msg)) {
            error!("Login Error: {:?}", e);
        }
    }


    /// 从本地chanel接收消息-->往websocket 发送消息
    pub fn send_loop(mut sender: Writer<TcpStream>, rx: Receiver<OwnedMessage>) {
        loop {
            // Send loop
            let message = match rx.recv() {
                Ok(m) => m,
                Err(e) => {
                    error!("Receive Channel Error: {:?}", e);
                    continue;
                }
            };
            match message {
                OwnedMessage::Close(_) => {
                    let _ = sender.send_message(&message);
                    // If it's a close message, just send it and then return.
                    return;
                }
                OwnedMessage::Text(str) => {
                    match parse_message(str) {
                        Some(data) => {
                            // info!("Send {:?}",data);
                            let x = OwnedMessage::Text(data);
                            if let Err(e) = sender.send_message(&x) {
                                error!("Send Error: {:?}", e);
                            }
                        }
                        None => {
                            warn!("Send Cancel,消息格式错误/未知消息");
                        }
                    }
                }
                _ => ()
            };
        }
    }

    /// 接收websokcet 消息
    pub fn receive_loop(mut receiver: Reader<TcpStream>, ws_send: Sender<OwnedMessage>, db_send: Sender<String>) {
        for message in receiver.incoming_messages() {
            {
                // Peek
                let peek = XPeek { topic: "peek".to_string(), aid: "peek_message".to_string() };
                let b = serde_json::to_string(&peek).unwrap();
                ws_send.send(OwnedMessage::Text(b));
            }

            match message {
                Ok(om) => {
                    match om {
                        OwnedMessage::Text(msg) => {
                            debug!("Receive WebSocket Data: {:?}", msg);
                            db_send.send(msg);
                        }
                        _ => ()
                    }
                }
                Err(e) => {
                    error!(" Receive WebSocket Error: {:?}", e);
                    match e{
                        WebSocketError::NoDataAvailable=>{}
                        WebSocketError::IoError(e)=>{
                            // 重连机制
                        }
                        _ => {}
                    }
                    continue;
                }
            };
        }
    }
}


