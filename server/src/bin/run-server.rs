use futures::{Future, Stream};
use tokio::executor::DefaultExecutor;
use tokio::net::TcpListener;
use tower_h2::Server;

use rocksdb_server::server::ServerImpl;
use rocksdb_server::storage::RocksDbStorageLayer;

pub fn main() {
    let store = RocksDbStorageLayer::new("/tmp/foo".to_string()).expect("open rocksdb");
    let kvstore_service = ServerImpl::new(store);
    let mut server = Server::new(
        kvstore_service.into_service(),
        Default::default(),
        DefaultExecutor::current(),
    );

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
