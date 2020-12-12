use crate::DownloadQueue;
use reqwest::header::{HeaderMap, HeaderValue};
use tokio::time::{Duration, Instant};
use tokio_compat_02::FutureExt;

pub mod types;

pub async fn search_crawl(base: &str, tags: &str, queue: DownloadQueue) {
    let mut header = HeaderMap::new();
    header.insert("Accept", HeaderValue::from_static("application/json"));
    let client = reqwest::Client::builder()
        .user_agent("e6d - e621 dumping agent")
        .default_headers(header)
        .build()
        .unwrap();
    let mut position = 0u32;

    loop {
        println!(
            "Downloading page {}, {:?}",
            position,
            gen_post_url(base, tags, position)
        );
        let pause = Instant::now() + Duration::from_millis(500);
        let bytes = client
            .post(&gen_post_url(base, tags, position))
            .send()
            .compat()
            .await
            .unwrap()
            .bytes()
            .compat()
            .await
            .unwrap();
        let posts = serde_json::from_slice::<types::Posts>(bytes.as_ref()).unwrap();
        posts.posts.iter().for_each(|post| {
            if post.file.url.is_none() {
                return;
            }
            let name = post.file.url.as_ref().unwrap().split("/").last().unwrap();
            queue
                .send((post.file.url.as_ref().unwrap().clone(), name.to_string()))
                .unwrap();
        });
        position += 1;
        if posts.posts.len() < 75 {
            break;
        }
        if pause > Instant::now() {
            tokio::time::sleep_until(pause).await;
        }
    }
}

fn gen_post_url(base: &str, tags: &str, page: u32) -> String {
    format!(
        "https://{}/posts.json?page={}&tags={}",
        base,
        page,
        urlencoding::encode(tags)
    )
}
