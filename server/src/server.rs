use futures::future;
use tower_grpc::{Code, Request, Response, Status};

use crate::proto::server;
use crate::proto::{GetRequest, GetResponse, HelloRequest, HelloResponse, PutRequest, PutResponse};
use crate::storage::StorageLayer;

#[derive(Clone, Debug, Default)]
pub struct ServerImpl<T: StorageLayer> {
    storage: T,
}

impl<T: StorageLayer> ServerImpl<T> {
    pub fn new(storage: T) -> Self {
        ServerImpl { storage }
    }
    pub fn into_service(self) -> server::KvStoreServer<Self> {
        server::KvStoreServer::new(self)
    }
}

impl<T: StorageLayer> server::KvStore for ServerImpl<T> {
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
        let result = self.storage.put(
            request.get_ref().key.clone(),
            request.get_ref().value.clone(),
        );
        match result {
            Ok(_) => future::ok(Response::new(PutResponse {})),
            Err(err) => future::err(err.into()),
        }
    }

    fn get(&mut self, request: Request<GetRequest>) -> Self::GetFuture {
        println!("GetRequest = {:?}", request);

        let key = request.get_ref().key.clone();
        let result = self.storage.get(&request.get_ref().key);
        match result {
            Ok(Some(value)) => future::ok(Response::new(GetResponse { value })),
            Ok(None) => future::err(Status::new(Code::NotFound, format!("no such key: {}", key))),
            Err(err) => future::err(err.into()),
        }
    }
}
