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

- Cargo test
    - "--test" tokio_flow loop_find_run 
        - running 1 tests
        - test loop_find_run has been running for over 60 seconds
        - two thread
            - tokio_flow-id.exe xx
            - tokio_flow-id.exe xx
```rust
//tokio_flow.rs
#[test]
fn loop_find_run(){
    let mut count = 0;
    loop {
        count += 1;
        if count > 2{
            count = 0;
        }
    }
}
```