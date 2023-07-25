# prints this help text
help:
    just --list

rook-rays:
    cargo run --bin rook_rays > chess-lookup/src/rook_rays.rs

bishop-rays:
    cargo run --bin bishop_rays > chess-lookup/src/bishop_rays.rs

lookup: rook-rays bishop-rays

make-bot name:
    cargo run -p chess-cli --features colorz/strip-colors --features tracing/max_level_off -- make-bot {{name}} --strip

fight *bots:
    cargo run -p chess-cli -r -- bot-fight {{bots}} -g 200 -t 1ms -g 50 -t 10ms -g 50 -t 100ms --thread-count 24