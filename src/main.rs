use clap::Parser;
use futures::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Url to make request to
    #[arg(short, long)]
    url: String,

    /// Number of requests to make
    #[arg(short, long, default_value_t = 1)]
    number: u8,

    /// Number of concurrent threads
    #[arg(short, long, default_value_t = 1)]
    threads: usize,
}

async fn make_request(url: String) -> Result<reqwest::Response, reqwest::Error> {
    return Ok(reqwest::get(url).await?);
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let tasks = (0..args.number).map(|_| async {
        let args = Args::parse();
        make_request(args.url.clone()).await
    });

    let stream = futures::stream::iter(tasks).buffer_unordered(args.threads);
    let results = stream.collect::<Vec<_>>().await;

    let mut success_count = 0;
    let mut failure_count = 0;

    for result in results.iter() {
        match result {
            Ok(r) => match r.status() {
                reqwest::StatusCode::OK => success_count += 1,
                _ => failure_count += 1,
            },
            Err(_) => failure_count += 1,
        };
    }

    println!("Successes: {}", success_count);
    println!("Failures: {}", failure_count);
}
