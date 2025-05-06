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

## Create Simple Future

- import
    - std::future::Future
        - impl Future for SimpleFuture
            - fn poll(args) -> output
    - std::pin:Pin
        - args[0] code  "mut self: std::pin::Pin<&mut Self>"
    - std::task::Context
        - args[1] code "context: &mut std::task::Context<'_>"
        - context.waker().wake_by_ref()
        - methods: waker(), from_waker(), ext() ...
    - std::task::Poll
        - output code "std::task::Poll<Self::Output>"
        - std::task::Poll::Pending
            - if the future chooses to return Poll::Pending, it must also store the waker somehow and call Waker::wake() when the future should be polled again.
        - std::task::Poll::Reading()
        - methods: map(f:F), is_read(), is_pending(),map_oK(),map_err()
    - std::task::Waker
        - methods: wake(), wake_by_ref(), will_wake(), data(), vtable()


## Simple Future Code

```rust


struct SimpleFuture {
    counter: u32,
}

impl SimpleFuture {
    fn new() -> Self {
        SimpleFuture { counter: 0 }
    }
}

impl std::future::Future for SimpleFuture {
    type Output = String;
    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        self.counter += 1;
        if self.counter > 3 {
            std::task::Poll::Ready(format!("Future resolved after {} polls", self.counter))
        } else {
            println!("Future is pending,poll count: {}", self.counter);
            cx.waker().wake_by_ref();
            std::task::Poll::Pending
        }
    }
}

#[tokio::test]
async fn test_simple_future(){
    println!("run simplefuture");
    let simple = SimpleFuture::new();
    let res = simple.await;
    println!("run simplefuture:{}",res);
}



// Define dummy functions for the vtable operations
unsafe fn dummy_clone(data: *const ()) -> std::task::RawWaker {
    std::task::RawWaker::new(data, &DUMMY_VTABLE)
}

unsafe fn dummy_wake(data: *const ()) {
    // Do nothing
}

unsafe fn dummy_wake_by_ref(data: *const ()) {
    // Do nothing
}

unsafe fn dummy_drop(data: *const ()) {
    // Do nothing
}

// Create the dummy RawWakerVTable
static DUMMY_VTABLE: std::task::RawWakerVTable = std::task::RawWakerVTable::new(
    dummy_clone,
    dummy_wake,
    dummy_wake_by_ref,
    dummy_drop,
);


fn minimal_waker() -> std::task::Waker {
    // Create a dummy raw waker vtable
    let raw_waker_vtable = &std::task::RawWakerVTable::new(
        |_| std::task::RawWaker::new(std::ptr::null(), &DUMMY_VTABLE), // clone
        |_| {},                                               // wake
        |_| {},                                               // wake_by_ref
        |_| {},                                               // drop
    );

    // Create a dummy raw waker
    let raw_waker = std::task::RawWaker::new(std::ptr::null(), raw_waker_vtable);

    // Create a Waker from the raw waker
    unsafe { std::task::Waker::from_raw(raw_waker) }
}

#[test]
fn run_future_simple() {
    let mut future  = SimpleFuture::new();
    let mut future = unsafe { std::pin::Pin::new_unchecked(&mut future) };
    
    let waker = minimal_waker();
    let mut context = std::task::Context::from_waker(&waker);

    loop {
        //future().as_mut().poll() need use std::future::Future;
        match future.as_mut().poll(&mut context){
            std::task::Poll::Ready(result) => {
                println!("Result: {}", result);
                break;
            }
            std::task::Poll::Pending => {
                println!("Future is pending");
                std::thread::sleep(std::time::Duration::from_millis(5000)); //wait 5s 
            }
        }
    }
}
```