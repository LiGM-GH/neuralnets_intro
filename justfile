default:
    just --list

run cargoflags="":
    cargo run {{cargoflags}}

pt1:
    cargo run part1 && feh ./images/part1.svg

pt2:
    cargo run part2 && feh ./images/part2.svg

peek-pt1:
    feh ./images/part1.svg

peek-pt2:
    feh ./images/part2.svg

chfun:
    python3 create_data.py

show cargoflags="":
    cargo run {{cargoflags}}
    xdg-open images/part1.svg
