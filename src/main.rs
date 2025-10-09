use axum::{extract::Query, response::Json, routing::get, Router};
use serde::Deserialize;
use std::net::SocketAddr;

use redis::AsyncCommands;

// lib
use code_kata_bitonic_sequence_rust::get_bitonic_sequence;

#[derive(Deserialize)]
struct Params {
    n: usize,
    l: i32,
    r: i32,
}

//Store the response?

//return function
async fn bitonic_handler(Query(params): Query<Params>) -> Json<Vec<i32>> {
    let result = get_bitonic_sequence(params.n, params.l, params.r);
    println!("Solution: {:?}", result);

    //Save into the redis?


    Json(result)
}

#[tokio::main]
async fn main() {
    // bitonic_array(n, l, r);
    //let solution = get_bitonic_sequence(5, 3, 10);
    
    let app = Router::new().route("/bitonic", get(bitonic_handler));


    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}/bitonic", addr);
    //
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
    .await
    .unwrap();
}
