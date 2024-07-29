use demo::{lzw::LZW, new_maps};

use std::{
    error::Error,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

use reqwest::{
    header::{ACCEPT, CONTENT_TYPE},
    Client,
};
use tokio::sync::mpsc::{channel, Sender};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

async fn start_producer_threads(
    tx: Sender<Vec<u8>>,
    number_of_threads: u32,
    articles_per_thread: u32,
) -> Result<Vec<tokio::task::JoinHandle<()>>> {
    let mut join_handles: Vec<tokio::task::JoinHandle<()>> = Vec::default();
    for _ in 0..number_of_threads {
        let join_handle = tokio::spawn({
            let tx = tx.clone();
            async move {
                let base_address = String::from("https://en.wikipedia.org");
                let extension_address = String::from("/wiki/Special:Random");
                let address = format!("{}{}", base_address, extension_address);

                let client = Client::new();
                for _ in 0..articles_per_thread {
                    let response = client
                        .get(&format!("{}/accounts", &address))
                        .header(CONTENT_TYPE, "application/json")
                        .header(ACCEPT, "application/json")
                        .send()
                        .await
                        .expect("Failed to execute request.");

                    match response.status() {
                        reqwest::StatusCode::UNAUTHORIZED => println!("{}", response.status()),
                        reqwest::StatusCode::OK => {
                            // println!("response: {:?}", response);
                            let message = response.bytes().await.unwrap();

                            tx.send(message.to_vec()).await.unwrap();
                        }
                        _ => println!("unhandled response: {}", response.status()),
                    }
                }
                println!("Closing thread");
            }
        });
        join_handles.push(join_handle);
    }

    Ok(join_handles)
}

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = channel(512);

    let threads = match thread::available_parallelism() {
        Ok(t) => {
            println!("Threads used: {:?}", t);
            usize::from(t)
        }
        Err(_) => 0,
    };
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .unwrap();

    let join_handles = start_producer_threads(tx, 5, 20).await?;

    let (compression_map, decompression_map) = new_maps();
    let compression_map = Arc::new(Mutex::new(compression_map));
    // let decompression_map = Arc::new(Mutex::new(decompression_map));

    println!("Waiting for data...");
    // let mut input_message_length = u32::default();
    // let mut output_message_length = u32::default();
    let start = Instant::now();
    let mut message_count = 0;
    while let Some(message) = rx.recv().await {
        message_count += 1;
        println!(
            "pages processed:{:?}, backlog:{:?}",
            message_count,
            rx.len()
        );

        // input_message_length += 8 * message.len() as u32;
        let lzw = LZW;
        let _res = lzw.compress(&message, compression_map.clone());
        // let _res = lzw.decompress(&res, decompression_map.clone());
        // println!("{:?}", res);
        // output_message_length += res.len() as u32;

        // pool.install({
        //     let compression_map = compression_map.clone();
        //     // let decompression_map = decompression_map.clone();
        //     move || {
        //         let lzw = LZW;
        //         let _res = lzw.compress(&message, compression_map);
        //         // let _res = lzw.decompress(&res, decompression_map);
        //         // println!("{:?}", res);
        //     }
        // });

        // pool.spawn({
        //     let compression_map = compression_map.clone();
        //     // let decompression_map = decompression_map.clone();
        //     move || {
        //         let lzw = LZW;
        //         let _res = lzw.compress(&message, compression_map);
        //         // let _res = lzw.decompress(&res, decompression_map);
        //         // println!("{:?}", res);
        //     }
        // });

        // let res = lzw.decompress(res);
        // println!("{:?}", res);
    }
    let dur = start.elapsed();
    println!("{:?}", dur);

    // let ratio = lzw.calculate_compression_ratio(input_message_length, output_message_length);
    // println!("{:?}", ratio);

    // a_thread.await?;

    // x.await.unwrap();

    let mut results = Vec::with_capacity(join_handles.len());
    for handle in join_handles {
        results.push(handle.await.unwrap());
    }

    Ok(())
}
