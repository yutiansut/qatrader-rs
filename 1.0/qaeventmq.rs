
use amiquip::{
    Connection, ConsumerMessage, ConsumerOptions, ExchangeDeclareOptions, ExchangeType, FieldTable,
    QueueDeclareOptions, Result, Publish
};
use amiquip::Delivery;

use crossbeam_channel::Sender;
pub struct QAEventMQ{pub amqp : String,
   pub exchange: String,
    pub model: String,
    pub routing_key: String
}

impl QAEventMQ{
    pub fn consume(
        eventmq: QAEventMQ,
        ws_event_tx: Sender<String>
    ) -> Result<()>{
        let client = eventmq;
        let mut connection = Connection::insecure_open(&client.amqp)?;
        let channel = connection.open_channel(None)?;
        let exchange = channel.exchange_declare(
            ExchangeType::Direct,
            &client.exchange,
            ExchangeDeclareOptions::default(),
        )?;
        let queue = channel.queue_declare(
            "",
            QueueDeclareOptions {
                exclusive: true,
                ..QueueDeclareOptions::default()
            },
        )?;
        println!("created exclusive queue {}", queue.name());


        queue.bind(&exchange, client.routing_key.clone(), FieldTable::new())?;
        // Start a consumer. Use no_ack: true so the server doesn't wait for us to ack
        // the messages it sends us.
        let consumer = queue.consume(ConsumerOptions {
            no_ack: true,
            ..ConsumerOptions::default()
        })?;

        for (_i, message) in consumer.receiver().iter().enumerate() {
            match message {
                ConsumerMessage::Delivery(delivery) => {
                    let body = String::from_utf8_lossy(&delivery.body);
//                    println!("({:>3}) {}:{}", i, delivery.routing_key, body);body.to_string()
                    QAEventMQ::callback(&client, &delivery, &ws_event_tx);
                }
                other => {
                    println!("Consumer ended: {:?}", other);
                    break;
                }
            }
        }
        connection.close()
    }


    pub fn callback(eventmq: &QAEventMQ,
        message: &Delivery,
        ws_event_tx:  &Sender<String>
    ){
        let msg  = message.body.clone();
        let foo = String::from_utf8(msg).unwrap();
        let data = foo.to_string();

        println!("{:?}",data);

        ws_event_tx.send(data).unwrap();

    }

}


