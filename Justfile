# prints this help text
help:
    just --list

rook-rays:
    cargo run --bin rook_rays > chess-lookup/src/rook_rays.rs

bishop-rays:
    cargo run --bin bishop_rays > chess-lookup/src/bishop_rays.rs
