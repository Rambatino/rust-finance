use futures::executor::block_on;
use futures::{stream, StreamExt}; // 0.3.5
use reqwest::Client; // 0.10.6

const CONCURRENT_REQUESTS: usize = 2;

#[derive(Debug)]
pub struct Parser {
    pub tickers: Vec<String>,
}

impl Parser {
    pub fn parse(&self)  {
        println!("hello1 {:#?}", self);
        for x in &self.tickers {
            println!("c = {:#?}", x);
        }
    }
}
async fn info(tickers: &Vec<String>) {
    let client = Client::new();

    let urls = tickers.into_iter().map(|t| format!("https://query1.finance.yahoo.com/v8/finance/chart/{}?region=GB&lang=en-GB&includePrePost=false&interval=2m&range=1d&corsDomain=uk.finance.yahoo.com&.tsrc=finance", t));
    let c = urls.len();
    println!("urls = {:#?}", urls);
    println!("c = {:#?}", c);

    stream::iter(urls)
        .map(|url| {
            let client = &client;
            async move {
                let resp = client.get(&url).send().await?;
                resp.bytes().await
            }
        })
        .buffer_unordered(c)
        .for_each(|b| async {
            match b {
                Ok(b) => 1,
                Err(e) => 2,
            }
        })
        .await;
}