//
//pub trait Subscriber {
//    fn subscribe_routing(&mut self) ->  Result<()>;
//    fn subscribe_fanout(&mut self) ->  Result<()>;
//}
//
//pub trait Publisher {
//    fn publish_routing(&mut self, message:String) -> Result<()>;
//}
//
//
//impl Subscriber for QAEventMQ{
//    fn subscribe_routing(&mut self) ->Result<()>{
//        let mut connection = Connection::insecure_open(&self.amqp)?;
//        let channel = connection.open_channel(None)?;
//        let exchange = channel.exchange_declare(
//            ExchangeType::Direct,
//            &self.exchange,
//            ExchangeDeclareOptions::default(),
//        )?;
//        let queue = channel.queue_declare(
//            "",
//            QueueDeclareOptions {
//                exclusive: true,
//                ..QueueDeclareOptions::default()
//            },
//        )?;
//        println!("created exclusive queue {}", queue.name());
//
//
//        queue.bind(&exchange, self.routing_key.clone(), FieldTable::new())?;
//        // Start a consumer. Use no_ack: true so the server doesn't wait for us to ack
//        // the messages it sends us.
//        let consumer = queue.consume(ConsumerOptions {
//            no_ack: true,
//            ..ConsumerOptions::default()
//        })?;
//
//        for (_i, message) in consumer.receiver().iter().enumerate() {
//            match message {
//                ConsumerMessage::Delivery(delivery) => {
//                    let body = String::from_utf8_lossy(&delivery.body);
////                    println!("({:>3}) {}:{}", i, delivery.routing_key, body);
//                    self.callback(body.to_string());
//                }
//                other => {
//                    println!("Consumer ended: {:?}", other);
//                    break;
//                }
//            }
//        }
//        connection.close()
//    }
//    fn subscribe_fanout(&mut self) ->  Result<()> {
//        // Open connection.
//        let mut connection = Connection::insecure_open(&self.amqp)?;
//
//        // Open a channel - None says let the library choose the channel ID.
//        let channel = connection.open_channel(None)?;
//
//        // Declare the fanout exchange we will bind to.
//        let exchange = channel.exchange_declare(
//            ExchangeType::Fanout,
//            &self.exchange,
//            ExchangeDeclareOptions::default(),
//        )?;
//
//        // Declare the exclusive, server-named queue we will use to consume.
//        let queue = channel.queue_declare(
//            "",
//            QueueDeclareOptions {
//                exclusive: true,
//                ..QueueDeclareOptions::default()
//            },
//        )?;
//        println!("created exclusive queue {}", queue.name());
//        // Bind our queue to the logs exchange.
//        queue.bind(&exchange, "", FieldTable::new())?;
//
//        // Start a consumer. Use no_ack: true so the server doesn't wait for us to ack
//        // the messages it sends us.
//        let consumer = queue.consume(ConsumerOptions {
//            no_ack: true,
//            ..ConsumerOptions::default()
//        })?;
//        println!("Waiting for logs. Press Ctrl-C to exit.");
//
//        for (i, message) in consumer.receiver().iter().enumerate() {
//            match message {
//                ConsumerMessage::Delivery(delivery) => {
//                    let body = String::from_utf8_lossy(&delivery.body);
//
//                    self.callback(body.to_string());
//                    println!("({:>3}) {}", i, body);
//                }
//                other => {
//                    println!("Consumer ended: {:?}", other);
//                    break;
//                }
//            }
//        }
//        connection.close()
//    }
//}
//
//impl Publisher for QAEventMQ{
//    fn publish_routing(&mut self, message:String) -> Result<()> {
//        let mut connection = Connection::insecure_open(&self.amqp)?;
//        let channel = connection.open_channel(None)?;
//        let exchange = channel.exchange_declare(
//            ExchangeType::Direct,
//            &self.exchange,
//            ExchangeDeclareOptions::default(),
//        )?;
//        exchange.publish(Publish::new(message.as_bytes(), self.routing_key.clone()))?;
//        println!("Sent {}:{}", self.routing_key, message);
//        connection.close()
//    }
//}
//
//
//pub fn main() -> Result<()> {
//    env_logger::init();
//
//    // Open connection.
//    let mut connection = Connection::insecure_open("amqp://admin:admin@localhost:5672")?;
//
//    // Open a channel - None says let the library choose the channel ID.
//    let channel = connection.open_channel(None)?;
//
//    // Declare the fanout exchange we will bind to.
//    let exchange = channel.exchange_declare(
//        ExchangeType::Direct,
//        "tick",
//        ExchangeDeclareOptions::default(),
//    )?;
//
//    // Declare the exclusive, server-named queue we will use to consume.
//    let queue = channel.queue_declare(
//        "",
//        QueueDeclareOptions {
//            exclusive: true,
//            ..QueueDeclareOptions::default()
//        },
//    )?;
//    println!("created exclusive queue {}", queue.name());
//
////    let mut args = std::env::args();
////    if args.len() < 2 {
////        eprintln!(
////            "usage: {} [info] [warning] [error]",
////            args.next()
////                .unwrap_or_else(|| "routing_receive_logs_direct".to_string())
////        );
////        std::process::exit(1);
////    }
//
////    for severity in args.skip(1) {
////        // Bind to each requested log severity.
////        queue.bind(&exchange, severity, FieldTable::new())?;
////    }
//    queue.bind(&exchange, "rb2001".to_string(), FieldTable::new())?;
//    // Start a consumer. Use no_ack: true so the server doesn't wait for us to ack
//    // the messages it sends us.
//    let consumer = queue.consume(ConsumerOptions {
//        no_ack: true,
//        ..ConsumerOptions::default()
//    })?;
//    println!("Waiting for logs. Press Ctrl-C to exit.");
//
//    for (i, message) in consumer.receiver().iter().enumerate() {
//        match message {
//            ConsumerMessage::Delivery(delivery) => {
//                let body = String::from_utf8_lossy(&delivery.body);
//                println!("({:>3}) {}:{}", i, delivery.routing_key, body);
//            }
//            other => {
//                println!("Consumer ended: {:?}", other);
//                break;
//            }
//        }
//    }
//
//    connection.close()
//}