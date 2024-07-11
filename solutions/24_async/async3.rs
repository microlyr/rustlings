// This program demonstrates error handling in asynchronous Rust code. We simulate a function that can fail,
// and handle the error appropriately in an asynchronous context.

use std::time::Duration;
use futures::executor::block_on; // For running the async main function in this educational context.
use tokio; // We use tokio for the async functionality.
use anyhow::{Result, anyhow}; // To simplify error handling

// Asynchronous function that might fail
async fn might_fail(id: u32) -> Result<u32> {
    if id % 2 == 0 {
        tokio::time::sleep(Duration::from_millis(500)).await;
        Ok(id)
    } else {
        tokio::time::sleep(Duration::from_millis(500)).await;
        Err(anyhow!("Failed on odd id"))
    }
}

async fn main_async() {
    let ids = [1, 2, 3, 4, 5];
    let mut results = Vec::new();

    for id in ids {
        let result = might_fail(id).await;
        match result {
            Ok(num) => results.push(Ok(num)),
            Err(e) => results.push(Err(e)),
        }
    }

    // Display results
    for result in results {
        match result {
            Ok(num) => println!("Processed number: {}", num),
            Err(e) => println!("Error occurred: {}", e),
        }
    }
}

fn main() {
    block_on(main_async());
}
