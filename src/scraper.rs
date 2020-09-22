pub mod parser;
pub mod info;

pub fn run(tickers: Vec<String>) {
    let parser = parser::Parser{tickers: tickers};
    println!("{:#?}", parser.parse());
}
