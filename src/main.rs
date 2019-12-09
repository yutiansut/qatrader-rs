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
use crate::qawebsocket::{ReqOrder, ReqCancel, ReqTransfer};
use serde_json::value::Value;
extern crate uuid;
use uuid::Uuid;
use serde::Deserializer;

fn main() {


    let (s1, r1) = bounded(0);

    let user_name = "000002".to_string();

    {

        thread::spawn(move || {
            let mut client = qaeventmq::QAEventMQ{
                amqp: "amqp://admin:admin@192.168.2.118:5672/".to_string(),
                exchange: "QAORDER_ROUTER".to_string(),
                model: "direct".to_string(),
                routing_key: user_name.clone(),
            };
            println!("xxx");
            qaeventmq::QAEventMQ::consume(client, s1).unwrap();
        });
    }
    let user_name = "000002".to_string();

    let mut ws = WebSocket::new(move |out| {
        qawebsocket::QAtradeR{
            user_name: user_name.clone(),
            password: user_name.clone(),
            broker:"QUANTAXIS".to_string(),
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
                        bank_id: "".to_string(),
                        future_account: "".to_string(),
                        future_password: "".to_string(),
                        bank_password: "".to_string(),
                        currency: "".to_string(),
                        amount: 0.0
                    };
                    let b = serde_json::to_string(&transfermsg).unwrap();
                    println!("Pretend to send cancel {:?}", b);

                    sender.send(b).unwrap();
                }

                _ => {
                    println!("non receive! {:?}", resx)
                }
            }
//            sender.send(format!("{}", data)).unwrap();

        }

    });
    let parsed ="ws://101.132.37.31:7988";
    ws.connect(parsed.parse().unwrap()).unwrap();


    ws.handler.run(ws.poll.borrow_mut());
















    println!("trader");


    test_ndarray();
    test_datetime();
    test_timeseries();

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