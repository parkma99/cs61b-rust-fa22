use std::time::{Duration, Instant};

use crate::benchmarkable::Benchmarkable;

/*
 Benchmarks the "add" and "get" methods of the Benchmarkable type, formatting the
 results in tables like this:

 Timing table for Vec::push
           N     time (s)        # ops  microsec/op
---------------------------------------------------
        1000     0.000003         1000     0.003356
        2000     0.000003         2000     0.001613
        4000     0.000006         4000     0.001470
        8000     0.000011         8000     0.001320
       16000     0.000021        16000     0.001316
       32000     0.000059        32000     0.001850
       64000     0.000104        64000     0.001619
      128000     0.000240       128000     0.001871

Timing table for Vec::last
           N     time (s)        # ops  microsec/op
---------------------------------------------------
        1000     0.000000        10000     0.000002
        2000     0.000000        10000     0.000002
        4000     0.000000        10000     0.000002
        8000     0.000000        10000     0.000002
       16000     0.000000        10000     0.000002
       32000     0.000000        10000     0.000003
       64000     0.000000        10000     0.000003
      128000     0.000000        10000     0.000002

 */
pub fn benchmark<T: Benchmarkable>() {
    let mut cur_size = 1000;

    let mut add_timings: Vec<Timing> = Vec::new();

    for _ in 0..8 {
        let mut list = T::new();
        let start = Instant::now();
        for _ in 0..cur_size {
            list.add(42);
        }
        let add_elapsed = start.elapsed();
        add_timings.push(Timing::new(cur_size, add_elapsed, cur_size));
        cur_size = cur_size * 2;
    }
    print_timing_table(T::ALG_NAME, T::ADD_NAME, &add_timings);

    let mut get_timings: Vec<Timing> = Vec::new();
    cur_size = 1000;
    for _ in 0..8 {
        let mut list = T::new();
        let start = Instant::now();
        for _ in 0..1000 {
            list.add(42);
        }
        let add_elapsed = start.elapsed();
        get_timings.push(Timing::new(cur_size, add_elapsed, 1000));
        cur_size = cur_size * 2;
    }
    print_timing_table(T::ALG_NAME, T::GET_NAME, &get_timings);
}

struct Timing {
    n: u64,
    elapsed: Duration,
    op_count: u64,
}

impl Timing {
    fn new(n: u64, elapsed: Duration, op_count: u64) -> Self {
        Timing {
            n,
            elapsed,
            op_count,
        }
    }
}

fn print_timing_table(alg_name: &str, method_name: &str, timings: &Vec<Timing>) {
    println!("Timing table for {}::{}", alg_name, method_name);
    println!(
        "{:>12} {:>12} {:>12} {:>12}",
        "N", "time (s)", "# ops", "microsec/op"
    );
    println!("{}", "-".repeat(4 * 12 + 3));
    for timing in timings {
        let time = timing.elapsed.as_secs_f64();
        let time_per_op = time * 1e6 / timing.op_count as f64;
        println!(
            "{:>12} {:>12.6} {:>12} {:>12.6}",
            timing.n, time, timing.op_count, time_per_op
        );
    }
    println!();
}
