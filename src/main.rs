mod types;
mod io;
mod compositor;
mod write;

use std::{fs, time::Instant};
use crate::types::*;
use crate::io::read_raw_video_mmap;

fn main() {
    let input_path      = "video.rvid";
    let output_path     = "output.rvid";
    let png_output_dir  = "frames";

    // load rules
    let rules: Rules = serde_json::from_str(
        &fs::read_to_string("rules.json").expect("Failed to read rules.json"),
    ).expect("Failed to parse rules.json");

    let timer = Instant::now();

    // mmap the input once
    let input_video = read_raw_video_mmap(input_path).expect("Failed to mmap input");

    // recomposite
    let frames = compositor::composite_frames(&input_video, &rules);

    // write output
    write::write_raw_video(
        output_path,
        rules.size[0],       // output width
        rules.size[1],       // output height
        &frames,
    ).expect("Failed to write output");

    println!("Done in {:.2?}", timer.elapsed());

    // ── optional PNG export ───────────────────────────────
    // write::write_frames_as_pngs(png_output_dir, rules.size[0], rules.size[1], &frames)
    //     .expect("Failed to write PNG frames");
}

