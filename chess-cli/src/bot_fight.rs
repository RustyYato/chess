use std::path::PathBuf;

#[derive(Clone, clap::Parser)]
pub struct Args {
    #[clap(required = true, num_args = 2..)]
    bots: Vec<PathBuf>,
    #[clap(long)]
    thread_count: Option<u32>,
}

pub fn main(_args: Args) {
    todo!()
}
