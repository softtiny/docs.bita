# About Reqwest Proxy steps
- [ ] request send pass by proxy

```rust
//#features = ["stream", "socks"] #socks suppoert socks5
//By enabling the "socks" feature it is possible to use a socks proxy:
let proxy = reqwest::Proxy::http("socks5://192.168.1.1:9000")?;
let client_build = reqwest::Client::builder();
client_build.proxy(proxy);
```