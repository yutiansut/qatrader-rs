// Port of https://www.rabbitmq.com/tutorials/tutorial-four-python.html. Start this
// example in one shell, then run the routing_emit_log_direct example in another.
use amiquip::{
    Connection, ConsumerMessage, ConsumerOptions, ExchangeDeclareOptions, ExchangeType, FieldTable,
    QueueDeclareOptions, Result,
};


pub struct QAEventMQ{pub amqp : String,
   pub exchange: String,
    pub model: String,
    pub routing_key: String
}

pub trait Subscribe {
    fn subscribe_routing(&mut self) ->  Result<()>;
}

impl Subscribe for QAEventMQ{
    fn subscribe_routing(&mut self) ->Result<()>{
        let mut connection = Connection::insecure_open(&self.amqp)?;
        let channel = connection.open_channel(None)?;
        let exchange = channel.exchange_declare(
            ExchangeType::Direct,
            &self.exchange,
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


        queue.bind(&exchange, self.routing_key.clone(), FieldTable::new())?;
        // Start a consumer. Use no_ack: true so the server doesn't wait for us to ack
        // the messages it sends us.
        let consumer = queue.consume(ConsumerOptions {
            no_ack: true,
            ..ConsumerOptions::default()
        })?;
        println!("Waiting for logs. Press Ctrl-C to exit.");

        for (i, message) in consumer.receiver().iter().enumerate() {
            match message {
                ConsumerMessage::Delivery(delivery) => {
                    let body = String::from_utf8_lossy(&delivery.body);
                    println!("({:>3}) {}:{}", i, delivery.routing_key, body);
                }
                other => {
                    println!("Consumer ended: {:?}", other);
                    break;
                }
            }
        }
        connection.close()
    }
}

pub fn main() -> Result<()> {
    env_logger::init();

    // Open connection.
    let mut connection = Connection::insecure_open("amqp://admin:admin@localhost:5672")?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Declare the fanout exchange we will bind to.
    let exchange = channel.exchange_declare(
        ExchangeType::Direct,
        "tick",
        ExchangeDeclareOptions::default(),
    )?;

    // Declare the exclusive, server-named queue we will use to consume.
    let queue = channel.queue_declare(
        "",
        QueueDeclareOptions {
            exclusive: true,
            ..QueueDeclareOptions::default()
        },
    )?;
    println!("created exclusive queue {}", queue.name());

//    let mut args = std::env::args();
//    if args.len() < 2 {
//        eprintln!(
//            "usage: {} [info] [warning] [error]",
//            args.next()
//                .unwrap_or_else(|| "routing_receive_logs_direct".to_string())
//        );
//        std::process::exit(1);
//    }

//    for severity in args.skip(1) {
//        // Bind to each requested log severity.
//        queue.bind(&exchange, severity, FieldTable::new())?;
//    }
    queue.bind(&exchange, "rb2001".to_string(), FieldTable::new())?;
    // Start a consumer. Use no_ack: true so the server doesn't wait for us to ack
    // the messages it sends us.
    let consumer = queue.consume(ConsumerOptions {
        no_ack: true,
        ..ConsumerOptions::default()
    })?;
    println!("Waiting for logs. Press Ctrl-C to exit.");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                println!("({:>3}) {}:{}", i, delivery.routing_key, body);
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()
}