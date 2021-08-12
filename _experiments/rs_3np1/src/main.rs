extern crate stopwatch;
use stopwatch::{Stopwatch};

fn main() {
    let max: u64 = 10_000_000;
    let mut ctr: u64 = 1;
    
    // stopwatch
    let mut top: u64 = 1;
    let sw = Stopwatch::start_new();
    loop {
        let mut val: u64 = ctr;
        let mut iters: u64 = 1;

        loop {
            if val == 4 {
                break;
            }
            val = tnpo(val);
            iters += 1;
        }

        if iters > top {
            top = iters;
            println!("\r{}: {} iterations", ctr, iters);
        }

        if ctr == max {
            break;
        } else {
            ctr += 1;
        }
    }

    let tms = sw.elapsed_ms();
    let ms = tms % 1000;
    let s = (tms - ms) / 1000;
    println!("Time elapsed: {}_{}", s, ms);
}

fn tnpo(n: u64) -> u64 {
    if n % 2 == 0 { n / 2 } else { n * 3 + 1 }
}