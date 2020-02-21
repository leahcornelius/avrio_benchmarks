# Avrio Benchmarks
![Build Status] (https://api.travis-ci.com/leocornelius/avrio_benchmarks.svg)
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

Wan't to see a diffrent aspect of the code benchmarked, tested or checked here? Open an issue asking for it to be added and i will see wht i can do. Alternativly if you know rust and can code it your self PR are allways welcome!
