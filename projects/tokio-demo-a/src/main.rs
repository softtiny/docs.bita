use tokio::{
    runtime,
    task,
};
use std::time::Duration;

async fn task_a() {
    println!("run task a!!");
    let mut start = 0;
    let end = 100;
    while start <= end {
        tokio::time::sleep(Duration::from_secs(1)).await;
        start += 1;
    }
}

async fn main_run() {
    task_a().await;
}

fn main() {
    runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                main_run().await;
            })
}

// #[tokio::main]
// async fn main() {
//     println!("tokio main run ok!");
// }
