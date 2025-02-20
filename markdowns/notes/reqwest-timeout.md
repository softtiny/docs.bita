# About Reqwest timeout steps
- [ ] request timeout event
- [ ] idle socket

```rust
let client_build = reqwest::Client::builder();
let timeout  = Duration::xx;
client_build.timeout(timeout);
client_build.read_timeout(timeout);
client_build.connect_timeout(timeout);
client_build.pool_idle_timeout(Option<timeout>)
```

