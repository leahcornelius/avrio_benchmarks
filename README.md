# Avrio Benchmarks
This repo contains benchmarks for diffrent aspects or avrios code. 
First ensure you have installed rust and git
Then run
```
sudo add-apt-repository ppa:ubuntu-toolchain-r/test -y
sudo apt-get update
sudo apt-get install -y build-essential g++-8 gcc-8 git libboost-all-dev libssl1.0-dev cmake
git pull https://github.com/leocornelius/avrio_benchmarks
cd avrio_benchmarks
```
To run one specific benchmark run
``` 
cargo build --release
cargo run -p benchmark_name
```
eg
```cargo run -p transaction```
