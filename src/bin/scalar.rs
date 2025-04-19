use std::fs;

use video_recompositor::compositor;
use video_recompositor::io;
use video_recompositor::types::Rules;
use video_recompositor::write;

//use std::time::Instant;

fn main() {
    let input_path = "input/video.rvid"; 
    let output_path = "output/output.rvid"; 

    let rules: Rules = serde_json::from_str(
        &fs::read_to_string("input/rules.json").expect("Failed to read rules.json")
    ).expect("Failed to parse rules.json");

    //let now = Instant::now();
    let input = io::read_raw_video(input_path).expect("Failed to read input");
    let frames = compositor::composite_frames(&input.frames, &rules, (input.width, input.height));

    // Write back to raw binary
    write::write_raw_video(output_path, rules.size[0], rules.size[1], &frames)
        .expect("Failed to write output");
    //println!("Done in {:.2?}", now.elapsed());
}

