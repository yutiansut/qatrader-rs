pub mod qamongo;
pub mod eventmq;
pub mod qawebsockets;
// use tokio::net::TcpListener;
// use tokio::prelude::*;



// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {

//     let mut listener = TcpListener::bind("127.0.0.1:8082").await?;
//     println!("created stream");
//     loop {
//         let (mut socket, _) = listener.accept().await?;

//         tokio::spawn(async move {
//             let mut buf = [0; 1024];

//             // In a loop, read data from the socket and write the data back.
//             loop {
//                 let n = match socket.read(&mut buf).await {
//                     // socket closed
//                     Ok(n) if n == 0 => return,
//                     Ok(n) => {
//                         println!("{}", n);
//                         n
//                     },
//                     Err(e) => {
//                         eprintln!("failed to read from socket; err = {:?}", e);
//                         return;
//                     }
//                 };

//                 // Write the data back
//                 // let result = stream.write(b"hello world\n").await;
//                 if let Err(e) = socket.write_all(&buf[0..n]).await {
//                     eprintln!("failed to write to socket; err = {:?}", e);
//                     return;
//                 }
//             }
//         });
//     }
// }
extern crate ndarray;

extern crate chrono;
use chrono::prelude::*;
use ndarray::array;
// use ndarray::{ArrayD, ArrayViewD, ArrayViewMutD};



fn main() {
   qamongo::action::query_account("192.168.2.24".to_string(), "288870".to_string());
   //eventmq::mqbase::connect_mq("192.168.2.24".to_string(), "test".to_string(), "test".to_string(), "thisisQUANTAXIS".to_string());
    // qawebsockets::websocketclient::wsmain(
    //     "ws://101.132.37.31:7988".to_string());
    test_ndarray();
    test_datetime();
    test_timeseries();
    // test_pyo3();
    //rust_ext();
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


// fn test_pyo3() -> Result<(), ()> {
//     let gil = Python::acquire_gil();
//     test_pyo3_(gil.python()).map_err(|e| {
//         eprintln!("error! :{:?}", e);
//         // we can't display python error type via ::std::fmt::Display
//         // so print error here manually
//         e.print_and_set_sys_last_vars(gil.python());
//     })
// }

// fn test_pyo3_<'py>(py: Python<'py>) -> PyResult<()> {
//     let np = py.import("numpy")?;
//     let dict = PyDict::new(py);
//     dict.set_item("np", np)?;
//     let pyarray: &PyArray1<i32> = py
//         .eval("np.absolute(np.array([-1, -2, -3], dtype='int32'))", Some(&dict), None)?
//         .extract()?;
//     let slice = pyarray.as_slice()?;
//     assert_eq!(slice, &[1, 2, 3]);
//     Ok(())
// }




fn test_timeseries() {
    let mut stock = Quote::new("rb2001", "2019", 1, 2, 3, 4);
    println!("Current OPEN: {}", stock.open);
    stock.update();
}