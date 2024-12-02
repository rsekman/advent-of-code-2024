run DAY=`date +%d`:
    cargo run --release --bin {{DAY}} < input/{{DAY}}

build DAY=`date +%d`:
    cargo build --release --bin {{DAY}}
