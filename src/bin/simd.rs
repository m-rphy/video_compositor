#![cfg_attr(feature = "simd", feature(portable_simd))]

use video_recompositor::compositor;
use video_recompositor::io;
use video_recompositor::types;
use video_recompositor::write;
use std::fs;
use std::time::Instant;

// ── 1. Fast allocator ───────────────────────────────────────────────────────────
use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;        

// ── 2. Thread‑pool tuned to physical cores ─────────────────────────────────────
use rayon::ThreadPoolBuilder;
use num_cpus;            // add `num_cpus = "1.16"` to [dependencies] once

//  !! call this before any Rayon parallel is work !!
fn init_rayon() {
    // physical cores = better perf than logical on hybrid CPUs (Personal computer is M‑series w/ E/P‑cores)
    let phys = num_cpus::get_physical();   // 4 on M1, 8 on c7i.2xlarge
    ThreadPoolBuilder::new()
        .num_threads(phys)
        .build_global()
        .expect("failed to build global Rayon pool");
}

fn main() {
    init_rayon();

    let input_path  = "input/video.rvid";
    let output_path = "output/output.rvid";

    // read rules.json
    let rules: types::Rules = serde_json::from_str(
        &fs::read_to_string("input/rules.json").expect("Failed to read rules.json")
    ).expect("Failed to parse rules.json");

    let now = Instant::now();

    // load video into memory (your existing code)
    let input = io::read_raw_video(input_path).expect("Failed to read input");

    // composite
    let frames = compositor::composite_frames(
        &input.frames,
        &rules,
        (input.width, input.height),
    );

    // write output video
    write::write_raw_video(
        output_path,
        rules.size[0],
        rules.size[1],
        &frames,
    ).expect("Failed to write output");

    println!("Done in {:.2?}", now.elapsed());
}

