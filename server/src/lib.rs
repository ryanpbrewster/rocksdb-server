#![feature(range_contains)]

pub mod proto {
    use prost_derive::Message;
    include!(concat!(env!("OUT_DIR"), "/kvstore.rs"));
}

pub type KvResult<T> = std::result::Result<T, KvError>;

#[derive(Debug)]
pub enum KvError {
    Unknown(Box<dyn std::error::Error>),
}

impl From<KvError> for tower_grpc::Status {
    fn from(err: KvError) -> Self {
        match err {
            KvError::Unknown(underlying) => {
                tower_grpc::Status::new(tower_grpc::Code::Unknown, format!("{:?}", underlying))
            }
        }
    }
}

pub mod server;
pub mod storage;
