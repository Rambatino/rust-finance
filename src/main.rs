#[macro_use]
extern crate rocket;

mod scraper;

#[get("/version")]
fn version() -> String {
    format!("0.1")
}

#[get("/info/<tickers>")]
fn ticker(tickers: String) -> String {
    scraper::run(tickers.split(",").map(|s| s.to_string()).collect());
    format!("Ticker = {}", tickers)
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![version, ticker])
}
