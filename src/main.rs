use demo::lzw::{self, LZW};
use tokio::task;
// use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelBridge, ParallelIterator};

use std::error::Error;
use std::sync::Mutex;
use std::{collections::HashMap, sync::Arc};

use std::thread::available_parallelism;

use rayon::spawn;
use reqwest::{
    header::{ACCEPT, CONTENT_TYPE},
    Client,
};
use tokio::sync::mpsc::{channel, Sender};

use std::time::Instant;

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
                        _ => todo!("unhandled response: {}", response.status()),
                    }
                }
                println!("Closing thread");
            }
        });
        join_handles.push(join_handle);
    }

    Ok(join_handles)
}

fn new_maps() -> (HashMap<String, u32>, HashMap<u32, String>) {
    let mut compression_map = HashMap::new();
    let mut decompression_map = HashMap::new();

    (0..255).for_each(|i| {
        compression_map.insert((char::from(i)).to_string(), i as u32);
        decompression_map.insert(i as u32, (char::from(i)).to_string());
    });

    (compression_map, decompression_map)
}

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = channel(64);

    let threads = match available_parallelism() {
        Ok(t) => usize::from(t),
        Err(_) => 0,
    };
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .unwrap();

    start_producer_threads(tx, 10, 10).await?;

    let (compression_map, decompression_map) = new_maps();
    let compression_map = Arc::new(Mutex::new(compression_map));
    let _x_dm = Arc::new(Mutex::new(decompression_map));

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
        let mut compression_map = compression_map.lock().unwrap();
        let mut lzw = LZW::default();
        lzw.compress(message, &mut compression_map);
        // println!("{:?}", res);
        // output_message_length += res.len() as u32;
        // fs::write("./output", res).await?;
        // fs::write(path, contents)

        // pool.install({
        //     let compression_map = compression_map.clone();
        //     move || {
        //         let mut lzw = LZW::default();
        //         let mut compression_map = compression_map.lock().unwrap();
        //         lzw.compress(message, &mut compression_map);
        //     }
        // });

        // pool.spawn({
        //     let compression_map = compression_map.clone();
        //     move || {
        //         let mut lzw = LZW::default();
        //         let mut compression_map = compression_map.lock().unwrap();
        //         lzw.compress(message, &mut compression_map);
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

    Ok(())
}
