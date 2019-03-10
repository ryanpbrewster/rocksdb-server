extern crate bytes;
extern crate env_logger;
extern crate futures;
#[macro_use]
extern crate log;
extern crate prost;
#[macro_use]
extern crate prost_derive;
extern crate tokio;
extern crate tower_h2;
extern crate tower_grpc;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/kvstore.rs"));
}

use proto::{server, HelloRequest, HelloResponse, PutRequest, PutResponse, GetRequest, GetResponse};

use futures::{future, Future, Stream};
use tokio::executor::DefaultExecutor;
use tokio::net::TcpListener;
use tower_h2::Server;
use tower_grpc::{Request, Response, Status, Code};

#[derive(Clone, Debug)]
struct KvStoreServerImpl;

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
        future::err(Status::new(Code::Unimplemented, "Put unimplemented"))
    }

    fn get(&mut self, request: Request<GetRequest>) -> Self::GetFuture {
        println!("REQUEST = {:?}", request);
        future::err(Status::new(Code::Unimplemented, "Get unimplemented"))
    }
}

pub fn main() {
    let _ = ::env_logger::init();

    let new_service = server::KvStoreServer::new(KvStoreServerImpl);

    let h2_settings = Default::default();
    let mut h2 = Server::new(new_service, h2_settings, DefaultExecutor::current());

    let addr = "[::1]:50051".parse().unwrap();
    let bind = TcpListener::bind(&addr).expect("bind");

    let serve = bind.incoming()
        .for_each(move |sock| {
            if let Err(e) = sock.set_nodelay(true) {
                return Err(e);
            }

            let serve = h2.serve(sock);
            tokio::spawn(serve.map_err(|e| error!("h2 error: {:?}", e)));

            Ok(())
        })
        .map_err(|e| eprintln!("accept error: {}", e));

    println!("listening on {}...", addr);
    tokio::run(serve)
}
