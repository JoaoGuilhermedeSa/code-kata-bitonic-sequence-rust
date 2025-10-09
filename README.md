# Challenge
Implement: https://www.geeksforgeeks.org/generate-bitonic-sequence-of-length-n-from-integers-in-a-given-range/

## Solution proposal

### Using Deque - O(n) time and O(n) space
> The idea is to start with r-1, then the maximum value r at the peak, then build the sequence by strategically adding elements from the range [l, r] in a way that maintains the bitonic property, using a deque data structure to efficiently build both the increasing and decreasing portions.

### Step by step approach:

1. Check if n > (r-l)*2+1, if true return [-1] as it's impossible.
2. Start with r-1 as it is the maximum value less than peak value r.
3. While the size of deque is less than n:
    - Add decreasing elements from r down to l at the tail of deque.
    - Add increasing elements from r-2 down to L at the head of deque.
4. Convert the deque into array.


## Do:
-[ok] Implementation 
-[ok] Unit tests
- Performance Test / Benchmarks
-[ok] Proper Documentation
-[ok] Expose Solution via REST API
- Store Results into a Database (Redis with Docker Podman)


## How to install 
1. Install Rust:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Create projetct
```
cargo new hello-rust
```

3. Build/install dependencies
```
cargo build
```

4. Run project
```
cargo run
```

## How to test

1. Get Endpoint with parameters

```
curl --location 'http://localhost:3000/bitonic?n=5&l=3&r=10' \
--header 'Content-Type: application/json'
```


## References
- https://rust-lang.org/pt-BR/learn/get-started/






