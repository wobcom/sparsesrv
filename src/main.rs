use async_stream::stream;
use axum::{
    body::{Bytes, StreamBody},
    extract::Path,
    routing::get,
    Router,
};
use futures::stream::Stream;
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};
use std::io;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/:size", get(sparse));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn sparse(Path(size): Path<String>) -> StreamBody<impl Stream<Item = io::Result<Bytes>>> {
    let size: u64 = match size.as_str() {
        "1M" => 1,
        "10M" => 10,
        "100M" => 100,
        "1G" => 1024,
        "10G" => 10240,
        "100G" => 102400,
        _ => 0,
    };
    let mut buffer = vec![0; 1048576 as usize];
    let mut rng = SmallRng::from_entropy();
    let stream = stream! {
        for _i in 0..size {
            rng.fill_bytes(&mut buffer);
            yield Ok(Bytes::from(buffer.clone()));
        }
    };
    StreamBody::new(stream)
}
