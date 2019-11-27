pub mod qamongo;
pub mod eventmq;


fn main() {
    qamongo::query::query_account("192.168.2.24".to_string(), "288870".to_string());
    eventmq::mqbase::connect_mq("192.168.2.24".to_string(), "test".to_string(), "test".to_string(), "thisisQUANTAXIS".to_string());
}
