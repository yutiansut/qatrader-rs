pub mod qamongo;
pub mod qawebsocket;
pub mod qaeventmq;

extern crate ndarray;
use wsq::{connect, WebSocket, Error, ErrorKind};

extern crate chrono;
use chrono::prelude::*;
use ndarray::array;

use std::thread;
use std::borrow::BorrowMut;
extern crate crossbeam_utils;

use crossbeam_channel::bounded;
use crossbeam_utils::thread::scope;
use crate::qawebsocket::{ReqOrder, ReqCancel, ReqTransfer, Peek, ReqChangePassword, ReqQueryBank, ReqQuerySettlement};
use serde_json::value::Value;
extern crate uuid;
use uuid::Uuid;
use serde::Deserializer;

fn main() {

    let (s1, r1) = bounded(0);
    let user_name = "133495".to_string();
    let password = "QCHL1234".to_string();
    {
        thread::spawn(move || {
            let client = qaeventmq::QAEventMQ{
                amqp: "amqp://admin:admin@127.0.0.1:5672/".to_string(),
                exchange: "QAORDER_ROUTER".to_string(),
                model: "direct".to_string(),
                routing_key: user_name.clone(),
            };
            qaeventmq::QAEventMQ::consume(client, s1).unwrap();
        });
    }
    let user_name = "133495".to_string();
    let mut ws = WebSocket::new(move |out| {
        qawebsocket::QAtradeR{
            user_name: user_name.clone(),
            password: password.clone(),
            broker:"simnow".to_string(),
            out:out,
        }}
    ).unwrap();
    let sender = ws.handler.sender().clone();
    thread::spawn(move||{

        loop{
            let data = r1.recv().unwrap();

            println!("receive !!{:?}",data);
            let resx:Value = serde_json::from_str(&data).unwrap();

            let topic = resx["topic"].as_str();
            println!("topic !!{:?}",topic);
            match topic.unwrap() {
                "sendorder" => {
                    let order_id =  uuid::Uuid::new_v4();
                    println!("this is sendorder {:?}",resx);
                    let order = ReqOrder{
                        aid: "insert_order".to_string(),
                        user_id: resx["account_cookie"].as_str().unwrap().parse().unwrap(),
                        order_id: order_id.to_string(),
                        exchange_id: resx["exchange_id"].as_str().unwrap().parse().unwrap(),
                        instrument_id: resx["code"].as_str().unwrap().parse().unwrap(),
                        direction: resx["order_direction"].as_str().unwrap().parse().unwrap(),
                        offset: resx["order_offset"].as_str().unwrap().parse().unwrap(),
                        volume: resx["volume"].as_i64().unwrap(),
                        price_type: "LIMIT".to_string(),
                        limit_price: resx["price"].as_f64().unwrap(),
                        volume_condition: "ANY".to_string(),
                        time_condition: "GFD".to_string()
                    };
                    let b = serde_json::to_string(&order).unwrap();
                    println!("Pretend to send {:?}", b);

                    sender.send(b).unwrap();

                }
                "cancel_order" => {
                    let cancelorder = ReqCancel{
                        aid: "cancel_order".to_string(),
                        user_id: resx["account_cookie"].as_str().unwrap().parse().unwrap(),
                        order_id: resx["order_id"].as_str().unwrap().parse().unwrap()
                    };
                    let b = serde_json::to_string(&cancelorder).unwrap();
                    println!("Pretend to send cancel {:?}", b);

                    sender.send(b).unwrap();
                }
                "transfer" => {
                    let transfermsg = ReqTransfer {
                        aid: "req_transfer".to_string(),
                        bank_id: resx["bank_id"].as_str().unwrap().parse().unwrap(),
                        future_account: resx["account_cookie"].as_str().unwrap().parse().unwrap(),
                        future_password: resx["future_password"].as_str().unwrap().parse().unwrap(),
                        bank_password: resx["bank_password"].as_str().unwrap().parse().unwrap(),
                        currency: "CNY".to_string(),
                        amount: resx["account_cookie"].as_f64().unwrap()
                    };
                    let b = serde_json::to_string(&transfermsg).unwrap();
                    println!("Pretend to send transfer {:?}", b);

                    sender.send(b).unwrap();
                }
                "query_settlement" => {
                    let qsettlementmsg = ReqQuerySettlement{
                        aid: "qry_settlement_info".to_string(),
                        trading_day: resx["trading_day"].as_i64().unwrap()
                    };
                    let b = serde_json::to_string(&qsettlementmsg).unwrap();
                    println!("Pretend to send QuerySettlement {:?}", b);

                    sender.send(b).unwrap();
                }
                "query_bank" => {
                    let qbankmsg = ReqQueryBank{
                        aid: "qry_bankcapital".to_string(),
                        bank_id: resx["bank_id"].as_str().unwrap().parse().unwrap(),
                        future_account: resx["account_cookie"].as_str().unwrap().parse().unwrap(),
                        future_password: resx["future_password"].as_str().unwrap().parse().unwrap(),
                        bank_password: resx["bank_password"].as_str().unwrap().parse().unwrap(),
                        currency: "CNY".to_string()
                    };
                    let b = serde_json::to_string(&qbankmsg).unwrap();
                    println!("Pretend to send QueryBank {:?}", b);

                    sender.send(b).unwrap();
                }
                "change_password" => {
                    let changepwdmsg = ReqChangePassword{
                        aid: "change_password".to_string(),
                        old_password: resx["old_password"].as_str().unwrap().parse().unwrap(),
                        new_password: resx["new_password"].as_str().unwrap().parse().unwrap()
                    };
                    let b = serde_json::to_string(&changepwdmsg).unwrap();
                    println!("Pretend to send ChangePassword {:?}", b);

                    sender.send(b).unwrap();
                }
                "peek" => {
                    let peek = Peek { aid: "peek_message".to_string()};
                    let b = serde_json::to_string(&peek).unwrap();
                    println!("Pretend to send Peek {:?}", b);
                    sender.send(b).unwrap();
                }
                _ => {
                    println!("non receive! {:?}", resx)
                }
            }
//            sender.send(format!("{}", data)).unwrap();

        }

    });
    let parsed ="ws://192.168.2.22:7988";
    ws.connect(parsed.parse().unwrap()).unwrap();
    ws.handler.run(ws.poll.borrow_mut());
    println!("trader");
}




