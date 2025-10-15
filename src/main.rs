//! # Bitonic Sequence REST API Server
//!
//! This application provides a REST API for generating bitonic sequences and caching
//! results using Redis. It's built with Axum web framework and Tokio async runtime.
//!
//! ## Endpoints
//!
//! - `GET /bitonic?n=<length>&l=<start>&r=<end>` - Generate bitonic sequence
//! - `GET /bitonic/cache` - Retrieve all cached results
//!
//! ## Usage
//!
//! Start the server with `cargo run` and access endpoints at `http://localhost:3000`
//!
use axum::{extract::Query, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use redis::AsyncCommands;

// lib
use code_kata_bitonic_sequence_rust::get_bitonic_sequence_v2;

/// Request parameters for the bitonic sequence generation endpoint.
///
/// # Fields
///
/// * `n` - The desired length of the bitonic sequence
/// * `l` - The start of the range (inclusive)
/// * `r` - The end of the range (inclusive)
#[derive(Deserialize, Serialize)]
struct Params {
    n: u32,
    l: i32,
    r: i32,
}

/// Response structure for the bitonic sequence API.
///
/// Contains the generated sequence and metadata about the request.
#[derive(Serialize)]
struct BitonicResponse {
    /// Input params, eg. n=5, l=3, r=10
    input: Params,
    /// The generated bitonic sequence
    result: Vec<i32>,
}

//return function
async fn bitonic_handler(Query(params): Query<Params>) -> impl IntoResponse {
    
    //TODO Maybe use proxy, and create a singleton connection?

    //Check Redis connection
    let mut conn = redis::Client::open("redis://127.0.0.1:6379/")
        .unwrap()
        .get_multiplexed_async_connection()
        .await
        .unwrap();
    
    //Create a unique key for the parameters
    //TODO:  create a TTL?
    let key = format!("bitonic:{}:{}:{}", params.n, params.l, params.r);
    
    //Check if the result is already cached
    let result: redis::RedisResult<Option<String>> = conn.get(&key).await;

    if let Ok(Some(cached)) = result {
        println!("Result already cached on Redis. Key: {:?}", key);
        let result: Vec<i32> = serde_json::from_str(&cached).unwrap();
        return Json(BitonicResponse { input: params, result });
    }

    //Compute the result if not cached
    let result = get_bitonic_sequence_v2(params.n, params.l, params.r);
    //Store the result in Redis
    let _: () = conn
        .set(&key, serde_json::to_string(&result).unwrap())
        .await
        .unwrap();

    println!("Saved on Redis. Key: {:?}", key);
    println!("Solution: {:?}", result);

    Json(BitonicResponse { input: params, result })
}

//list all cached results
async fn cache_list_handler() -> impl IntoResponse {
    let mut conn = redis::Client::open("redis://127.0.0.1:6379/")
        .unwrap()
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let keys: Vec<String> = conn.keys("bitonic:*").await.unwrap_or_default();

    let mut results = Vec::new();
    
    for key in keys {
        let result: redis::RedisResult<Option<String>> = conn.get(&key).await;

        if let Ok(value) = result {
            results.push((key, value));
        }
    }

    Json(results)
}

#[tokio::main]
async fn main() {
    
    let app = Router::new()
    .route("/bitonic", get(bitonic_handler))
    .route("/bitonic/cache", get(cache_list_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}/bitonic", addr);
    //
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
    .await
    .unwrap();
}
