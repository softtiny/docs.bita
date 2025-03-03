# About Reqwest timeout steps
- [ ] request timeout event
- [ ] idle socket
- [ ] hyper server
- [x] tokio task join

```rust
let client_build = reqwest::Client::builder();
let timeout  = Duration::xx;
client_build.timeout(timeout);
client_build.read_timeout(timeout);
client_build.connect_timeout(timeout);
client_build.pool_idle_timeout(Option<timeout>)
```

### ClientBuilder timeout method

- **What it dose:** Sets a total timeout for the entire request, including connection establishment, sending the request, and receiving the response.

- **Scope:** Applies to the full lifecycle of a single HTTP request.
- **Behavior:**  If the request (from start to finish) takes longer than the specified duration, it will fail with a timeout error.

- **Use case:** Use this when you want to limit the overall time a request can take, regardless of which part of the process is slow.

- **Notes:** This is a catch-all timeout. If you set this, it will override any longer individual timeouts (like connect_timeout or read_timeout) because it applies to the whole operation.

### ClientBuilder connect_timeout(Duration)

- **What it dose:** Sets a timeout specifically for establishing the initial connection to the server (e.g., TCP handshake or TLS negotiation).

- **Scope:** Only applies to the connection phase, before the request is sent.

- **Behavior:**  If the client cannot connect to the server within the specified duration, it fails with a connection timeout error.

- **Use case:** Useful when you want to quickly fail if the server is unreachable or slow to accept connections (e.g., network issues or server down).

- **Notes:** This timeout is independent of sending or receiving data—it only governs the initial connection. If timeout is set and shorter than connect_timeout, the overall timeout will take precedence.

### ClientBuilder read_timeout(Duration)

- **What it dose:** Sets a timeout for reading data from the server after the connection is established.

- **Scope:** Applies to the time between receiving chunks of data from the server (e.g., response body).

- **Behavior:**   If no data is received from the server within the specified duration (i.e., the server stops sending data mid-response), it fails with a read timeout error. This is a per-read timeout, not a total response timeout.

- **Use case:** Useful for detecting stalled or unresponsive servers during the response phase, especially for large downloads or streaming responses.

- **Notes:** This is different from timeout because it only applies to the reading phase and resets with each chunk of data received. It’s particularly relevant for slow servers or large payloads where data arrives incrementally.

### ClientBuilder read_timeout(Option&lt;Duration&gt;)

- **What it dose:** Sets a timeout for how long idle connections in the connection pool can remain open before being closed.

- **Scope:** Applies to the connection pool management, not individual requests.

- **Behavior:**   If a connection in the pool (reused from a previous request) remains idle longer than the specified duration, it’s closed. Setting it to None disables the timeout (connections stay open indefinitely until explicitly closed or the pool reaches its max size).

- **Use case:** Useful for managing resource usage in long-running applications, especially when dealing with many servers or when connections should be refreshed periodically.

- **Notes:** This only applies when connection pooling is enabled (default in reqwest for HTTP/1.1 and HTTP/2). It doesn’t affect active requests—just connections sitting idle in the pool.

## Key Differences Summarized

| **Method**            | **Scope**                | **Triggers When**                             | **Typical Use Case**                   |
|-----------------------|--------------------------|-----------------------------------------------|----------------------------------------|
| **timeout**           | Entire request           | Total time from start to finish exceeds limit | General request time limit             |
| **connect_timeout**   | Connection establishment | Initial connection takes too long             | Fast failure on unreachable servers    |
| **read_timeout**      | Reading response data    | No data received within limit after last read | Detecting stalled responses            |
| **pool_idle_timeout** | Idle pooled connections  | Connection sits unused in pool too long       | Resource management in connection pool |


## How They Interact

- **Precedence:** If timeout is shorter than the sum of connect_timeout and time spent reading, it will trigger first. For example, if timeout is 5s and connect_timeout is 3s, a slow connection might hit the overall timeout before the connect timeout.

- **Granularity:** connect_timeout and read_timeout offer finer control over specific phases, while timeout is a blunt instrument for the whole request.

- **Pooling:** pool_idle_timeout is unrelated to request timing—it only affects connection reuse between requests.

