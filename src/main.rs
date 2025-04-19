mod types;
mod io;
mod compositor;
mod write;

use std::fs;
use types::*;
use std::time::Instant;

fn main() {
    let input_path = "video.rvid"; 
    let output_path = "output.rvid"; 

    let rules: Rules = serde_json::from_str(
        &fs::read_to_string("rules.json").expect("Failed to read rules.json")
    ).expect("Failed to parse rules.json");

    let now = Instant::now();
    let input = io::read_raw_video(input_path).expect("Failed to read input");
    let frames = compositor::composite_frames(&input.frames, &rules, (input.width, input.height));

    // Write back to raw binary
    write::write_raw_video(output_path, rules.size[0], rules.size[1], &frames)
        .expect("Failed to write output");
    println!("Done in {:.2?}", now.elapsed());

    /* dir for png frames to be stitched together with ffmpeg */
    //let png_output_dir = "frames";
    
    /* Write PNGs for ffmpeg */
    //write::write_frames_as_pngs(png_output_dir, rules.size[0], rules.size[1], &frames)
    //    .expect("Failed to write PNG frames");

}

