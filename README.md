```sh
sudo apt-get update
sudo apt install build-essential python3 python3-pip
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
wasm-pack build
```