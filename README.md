# WAGMI Challenge 001 – WAGMI 9000

This repository contains my solution to the **WAGMI Challenge 001**, focused on building a simple and efficient **HTTP load tester** using Rust.

## Overview

The goal of the challenge was to create a tool that simulates HTTP GET requests to a target URL and measures basic performance metrics such as:

- Request throughput
- Response latency
- Success/failure counts

My solution is built with Rust with an emphasis on concurrency and speed.

## File Structure

- `main.rs` – CLI entrypoint that handles argument parsing and launches the load test.
- `load_test.rs` – Script for stress testing main.rs with concurrent requests.

Here’s an updated **Usage** section for your README that includes testing with `curl` against your deployed endpoint:

---

## Usage

This project implements two main endpoints at:

```
POST https://wagmi-challenge-1-production.up.railway.app/wagmi
```

---

### 🧪 1. **Ping Test Request**

Send a `POST` with an empty JSON object `{}` to verify the server is online.

#### ✅ Example:

```bash
curl -X POST https://wagmi-challenge-1-production.up.railway.app/wagmi \
  -H "Content-Type: application/json" \
  -d '{}'
```

#### 🔁 Response:

```json
{
  "message": "wagmi",
  "timestamp": "2025-05-18T12:00:00Z",
  "lang": "Rust"
}
```

---

### ➕ 2. **Addition Request**

Send two integers `a` and `b`. Returns their sum **only if** the result ≤ 100.

#### ✅ Example:

```bash
curl -X POST https://wagmi-challenge-1-production.up.railway.app/wagmi \
  -H "Content-Type: application/json" \
  -d '{"a": 40, "b": 55}'
```

#### 🔁 Response:

```json
{
  "result": 95,
  "a": 40,
  "b": 55,
  "status": "success"
}
```

#### ❌ Invalid Input (e.g. missing fields or result > 100):

```json
{
  "error": "Invalid input"
}
```

## Features

- Asynchronous requests with `tokio` and `reqwest`
- Configurable number of concurrent requests
- Total requests & concurrency levels via CLI args
- Aggregated stats: average latency, success/failure counts
- Graceful handling of errors and timeouts

## Dependencies

This project uses the following crates:

- [`actix-web`](https://docs.rs/actix-web) – Web framework for building fast and secure web services.
- [`sonic-rs`](https://docs.rs/sonic-rs) – Ultra-fast JSON parser for Rust (based on [simdjson](https://github.com/simdjson/simdjson)).
- [`chrono`](https://docs.rs/chrono) – Date and time handling.
- [`tokio`](https://docs.rs/tokio) – Asynchronous runtime with support for multi-threaded execution and timing utilities.
- [`serde`](https://docs.rs/serde) and [`serde_json`](https://docs.rs/serde_json) – Serialization and deserialization of data structures and JSON.
- [`once_cell`](https://docs.rs/once_cell) – Efficient global single-assignment values.
- [`futures`](https://docs.rs/futures) – Abstractions for asynchronous programming.
- [`reqwest`](https://docs.rs/reqwest) – HTTP client with JSON support for making network requests.

## Learnings

* Leveraged async/await and Rust's type system to safely manage high concurrency.
* Practiced error handling and graceful exits for robust CLI tools.
* Learned efficient data aggregation under concurrent workloads.

## License

MIT License

---

🛠️ Built with love and Rust for the WAGMI Challenge 001. :)
