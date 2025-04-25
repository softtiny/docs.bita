# Tokio

```rust
#[tokio::test]
async fn test_run_ok() {

}
#[tokio::main]
async fn run_ok() {

}
```

## runtime thread

```rust
use tokio::runtime;
runtime::Builder::new_multi_thread()
    .worker_threads(1)
    .enable_all() 
    .build()
    .unwrap()
    .block_on(async move {})
//A Tokio 1.x context was found, but timers are disabled. Call `enable_time` on the runtime builder to enable timers
//.enable_all
```