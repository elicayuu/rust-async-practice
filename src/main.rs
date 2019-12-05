use std::thread;
use std::time::{Duration, Instant};
use async_std::{task};
use futures::stream::{FuturesUnordered, StreamExt};
use serde::{Serialize, Deserialize};
use rand::distributions::{Distribution, Uniform};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct JsonData {
    user_id: i32,
    id: u32,
    title: String,
    completed: bool,
}

fn main() {
    let start_time = Instant::now();
    let urls = vec![
        "https://jsonplaceholder.typicode.com/todos/1".to_owned(),
        "https://jsonplaceholder.typicode.com/todos/2".to_owned(),
        "https://jsonplaceholder.typicode.com/todos/2".to_owned(),
    ];

    demo_for_multiple_random_sleeps();

    demo_download_urls(&urls);

    println!("Program finished in {} ms", start_time.elapsed().as_millis());
}

fn demo_for_multiple_random_sleeps() {
    let between = Uniform::from(500..10_000);
    let mut rng = rand::thread_rng();
    let mut futures = FuturesUnordered::new();

    for future_number in 0..10 {
        let sleep_millis = between.sample(&mut rng);
        futures.push(sleep_and_print(future_number, sleep_millis));
    }

    task::block_on(async {
        while let Some(_value_returned_from_the_future) = futures.next().await {
        }
    });
}

async fn sleep_and_print(number: u32, sleep_millis: u64) {
    let sleep_duration = Duration::from_millis(sleep_millis);

    task::sleep(sleep_duration).await;
    println!("Future {} slept for {} ms on {:?}", number, sleep_millis, thread::current().id());
}

async fn download_url(url: &str) -> Result<JsonData, surf::Exception> {
    println!("Downloading {} on thread {:?}", url, thread::current().id());

    let result = surf::get(url).recv_json::<JsonData>().await?;
    Ok(result)
}

fn demo_download_urls(urls: &Vec<String>) {
    let mut cf = urls.iter()
        .map(|url| download_url(url))
        .collect::<FuturesUnordered<_>>();

    task::block_on(async {
        while let Some(val) = cf.next().await {
            match val {
                Ok(body) => println!("Get body: {:?}", body),
                Err(e) => println!("Ooooooops!! Got error {:?}", e),
            }
        }
    });
}