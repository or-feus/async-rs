use std::sync::Arc;
use tokio::sync::Mutex;
use tokio;


#[tokio::main]
async fn main() {


    let data = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let data = Arc::clone(&data);

        for _ in 0..10 {
            let data_clone = Arc::clone(&data);
            let handle = tokio::spawn(async move {
                let mut data = data_clone.lock().await;
                *data += 1;
            });
            handles.push(handle);
        }
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("Result: {}", *data.lock().await)

}
