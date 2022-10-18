use clap::Parser;

/// Command-line Big 2 card game.
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct GameArgs {
    /// Number of players.
    #[arg(short, long, default_value_t = 4)]
    pub players: usize,
    /// Play a hotseat game without AI.
    #[arg(long, default_value_t = true)]
    pub hotseat: bool,
}
