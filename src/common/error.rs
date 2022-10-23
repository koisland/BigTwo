use std::io::Error;

#[derive(Debug)]
pub struct InvalidChunks;

#[derive(Debug)]
pub struct InvalidComparison;

#[derive(Debug)]
pub enum DeckError {
    InvalidChunks(String),
}

#[derive(Debug)]
//https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/README.html
pub enum HandError {
    InvalidHand(String),
    InvalidChunks(String),
    InvalidComparison(String),
}
