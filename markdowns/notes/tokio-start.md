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
    .worker_threads(4)
    .enable_all()
    .build()
    .unwrap()
    .block_on(async move {})
```