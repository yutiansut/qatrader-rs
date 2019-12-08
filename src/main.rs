pub mod qamongo;
pub mod qawebsocket;
pub mod qaeventmq;

extern crate ndarray;
use ws::{connect};
extern crate chrono;
use chrono::prelude::*;
use ndarray::array;
use std::sync::mpsc::channel;
use std::thread;



fn main() {


    let (ws_event_tx, ws_event_rx) = channel();
    let user_name = "000001".to_string();

    {
        let event_tx = ws_event_tx.clone();
        thread::spawn(move || {
            let mut client = qaeventmq::QAEventMQ{
                amqp: "amqp://admin:admin@127.0.0.1:5672/".to_string(),
                exchange: "QAORDER_ROUTER".to_string(),
                model: "direct".to_string(),
                routing_key: user_name.clone(),
            };
            println!("xxx");
            qaeventmq::QAEventMQ::consume(client, event_tx).unwrap();
        });
    }
    let user_name = "000001".to_string();
    thread::spawn(|| {
        connect("ws://www.yutiansut.com:7788", move |out| {
            qawebsocket::QAtradeR{
                user_name: user_name.clone(),
                password: user_name.clone(),
                broker:"QUANTAXIS".to_string(),
                out:out
            }

        }).unwrap()
    });
    qawebsocket::QAtradeR::start(ws_event_rx);
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