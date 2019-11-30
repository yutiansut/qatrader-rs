pub mod qamongo;
pub mod eventmq;

use tokio::net::TcpListener;
use tokio::prelude::*;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut listener = TcpListener::bind("127.0.0.1:8082").await?;
    println!("created stream");
    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => {
                        println!("{}", n);
                        n
                    },
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                // let result = stream.write(b"hello world\n").await;
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
//fn main() {
//    qamongo::query::query_account("192.168.2.24".to_string(), "288870".to_string());
//    //eventmq::mqbase::connect_mq("192.168.2.24".to_string(), "test".to_string(), "test".to_string(), "thisisQUANTAXIS".to_string());
//
//}
