use crate::types::*;
use rayon::prelude::*;

pub fn composite_frames(
    input: &[Vec<u8>],
    rules: &Rules,
    input_dims: (u32, u32),
) -> Vec<Vec<u8>> {
    let (in_width, in_height) = input_dims;
    let out_width = rules.size[0] as usize;
    let out_height = rules.size[1] as usize;
    let frame_size = out_width * out_height * 3;

    // Sort rules by z-index
    let mut rules_sorted = rules.rects.clone();
    rules_sorted.sort_by_key(|r| r.z);

    input.par_iter().map(|frame| {
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

                    if src_x >= in_width as usize || src_y >= in_height as usize { continue; }
                    if dst_x >= out_width || dst_y >= out_height { continue; }

                    let src_i = (src_y * in_width as usize + src_x) * 3;
                    let dst_i = (dst_y * out_width + dst_x) * 3;

                    for c in 0..3 {
                        let src_val = frame[src_i + c] as f32;
                        let dst_val = out[dst_i + c] as f32;
                        let blended = (src_val * alpha + dst_val * (1.0 - alpha)).min(255.0);
                        out[dst_i + c] = blended as u8;
                    }
                }
            }
        }

        out
    }).collect()
}

