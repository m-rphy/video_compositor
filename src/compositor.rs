use crate::types::*;
use rayon::prelude::*;
use std::simd::{Simd, LaneCount, SupportedLaneCount};
use std::simd::prelude::SimdUint;

#[derive(Clone)]
struct PreparedRule {
    sx: u32, sy: u32, sw: u32, sh: u32,
    dx: u32, dy: u32,
    a256:  u16,
    inv256: u16,
    opaque: bool,
}

#[inline(always)]
fn blend_bytes_simd<const LANES: usize>(
    src: &[u8],
    dst: &mut [u8],
    a256: u16,
    inv256: u16,
) where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Vu8 <const N: usize> = Simd<u8 , N>;
    type Vu16<const N: usize> = Simd<u16, N>;

    let va   = Vu16::<LANES>::splat(a256);
    let vinv = Vu16::<LANES>::splat(inv256);

    let mut i = 0;
    while i + LANES <= src.len() {
        let s  = Vu8::<LANES>::from_slice(&src[i..]);
        let d0 = Vu8::<LANES>::from_slice(&dst[i..]);
        let res = ((s.cast::<u16>() * va + d0.cast::<u16>() * vinv) >> 8)
                    .cast::<u8>();

        // stable-ish, portable write‑back
        dst[i .. i + LANES].copy_from_slice(&res.to_array());
        i += LANES;
    }
    for j in i..src.len() {
        dst[j] = ((src[j] as u16 * a256 + dst[j] as u16 * inv256) >> 8) as u8;
    }
}

/// Returns composited frames; one `Vec<u8>` per frame.
pub fn composite_frames(
    input: &[Vec<u8>],
    rules: &Rules,
    (in_width, in_height): (u32, u32),
) -> Vec<Vec<u8>> {
    let out_width   = rules.size[0] as usize;
    let out_height  = rules.size[1] as usize;
    let frame_size  = out_width * out_height * 3;

    // ── 1. Pre‑compute immutable rule table ─────────────────────────────
    let mut prules: Vec<PreparedRule> = rules.rects.iter().map(|r| {
        let a256   = (r.alpha * 256.0 + 0.5) as u16;
        let inv256 = 256 - a256;
        PreparedRule {
            sx: r.src[0], sy: r.src[1], sw: r.src[2], sh: r.src[3],
            dx: r.dest[0], dy: r.dest[1],
            a256, inv256,
            opaque: a256 == 256,
        }
    }).collect();
    prules.sort_by_key(|r| r.dy);   // or r.z for strict z‑order

    let frame_stride = in_width  as usize * 3;
    let out_stride   = out_width as usize * 3;

    // ── 2. Allocate one buffer per output frame ─────────────────────────
    let mut outputs: Vec<Vec<u8>> = (0..input.len())
        .map(|_| vec![0u8; frame_size])
        .collect();

    // ── 3. Parallel composition ─────────────────────────────────────────
    outputs.par_iter_mut().enumerate().for_each(|(idx, out)| {
        let frame = &input[idx];
        out.fill(0);

        for r in &prules {
            if r.sx >= in_width || r.sy >= in_height { continue; }

            for y in 0..r.sh {
                let src_row = (r.sy + y) as usize * frame_stride + r.sx as usize * 3;
                let dst_row = (r.dy + y) as usize * out_stride  + r.dx as usize * 3;

                let src = &frame[src_row .. src_row + r.sw as usize * 3];
                let dst = &mut out [dst_row .. dst_row + r.sw as usize * 3];

                if r.opaque {
                    dst.copy_from_slice(src);
                } else {
                    // 64 = 512 / 8; safe on all CPUs (LLVM splits as needed)
                    blend_bytes_simd::<64>(src, dst, r.a256, r.inv256);
                }
            }
        }
    });

    outputs
}

