use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use futures::future;
use tower_grpc::{Code, Request, Response, Status};

use crate::proto::server;
use crate::proto::{GetRequest, GetResponse, HelloRequest, HelloResponse, PutRequest, PutResponse};

#[derive(Clone, Debug, Default)]
pub struct InMemoryKvStore {
    data: Arc<Mutex<HashMap<String, String>>>,
}

impl InMemoryKvStore {
    pub fn into_service(self) -> server::KvStoreServer<Self> {
        server::KvStoreServer::new(self)
    }
}

impl server::KvStore for InMemoryKvStore {
    type SayHelloFuture = future::FutureResult<Response<HelloResponse>, Status>;
    type PutFuture = future::FutureResult<Response<PutResponse>, Status>;
    type GetFuture = future::FutureResult<Response<GetResponse>, Status>;

    fn say_hello(&mut self, request: Request<HelloRequest>) -> Self::SayHelloFuture {
        println!("HelloRequest = {:?}", request);

        let response = Response::new(HelloResponse {
            message: "Zomg, it works!".to_string(),
        });

        future::ok(response)
    }

    fn put(&mut self, request: Request<PutRequest>) -> Self::PutFuture {
        println!("PutRequest = {:?}", request);
        self.data.lock().unwrap().insert(
            request.get_ref().key.clone(),
            request.get_ref().value.clone(),
        );
        future::ok(Response::new(PutResponse {}))
    }

    fn get(&mut self, request: Request<GetRequest>) -> Self::GetFuture {
        println!("GetRequest = {:?}", request);

        let key = request.get_ref().key.clone();
        if let Some(value) = self.data.lock().unwrap().get(&key).cloned() {
            future::ok(Response::new(GetResponse { value }))
        } else {
            future::err(Status::new(Code::NotFound, format!("no such key: {}", key)))
        }
    }
}
