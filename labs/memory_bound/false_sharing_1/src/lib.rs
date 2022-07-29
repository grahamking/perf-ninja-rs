#![feature(thread_id_value)]

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;

pub fn solution(data: &[u32], thread_count: usize) -> i32 {
    // Using std::atomic counters to disallow compiler to promote `target`
    // memory location into a register. This way we ensure that the store
    // to `target` stays inside the loop.
    let mut accumulators: Vec<Arc<AtomicU32>> = Vec::with_capacity(thread_count);
    for _ in 0..thread_count {
        accumulators.push(Default::default());
    }
    let chunks = data.chunks(data.len() / thread_count);
    thread::scope(|s| {
        for (idx, chunk) in chunks.enumerate() {
            let target_acc = accumulators[idx % thread_count].clone();
            s.spawn(move || {
                for v in chunk {
                    // Perform computation on each input
                    let mut item = *v;
                    item += 1000;
                    item ^= 0xADEDAE;
                    item |= item >> 24;

                    // Write result to accumulator
                    target_acc.fetch_add(item % 13, Ordering::Relaxed);
                }
            });
        }
    });

    accumulators
        .iter()
        .map(|v| v.load(Ordering::Relaxed) as usize)
        .sum::<usize>()
        .try_into()
        .unwrap()
}
