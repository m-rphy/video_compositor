use crate::{types::*, io::MappedVideo};
use rayon::prelude::*;

/// Returns the recomposited frames as owned `Vec<u8>` buffers
/// (you still need owned data for the output file).
pub fn composite_frames(video: &MappedVideo, rules: &Rules) -> Vec<Vec<u8>> {
    let out_width   = rules.size[0] as usize;
    let out_height  = rules.size[1] as usize;
    let frame_size  = out_width * out_height * 3;

    // sort rules once
    let mut rules_sorted = rules.rects.clone();
    rules_sorted.sort_by_key(|r| r.z);

    (0..video.frames()).into_par_iter().map(|idx| {
        let frame = video.frame(idx);          // borrow on demand
        let mut out = vec![0u8; frame_size];

        for rule in &rules_sorted {
            let [sx, sy, sw, sh] = rule.src;
            let [dx, dy] = rule.dest;
            let alpha = rule.alpha;

            for y in 0..sh {
                for x in 0..sw {
                    let src_x = (sx + x) as usize;
                    let src_y = (sy + y) as usize;
                    let dst_x = (dx + x) as usize;
                    let dst_y = (dy + y) as usize;

                    if src_x >= video.width  as usize || src_y >= video.height as usize { continue; }
                    if dst_x >= out_width || dst_y >= out_height { continue; }

                    let src_i = (src_y * video.width  as usize + src_x) * 3;
                    let dst_i = (dst_y * out_width + dst_x) * 3;

                    for c in 0..3 {
                        let s = frame[src_i + c] as f32;
                        let d = out[dst_i + c] as f32;
                        out[dst_i + c] = (s * alpha + d * (1.0 - alpha)).min(255.0) as u8;
                    }
                }
            }
        }
        out
    }).collect()
}

