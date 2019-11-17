import time
from math import tan, atan

def main():
    sfip_start = time.time()
    result = 0
    upperbound = 10 ** 7
    for i in range(upperbound, 0, -1):
        result += tan(i) * atan(i)
    sfip_end = time.time()
    print(f"Time taken for slow function is {sfip_end - sfip_start} seconds")

if __name__ == "__main__":
    main()