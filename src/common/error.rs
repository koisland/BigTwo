use io::Error;

enum DeckError {
    InvalidChunks(Error)
}
//https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/README.html
enum HandError {
    InvalidChunks(Error),
    InvalidComparison(),
}
