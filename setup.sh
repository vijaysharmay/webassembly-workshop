#!/bin/sh

curl -sL https://deb.nodesource.com/setup_13.x | sudo -E bash -
sudo apt-get update
sudo apt install -y build-essential python3 python3-pip nodejs gcc g++ make zsh
sh -c "$(wget -O- https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh)"
git clone https://github.com/emscripten-core/emsdk.git
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
curl https://wasmtime.dev/install.sh -sSf | bash
