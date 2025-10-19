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
use axum::{ extract::{ Query, State }, http::StatusCode, routing::get, Json, Router };
use serde::{ Deserialize, Serialize };
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
async fn bitonic_handler(
    State(mut conn): State<redis::aio::MultiplexedConnection>,
    Query(params): Query<Params>
) -> Result<Json<BitonicResponse>, StatusCode> {

    //Create a unique key for the parameters
    let key = format!("bitonic:{}:{}:{}", params.n, params.l, params.r);

    //Check if the result is already cached
    if let Ok(Some(cached)) = conn.get::<_, Option<String>>(&key).await {
        println!("Result already cached on Redis. Key: {:?}", key);

        if let Ok(result) = serde_json::from_str(&cached) {
            return Ok(
                Json(BitonicResponse {
                    input: params,
                    result,
                })
            );
        }
    }

    //Compute the result if not cached
    let result = get_bitonic_sequence_v2(params.n, params.l, params.r);

    //Store the result in Redis
    if let Ok(result_json) = serde_json::to_string(&result) {
        if let Err(e) = conn.set_ex::<_, _, ()>(&key, result_json, 3600).await {
            eprintln!("Failed to cache result in Redis: {}", e);
            // Not returning an error to the client, just logging it
        } else {
            println!("Saved on Redis. Key: {:?}", key);
        }
    }
    println!("Solution: {:?}", result);

    //Json(BitonicResponse { input: params, result })
    Ok(Json(BitonicResponse { input: params, result }))
}

//list all cached results
async fn cache_list_handler(State(mut conn): State<redis::aio::MultiplexedConnection>) -> Result<
    Json<Vec<(String, Option<String>)>>,
    StatusCode
> {

    let keys: Vec<String> = conn.keys("bitonic:*").await.unwrap_or_default();
    let mut results = Vec::new();

    for key in keys {
        if let Ok(value) = conn.get::<_, Option<String>>(&key).await {
            results.push((key, value));
        }
    }

    Ok(Json(results))
}

#[tokio::main]
async fn main() {
    let redis_url = std::env
        ::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

    let client = redis::Client::open(redis_url).expect("Failed to open Redis client");
    let redis_connection = client
        .get_multiplexed_async_connection().await
        .expect("Failed to get Redis connection");

    let app = Router::new()
        .route("/bitonic", get(bitonic_handler))
        .route("/bitonic/cache", get(cache_list_handler))
        .with_state(redis_connection);

    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    println!("Server running at http://{}/bitonic", addr);
    //
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app).await.unwrap();
}
