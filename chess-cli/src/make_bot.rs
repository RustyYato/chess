use std::path::PathBuf;

#[derive(Clone, clap::Parser)]
pub struct Args {
    name: String,
    #[clap(long)]
    strip: bool,
}

pub fn main(args: Args) {
    const TARGET_PATH: &str = "target/release/libchess_bot.so";
    std::process::Command::new("cargo")
        .args(["build", "--release", "-p", "chess-bot"])
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    if args.strip {
        std::process::Command::new("strip")
            .args([TARGET_PATH])
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    let meta = std::fs::metadata(TARGET_PATH).unwrap();

    let bot_path = PathBuf::from(format!("bots/{}.so", args.name));
    if let Some(parent) = bot_path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::remove_file(&bot_path).unwrap();
    std::fs::copy(TARGET_PATH, &bot_path).unwrap();

    eprintln!(
        "Created a new bot at {}, it has a size of {}",
        bot_path.display(),
        bytesize::ByteSize::b(meta.len())
    );
}
