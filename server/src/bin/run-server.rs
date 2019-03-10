extern crate bytes;
extern crate futures;
#[macro_use]
extern crate prost_derive;
extern crate tokio;
extern crate tower_grpc;
extern crate tower_h2;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/kvstore.rs"));
}

use proto::{
    server, GetRequest, GetResponse, HelloRequest, HelloResponse, PutRequest, PutResponse,
};

use futures::{future, Future, Stream};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::executor::DefaultExecutor;
use tokio::net::TcpListener;
use tower_grpc::{Code, Request, Response, Status};
use tower_h2::Server;

#[derive(Clone, Debug, Default)]
struct KvStoreServerImpl {
    data: Arc<Mutex<HashMap<String, String>>>,
}

impl server::KvStore for KvStoreServerImpl {
    type SayHelloFuture = future::FutureResult<Response<HelloResponse>, Status>;
    type PutFuture = future::FutureResult<Response<PutResponse>, Status>;
    type GetFuture = future::FutureResult<Response<GetResponse>, Status>;

    fn say_hello(&mut self, request: Request<HelloRequest>) -> Self::SayHelloFuture {
        println!("REQUEST = {:?}", request);

        let response = Response::new(HelloResponse {
            message: "Zomg, it works!".to_string(),
        });

        future::ok(response)
    }

    fn put(&mut self, request: Request<PutRequest>) -> Self::PutFuture {
        println!("REQUEST = {:?}", request);
        self.data.lock().unwrap().insert(
            request.get_ref().key.clone(),
            request.get_ref().value.clone(),
        );
        future::ok(Response::new(PutResponse {}))
    }

    fn get(&mut self, request: Request<GetRequest>) -> Self::GetFuture {
        println!("REQUEST = {:?}", request);

        let key = request.get_ref().key.clone();
        if let Some(value) = self.data.lock().unwrap().get(&key).cloned() {
            future::ok(Response::new(GetResponse { value }))
        } else {
            future::err(Status::new(Code::NotFound, format!("no such key: {}", key)))
        }
    }
}

pub fn main() {
    let new_service = server::KvStoreServer::new(KvStoreServerImpl::default());

    let mut server = Server::new(new_service, Default::default(), DefaultExecutor::current());

    let addr = "[::1]:50051".parse().unwrap();
    let bind = TcpListener::bind(&addr).expect("bind");

    let serve = bind
        .incoming()
        .for_each(move |sock| {
            if let Err(e) = sock.set_nodelay(true) {
                return Err(e);
            }

            let serve = server.serve(sock);
            tokio::spawn(serve.map_err(|e| eprintln!("h2 error: {:?}", e)));

            Ok(())
        })
        .map_err(|e| eprintln!("accept error: {}", e));

    println!("listening on {}...", addr);
    tokio::run(serve)
}
