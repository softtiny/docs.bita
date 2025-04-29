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


- Cargo test
    - "--test" tokio_flow loop_find_async 
        - running 1 tests
        - test loop_find_async has been running for over 60 seconds
        - two thread
            - tokio_flow-id.exe xx
            - tokio_flow-id.exe xx
```rust
//tokio_flow.rs
#[tokio::test]
async fn loop_find_async(){
    let mut count = 0;
    loop {
        count += 1;
        if count > 2{
            count = 0;
        }
    }
}
```

## Two === 2
```rust
#[tokio::test]
async fn loop_async_with_sleep(){
    tokio::time::sleep(std::time::Duration::from_secs(13203));
    let mut count = 0;
    loop {
        count += 1;
        if count > 2{
            count = 0;
        }
    }
}

//also 2
#[tokio::test]
async fn loop_async_sleep_await(){
    tokio::time::sleep(std::time::Duration::from_secs(13203)).await;
}


// also 2 
// also 2 2 2
// also 2 2 2 2 2
#[tokio::test]
async fn loop_asyncs_join(){
    //method cannot be called on `Pin<&mut MaybeDone<()>>` due to unsatisfied trait bounds
    //join!(tokio::time::sleep(std::time::Duration::from_secs(13203)).await);
    join!(
        tokio::time::sleep(std::time::Duration::from_secs(13203)),
        tokio::time::sleep(std::time::Duration::from_secs(13203)),
        tokio::time::sleep(std::time::Duration::from_secs(13203)),
        tokio::time::sleep(std::time::Duration::from_secs(13203)),
        tokio::time::sleep(std::time::Duration::from_secs(13203))
    );
}
```



- Cargo test
    - "--test" tokio_flow build_thread_w2 
        - running 1 tests
        - test build_thread_w2 has been running for over 60 seconds
        - 4 thread
            - tokio_flow-id.exe xx 1
                - runtime test 
            - tokio_flow-id.exe xx 2
                - test filter build_thread_w2
            - tokio_flow-id.exe xx 3
            - tokio_flow-id.exe xx 4
                - runtime::Builder::new_multi_thread().worker_threads(2).enable_all()

```rust
//tokio_flow.rs
#[test]
fn build_thread_w2(){
    runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let mut count = 0;
            loop {
                count += 1;
                if count > 2{
                    count = 0;
                }
            }

        })
}
```