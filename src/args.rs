use clap::Parser;

/// Command-line Big 2 card game.
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct GameArgs {
    /// Number of players.
    #[clap(short, long, default_value_t = 4)]
    pub players: usize,
    /// Play a hotseat game without AI.
    #[clap(long)]
    pub hotseat: bool,
}
