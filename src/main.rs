use reqwest::header::{HeaderMap, HeaderValue};
use std::path::Path;
use tokio_compat_02::FutureExt;

mod e621;

type DownloadQueue = flume::Sender<(String, String)>;

#[tokio::main]
async fn main() {
    let matches = clap::App::new("e6d")
        .arg(
            clap::Arg::with_name("tags")
                .required(true)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("out_dir")
                .short("o")
                .help("Output directory")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("domain")
                .long("domain")
                .takes_value(true)
                .default_value("www.e621.net")
                .help("Alternative: www.e926.net")
                .required(false),
        )
        .arg(
            clap::Arg::with_name("allow-blacklisted")
                .long("allow-blacklisted")
                .takes_value(false)
                .help("This option will enable the e621 by default blacklisted items and tried to resolve them\nDoes not work with --domain")
                .conflicts_with("domain")
                .required(false),
        )
        .get_matches();
    let (dl_queue, dl_receiver) = flume::unbounded();
    let tags = &*Box::leak(
        matches
            .value_of("tags")
            .unwrap()
            .to_string()
            .into_boxed_str(),
    );
    let out_dir = &*Box::leak(
        matches
            .value_of("out_dir")
            .unwrap_or("./")
            .to_string()
            .into_boxed_str(),
    );
    let base_url = &*Box::leak(
        matches
            .value_of("domain")
            .unwrap_or("./")
            .to_string()
            .into_boxed_str(),
    );
    let resolve_blacklisted = matches.occurrences_of("allow-blacklisted") > 0;
    if !Path::new(out_dir).exists() {
        tokio::fs::create_dir_all(out_dir).await.unwrap();
    }
    tokio::spawn(e621::search_crawl(
        base_url,
        &tags,
        dl_queue,
        resolve_blacklisted,
    ));
    let (rndv_tx, rndv_rx) = flume::bounded::<()>(0);
    for _ in 0..4 {
        let rx = dl_receiver.clone();
        let rndv_tx = rndv_tx.clone();
        let out_dir = Path::new(out_dir);
        tokio::spawn(async move {
            let mut header = HeaderMap::new();
            header.insert("Accept", HeaderValue::from_static("image/*"));
            let client = reqwest::Client::builder()
                .user_agent("e6d - e621 dumping agent")
                .default_headers(header)
                .build()
                .unwrap();
            while let Ok((url, file_name)) = rx.recv_async().await {
                let out = out_dir.join(std::path::Path::new(&file_name));
                if out.exists() {
                    println!("{} exists, skipping...", &file_name);
                    continue;
                }
                println!(
                    "Downloading {} -> {}",
                    url,
                    out.as_os_str().to_string_lossy()
                );
                let bytes = client
                    .get(&url)
                    .send()
                    .compat()
                    .await
                    .unwrap()
                    .bytes()
                    .compat()
                    .await
                    .unwrap();
                tokio::fs::write(&out, bytes).await.unwrap();
            }
            rndv_tx.send(()).unwrap()
        });
    }
    for _ in 0..4 {
        rndv_rx.recv_async().await.unwrap();
    }
}
