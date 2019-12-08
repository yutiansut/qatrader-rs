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


fn main() {


    let (s1, r1) = bounded(0);

    let user_name = "000001".to_string();

    {

        thread::spawn(move || {
            let mut client = qaeventmq::QAEventMQ{
                amqp: "amqp://admin:admin@127.0.0.1:5672/".to_string(),
                exchange: "QAORDER_ROUTER".to_string(),
                model: "direct".to_string(),
                routing_key: user_name.clone(),
            };
            println!("xxx");
            qaeventmq::QAEventMQ::consume(client, s1).unwrap();
        });
    }
    let user_name = "000001".to_string();



//
//    connect("ws://101.132.37.31:7988", |out| {
//        qawebsocket::QAtradeR{
//            user_name: user_name.clone(),
//            password: user_name.clone(),
//            broker:"QUANTAXIS".to_string(),
//            out:out,
//            recv: r1.clone()
//        }});
////
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
            sender.send(format!("{}", data));

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