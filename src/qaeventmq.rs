use lapin::channel::{
    BasicConsumeOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions,BasicProperties, BasicPublishOptions,
};
use lapin::client::ConnectionOptions;
use lapin::types::FieldTable;

use futures::future::Future;
use futures::stream::Stream;


use failure::Error;

use itertools::free::join;
use lapin_futures as lapin;
use tokio;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;





use lapin::client::Client as AMQPClient;

pub fn subscriber() {
    let addr = "127.0.0.1:5672".parse().unwrap();

    Runtime::new()
        .unwrap()
        .block_on_all(
            TcpStream::connect(&addr) // try to initiate a TCP connection
                .map_err(Error::from)
                .and_then(|stream| {
                    // if successful, pass it to AMQP client
                    AMQPClient::connect(stream, ConnectionOptions{
                        username: "admin".to_string(),
                        password: "admin".to_string(),
                        vhost: "/".to_string(),
                        frame_max: 0,
                        heartbeat: 0,
                        properties: Default::default()
                    }).map_err(Error::from)
                })
                .and_then(|(client, heartbeat)| {
                    // do a heartbeat on a dedicated thread to keep us connected
                    tokio::spawn(heartbeat.map_err(|_| ()));
                    // create a channel
                    client.create_channel().map_err(Error::from)
                })
                .and_then(|c| {
                    let channel = c.clone();
                    // generate an queue with a random name which deletes itself after use
                    let queue_options = QueueDeclareOptions {
                        durable: false,
                        exclusive: true,
                        auto_delete: true,
                        nowait: false,
                        passive: false,
                        ticket: 0u16,
                    };
                    channel
                        .exchange_declare(
                            "tick",
                            "direct",
                            ExchangeDeclareOptions::default(),
                            FieldTable::new(),
                        )
                        .map(move |_| channel.clone())
                        // declare a queue
                        .and_then(move |ch| {
                            // declare a queue using specified options
                            // if name is empty it will be generated
                            ch.queue_declare("", queue_options, FieldTable::new())
                                .map(move |queue| (ch.clone(), queue))
                        })
                        .and_then(move |(ch, queue)| {
                            // bind our queue to declared exchange
                            let name = queue.name();
                            ch.queue_bind(
                                &name,
                                "tick",
                                "rb2001",
                                QueueBindOptions::default(),
                                FieldTable::new(),
                            )
                                .map(move |_| (ch.clone(), queue))
                        })
                        .and_then(move |(ch, queue)| {
                            // create a message receiver
                            ch.basic_consume(
                                &queue,
                                "consumer",
                                BasicConsumeOptions::default(),
                                FieldTable::new(),
                            )
                                .map(move |s| (ch.clone(), s))
                        })
                        .and_then(move |(ch, stream)| {
                            // print received messages
                            stream.for_each(move |message| {
                                let text = std::str::from_utf8(&message.data).unwrap();
                                println!("Received: {:?}", text);
                                ch.basic_ack(message.delivery_tag, false)
                            })
                        })
                        .map_err(Error::from)
                }),
        )
        .expect("Failed to create tokio runtime");
}

fn publisher() {
    let addr = "192.168.2.118:5672".parse().unwrap();
    let args: Vec<_> = std::env::args().skip(1).collect();
    let message = match args.len() {
        0 => "hello".to_string(),
        _ => join(args, " "),
    };

    Runtime::new()
        .unwrap()
        .block_on_all(
            TcpStream::connect(&addr) // try to initiate a TCP connection
                .map_err(Error::from)
                .and_then(|stream| {
                    // if successful, pass it to AMQP client
                    AMQPClient::connect(stream, ConnectionOptions{username: "admin".to_string(), password: "admin".to_string(), vhost: "/".to_string(), frame_max: 0, heartbeat: 0, properties: Default::default() }).map_err(Error::from)
                })
                .and_then(|(client, _)| client.create_channel().map_err(Error::from)) // create a channel
                .and_then(move |c| {
                    let channel = c.clone();
                    channel
                        // declare a new exchange
                        .exchange_declare(
                            "test_fanout",
                            "fanout",
                            ExchangeDeclareOptions::default(),
                            FieldTable::new(),
                        )
                        .map(move |_| channel.clone())
                        .and_then(move |ch| {
                            // if successful, send a message
                            ch.basic_publish(
                                "logs",
                                "",
                                message.as_bytes().to_vec(),
                                BasicPublishOptions::default(),
                                BasicProperties::default(),
                            )
                                .map(|_| println!("Sent a message"))
                        })
                        .map_err(Error::from)
                }),
        )
        .expect("Failed to create tokio runtime");
}
