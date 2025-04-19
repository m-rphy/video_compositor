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

        let frame_stride = in_width  as usize * 3;
        let out_stride   = out_width as usize * 3;

        for rule in &rules_sorted {
            let [sx, sy, sw, sh] = rule.src;
            let [dx, dy]         = rule.dest;
            if sx >= in_width || sy >= in_height { continue; }
        
            let a     = (rule.alpha * 255.0 + 0.5) as u16;
            let inv_a = 255 - a;
            let opaque = a == 255;
        
            for y in 0..sh {
                let src_row = (sy + y) as usize * frame_stride + sx as usize * 3;
                let dst_row = (dy + y) as usize * out_stride  + dx as usize * 3;
        
                let src = &frame[src_row .. src_row + sw as usize * 3];
                let dst = &mut out[dst_row .. dst_row + sw as usize * 3];
        
                if opaque {
                    dst.copy_from_slice(src);           //  fast path
                } else {
                    for (s, d) in src.chunks_exact(3).zip(dst.chunks_exact_mut(3)) {
                        for c in 0..3 {
                            let sc = s[c] as u16;
                            let dc = d[c] as u16;
                            d[c] = (((sc * a + dc * inv_a) + 127) / 255) as u8;
                        }
                    }
                }
            }
        }

        out
    }).collect()
}

