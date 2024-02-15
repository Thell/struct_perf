import pytest
import struct_perf

TEST_SIZE = 1_000_000


@pytest.mark.benchmark(group="single_call")
def test_lcg_static(benchmark):
    struct_perf.lcg_static_init()
    benchmark(struct_perf.lcg_static_do_something)


@pytest.mark.benchmark(group="single_call")
def test_lcg_static_lazy(benchmark):
    struct_perf.lcg_static_lazy_init()
    benchmark(struct_perf.lcg_static_lazy_do_something)


@pytest.mark.benchmark(group="single_call")
def test_lcg_struct(benchmark):
    test_struct = struct_perf.LCGStruct()
    benchmark(test_struct.do_something)


@pytest.mark.benchmark(group="single_call")
def test_xoshiro_static_lazy(benchmark):
    struct_perf.xoshiro_static_lazy_init()
    benchmark(struct_perf.xoshiro_static_lazy_do_something)


@pytest.mark.benchmark(group="single_call")
def test_xoshiro_struct(benchmark):
    test_struct = struct_perf.XoshiroStruct()
    benchmark(test_struct.do_something)


@pytest.mark.benchmark(group=f"for_loop_{TEST_SIZE}")
def test_lcg_static_loop(benchmark):
    struct_perf.lcg_static_init()

    def do_loop():
        for _ in range(TEST_SIZE):
            struct_perf.lcg_static_do_something()

    benchmark(do_loop)


@pytest.mark.benchmark(group=f"for_loop_{TEST_SIZE}")
def test_lcg_static_lazy_loop(benchmark):
    struct_perf.lcg_static_lazy_init()

    def do_loop():
        for _ in range(TEST_SIZE):
            struct_perf.lcg_static_lazy_do_something()

    benchmark(do_loop)


@pytest.mark.benchmark(group=f"for_loop_{TEST_SIZE}")
def test_lcg_struct_loop(benchmark):
    test_struct = struct_perf.LCGStruct()

    def do_loop():
        for _ in range(TEST_SIZE):
            test_struct.do_something()

    benchmark(do_loop)


@pytest.mark.benchmark(group=f"for_loop_{TEST_SIZE}")
def test_xoshiro_static_lazy_loop(benchmark):
    struct_perf.xoshiro_static_lazy_init()

    def do_loop():
        for _ in range(TEST_SIZE):
            struct_perf.xoshiro_static_lazy_do_something()

    benchmark(do_loop)


@pytest.mark.benchmark(group=f"for_loop_{TEST_SIZE}")
def test_xoshiro_struct_loop(benchmark):
    test_struct = struct_perf.XoshiroStruct()

    def do_loop():
        for _ in range(TEST_SIZE):
            test_struct.do_something()

    benchmark(do_loop)
