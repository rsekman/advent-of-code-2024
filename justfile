run DAY=`date +%d`:
    cargo run --release --bin {{DAY}} < input/{{DAY}}

example DAY=`date +%d`:
    cargo run --release --bin {{DAY}} < input/{{DAY}}_example

build DAY=`date +%d`:
    cargo build --release --bin {{DAY}}