fn test_ndarray() {
    let a3 = array![[[1, 2], [3, 4]],
                    [[5, 6], [7, 8]]];
    println!("{}", a3);
}


pub struct Quote {
    pub datetime: String,
    pub code: String,
    pub open: i32,
    pub high: i32,
    pub low: i32,
    pub close: i32,
}

impl Quote {
    pub fn new(code: &str, datetime: &str, open: i32, high: i32, low: i32, close: i32) -> Quote {
        Quote {
            code: code.to_string(),
            datetime: datetime.to_string(),
            open,
            high,
            low,
            close,
        }
    }

    pub fn update(&mut self) {


        let dt: chrono::DateTime<Utc> = chrono::Utc::now();
        let fixed_dt = dt.with_timezone(&FixedOffset::east(8*3600));
        let data = array![4392, 4435, 4285, 9999999];
        println!("{}", data[0]);
        fixed_dt.to_string();
        "rb2001".to_string();
    }
}






fn test_datetime() {
    let dt: chrono::DateTime<Utc> = chrono::Utc::now();
    let fixed_dt = dt.with_timezone(&FixedOffset::east(8*3600));
    println!("{}", dt);
    println!("{}", fixed_dt);
}




fn test_timeseries() {
    let mut stock = Quote::new("rb2001", "2019", 1, 2, 3, 4);
    println!("Current OPEN: {}", stock.open);
    stock.update();
}