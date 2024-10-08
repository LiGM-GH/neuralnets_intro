default:
    just --list

run cargoflags="":
    cargo run {{cargoflags}}

l1:
    cargo run part1 learn && feh ./images/part1.svg

l2:
    cargo run part2 learn && feh ./images/part2.svg

p1 x:
    cargo run part1 predict {{x}}

p2 x y:
    cargo run part2 predict {{x}} {{y}}

peek-pt1:
    feh ./images/part1.svg

peek-pt2:
    feh ./images/part2.svg

chfun:
    python3 create_data.py

show cargoflags="":
    cargo run {{cargoflags}}
    xdg-open images/part1.svg
