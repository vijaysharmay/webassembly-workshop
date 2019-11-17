import time
from wasmer import Instance
import os

def main():
    sfip_start = time.time()
    with open(os.path.join("pkg", "slow_func_in_rust_bg.wasm"), "rb") as instance_bytes:
        instance = Instance(instance_bytes.read())
        instance.exports.slow_func()
    sfip_end = time.time()
    print(f"Time taken for slow function using WASM is {sfip_end - sfip_start} seconds")

if __name__ == "__main__":
    main()