use clap::Parser;

use crate::args::GameArgs;
use crate::logic::game;

fn main() {
    let args = GameArgs::parse();
    game::start(args.players)
}

mod logic;
mod common;
mod args;
