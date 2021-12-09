// use std::sync::Mutex;
// use std::thread;
use tokio::task;

#[tokio::main]
async fn main() {
    // let counter = Mutex::new(0);
    let mut handles = Vec::new();

    for i in 1..=10 {
        let handle = task::spawn(async move {
            print(i).await;
        });
        handles.push(handle);
    }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // tokio::join!(handles);

    for handle in handles {
        handle.await.unwrap();
    }

    // println!("Result: {}", *counter.lock().unwrap());
}

async fn print(i: i32) {
    println!("{}", i);
}