default:
    just --list

run cargoflags="":
    cargo run {{cargoflags}}

chfun:
    python3 create_data.py

show cargoflags="":
    cargo run {{cargoflags}}
    xdg-open images/part1.svg
