# Avrio Benchmarks
This repo contains benchmarks for diffrent aspects or avrios code. 
First ensure you have installed rust and git
Then run
```
git pull https://github.com/leocornelius/avrio_benchmarks
cd avrio_benchmarks
```
To Run them all run
```
cargo build --release
cargo run -p all
```
To run one specific benchmark run
``` 
cargo build --release
cargo run -p benchmark_name
```
eg
```cargo run -p transaction```
