use websocket::{OwnedMessage, Message, WebSocketError, ClientBuilder};
use websocket::receiver::Reader;
use websocket::sender::Writer;
use std::net::TcpStream;
use serde_json::Value;
use log::{warn, error, debug, info};
use chrono::Local;
use crate::msg::{parse_message, RtnData};
use crate::xmsg::{XPeek, XReqLogin};
use crate::config::CONFIG;
use crate::scheduler::{Scheduler, OwnedMessageWrap, SyncMessage, WSReStart, PongMessage};
use std::str::from_utf8;
use actix::Addr;

pub struct QAWebSocket {
    pub sender: Writer<TcpStream>,
    pub receiver: Reader<TcpStream>,
}

impl QAWebSocket {
    pub fn connect(wsuri: &str) -> Result<(Writer<TcpStream>, Reader<TcpStream>), WebSocketError> {
        let client = ClientBuilder::new(wsuri)
            .unwrap()
            .add_protocol("rust-websocket")
            .connect_insecure()?;

        let (receiver, sender) = client.split().unwrap();
        Ok((sender, receiver))
    }


    pub fn send(sender: &mut Writer<TcpStream>, message: OwnedMessage) {
        match message {
            OwnedMessage::Ping(_) => {
                let _ = sender.send_message(&message);
            }
            OwnedMessage::Text(str) => {
                match parse_message(str) {
                    Some(data) => {
                        let x = OwnedMessage::Text(data);
                        if let Err(e) = sender.send_message(&x) {
                            error!("[Send WebSocket] {:?}", e);
                        }
                    }
                    None => {
                        error!("[Send Cancel],消息格式错误/未知消息");
                    }
                }
            }
            _ => {
                error!("内部错误")
            }
        };
    }

    /// 接收websokcet 消息
    pub fn receive_loop(mut receiver: Reader<TcpStream>, ws_send: Addr<Scheduler>) {
        let mut error_count = 1;
        for message in receiver.incoming_messages() {
            {
                // Peek
                let peek = r#"{"topic":"peek","aid":"peek_message"}"#.to_string();
                ws_send.do_send(OwnedMessageWrap(OwnedMessage::Text(peek)));
            }

            match message {
                Ok(om) => {
                    match om {
                        OwnedMessage::Close(_) => {
                            break;
                        }
                        OwnedMessage::Text(msg) => {
                            info!("Receive WebSocket Data: {:?}", msg);
                            ws_send.do_send(SyncMessage(msg));
                        }
                        OwnedMessage::Pong(msg) => {
                            let _ = from_utf8(&msg).unwrap().to_string();
                            ws_send.do_send(PongMessage);
                        }
                        _ => ()
                    }
                }
                Err(e) => {
                    error!("Receive WebSocket Error {:?}", error_count);
                    if error_count >= 10 {
                        ws_send.do_send(WSReStart);
                        break;
                    }
                    error_count += 1;
                }
            };
        }
        info!("receive_loop exit");
    }
}


