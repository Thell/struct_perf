import struct_perf
import timeit

BENCH_SIZE = 100_000_000

struct_perf.lcg_static_init()
result = timeit.timeit(struct_perf.lcg_static_do_something, number=BENCH_SIZE)
print(f"lcg_static: {result}")

struct_perf.lcg_static_lazy_init()
result = timeit.timeit(struct_perf.lcg_static_lazy_do_something, number=BENCH_SIZE)
print(f"lcg_static_lazy: {result}")

lcg_struct = struct_perf.LCGStruct()
result = timeit.timeit(lcg_struct.do_something, number=BENCH_SIZE)
print(f"lcg_struct: {result}")

struct_perf.xoshiro_static_lazy_init()
result = timeit.timeit(struct_perf.xoshiro_static_lazy_do_something, number=BENCH_SIZE)
print(f"xoshiro_static_lazy: {result}")

xoshiro_struct = struct_perf.XoshiroStruct()
result = timeit.timeit(xoshiro_struct.do_something, number=BENCH_SIZE)
print(f"xoshiro_struct: {result}")
