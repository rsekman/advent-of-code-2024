run DAY=`date +%d`:
    cargo run --release {{DAY}} < input/{{DAY}}

build DAY=`date +%d`:
    cargo build --release --bin {{DAY}}
