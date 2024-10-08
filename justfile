set windows-shell := ["powershell.exe", "-c"]

default:
    just --list
    echo "We'll need images directory. Please created one."

run cargoflags="":
    cargo run {{cargoflags}}

l1:
    cargo run part1 learn

l2:
    cargo run part2 learn

p1 x:
    cargo run part1 predict {{x}}

p2 x y:
    cargo run part2 predict {{x}} {{y}}

chfun:
    python3 create_data.py
