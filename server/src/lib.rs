pub mod proto {
    use prost_derive::Message;
    include!(concat!(env!("OUT_DIR"), "/kvstore.rs"));
}

pub mod server;
pub mod storage;
