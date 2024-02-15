#![feature(test)]
extern crate test;

use pyo3::prelude::*;

use lazy_mut::lazy_mut;
use rand::RngCore;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;

//* STRUCTS

#[pyclass]
pub struct LCGStruct {
    state: u64,
}

impl LCGStruct {
    fn next_state(&mut self) -> u64 {
        const MULTIPLIER: u64 = 6364136223846793005;
        const INCREMENT: u64 = 1442695040888963407;
        self.state = self.state.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT);
        self.state
    }
}

#[pymethods]
impl LCGStruct {
    #[new]
    pub fn new() -> Self {
        Self {
            state: rand::random::<u64>(),
        }
    }

    pub fn do_something(&mut self) -> u64 {
        do_it(self.next_state())
    }
}

#[pyclass]
pub struct XoshiroStruct {
    state: Xoshiro256Plus,
}

impl XoshiroStruct {
    fn next_state(&mut self) -> u64 {
        self.state.next_u64()
    }
}

#[pymethods]
impl XoshiroStruct {
    #[new]
    pub fn new() -> Self {
        Self {
            state: Xoshiro256Plus::from_entropy(),
        }
    }

    pub fn do_something(&mut self) -> u64 {
        do_it(self.next_state())
    }
}

//* STATICS

static mut LCG_STATIC_STATE: u64 = 0;

#[pyfunction]
pub fn lcg_static_init() {
    unsafe {
        LCG_STATIC_STATE = rand::random::<u64>();
    };
}

fn lcg_static_next_state() -> u64 {
    const MULTIPLIER: u64 = 6364136223846793005;
    const INCREMENT: u64 = 1442695040888963407;

    unsafe {
        LCG_STATIC_STATE = LCG_STATIC_STATE
            .wrapping_mul(MULTIPLIER)
            .wrapping_add(INCREMENT);
        LCG_STATIC_STATE
    }
}

#[pyfunction]
pub fn lcg_static_do_something() -> u64 {
    do_it(lcg_static_next_state())
}

lazy_mut! {
    static mut LCG_STATIC_LAZY_STATE: u64 = rand::random::<u64>();
}

#[pyfunction]
pub fn lcg_static_lazy_init() {
    unsafe {
        LCG_STATIC_LAZY_STATE.init();
    };
}

fn lcg_static_lazy_next_state() -> u64 {
    const MULTIPLIER: u64 = 6364136223846793005;
    const INCREMENT: u64 = 1442695040888963407;

    unsafe {
        *LCG_STATIC_LAZY_STATE = LCG_STATIC_LAZY_STATE
            .wrapping_mul(MULTIPLIER)
            .wrapping_add(INCREMENT);
        *LCG_STATIC_LAZY_STATE
    }
}

#[pyfunction]
pub fn lcg_static_lazy_do_something() -> u64 {
    do_it(lcg_static_lazy_next_state())
}

lazy_mut! {
    static mut XOSHIRO_STATIC_LAZY_STATE: Xoshiro256Plus = Xoshiro256Plus::from_entropy();
}

fn xoshiro_static_lazy_next_state() -> u64 {
    unsafe { XOSHIRO_STATIC_LAZY_STATE.next_u64() }
}

#[pyfunction]
pub fn xoshiro_static_lazy_init() {
    unsafe { XOSHIRO_STATIC_LAZY_STATE.init() };
}

#[pyfunction]
pub fn xoshiro_static_lazy_do_something() -> u64 {
    do_it(xoshiro_static_lazy_next_state())
}

/**------------------------------------------------------------------------
 **                    Module
 *------------------------------------------------------------------------**/

#[pymodule]
fn struct_perf(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<LCGStruct>()?;
    m.add_class::<XoshiroStruct>()?;
    m.add_function(wrap_pyfunction!(lcg_static_init, m)?)?;
    m.add_function(wrap_pyfunction!(lcg_static_do_something, m)?)?;
    m.add_function(wrap_pyfunction!(lcg_static_lazy_init, m)?)?;
    m.add_function(wrap_pyfunction!(lcg_static_lazy_do_something, m)?)?;
    m.add_function(wrap_pyfunction!(xoshiro_static_lazy_init, m)?)?;
    m.add_function(wrap_pyfunction!(xoshiro_static_lazy_do_something, m)?)?;
    Ok(())
}

fn do_it(num: u64) -> u64 {
    static HALF: u64 = u64::MAX / 2;
    if num <= HALF {
        0
    } else {
        1
    }
}

/**------------------------------------------------------------------------
 **                    Cargo Bench
 *------------------------------------------------------------------------**/

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused)]
    use test::{black_box, Bencher};

    #[allow(unused)]
    static BENCH_SIZE: usize = 100_000_000;

    #[bench]
    fn bench_lcg_struct(b: &mut Bencher) {
        let mut rng = LCGStruct::new();
        b.iter(|| {
            for _ in 0..BENCH_SIZE {
                black_box(rng.do_something());
            }
        });
    }

    #[bench]
    fn bench_xoshiro_struct(b: &mut Bencher) {
        let mut rng = XoshiroStruct::new();
        b.iter(|| {
            for _ in 0..BENCH_SIZE {
                black_box(rng.do_something());
            }
        });
    }

    #[bench]
    fn bench_lcg_static(b: &mut Bencher) {
        lcg_static_init();
        b.iter(|| {
            for _ in 0..BENCH_SIZE {
                black_box(lcg_static_do_something());
            }
        });
    }

    #[bench]
    fn bench_lcg_static_lazy(b: &mut Bencher) {
        lcg_static_lazy_init();
        b.iter(|| {
            for _ in 0..BENCH_SIZE {
                black_box(lcg_static_lazy_do_something());
            }
        });
    }

    #[bench]
    fn bench_xoshiro_static_lazy(b: &mut Bencher) {
        xoshiro_static_lazy_init();
        b.iter(|| {
            for _ in 0..BENCH_SIZE {
                black_box(xoshiro_static_lazy_do_something());
            }
        });
    }
}
