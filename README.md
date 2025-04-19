# video_recompositor

> A zero‑copy raw‑video compositor with rule‑based cropping, alpha blending,
> multi‑threading (Rayon) and **portable SIMD** acceleration.

---

## Quick start

Put `.rvid` data and `.json` rules in `/input` directory.
Please make sure they are named `video.rvid` and `rules.json`

The output data will be raw binary that is put in the `/output` directory 
with the name `output.rvid`.

```bash
git clone …
cd video_recompositor
```

## Compile the scalar, stable build cross-arch build (always works)

```bash
cargo run --release --bin scalar
```

## Compile the SIMD, cross‑arch build (needs nightly)
```bash
rustup toolchain install nightly     # once
cargo +nightly run --release --features simd --bin simd
```

| Host                                     | Build                    | Wall-time* | Peak RSS |
|------------------------------------------|---------------------------|------------|----------|
| Apple M1 (4P + 4E)                       | stable scalar             | 16.5 s     | 730 MB   |
|                                          | nightly `SIMDLANES = 16`  | 14 s       | 730 MB   |
| AWS c7i.2xlarge                          | stable scalar             | 11 s       | 730 MB   |
(Sapphire Rapids AVX-512, 8 vCPU) 
|                                          | nightly `SIMDLANES = 64`  | 7.7 s      | 730 MB   |


## Architectural Choices & Optimizations

| Area           | Decision                             | Why                                                                 |
|----------------|--------------------------------------|----------------------------------------------------------------------|
| Memory layout  | Single read into `Vec<Vec<u8>>`      | Fits comfortably in RAM; avoids mmap page faults on Apple Silicon   |
| Parallelism    | Rayon, job = N frames                | Good core utilization; thread count = physical cores (override with `VR_THREADS`) |
| Allocator      | mimalloc (`feature = "fast-alloc"`)  | Lower contention than glibc on many-core Intel; opt-out via `--no-default-features` |
| Blending math  | 0–256 fixed-point α                  | 3× faster than `f32` with no precision loss                         |
| SIMD           | `std::simd` nightly feature          | Same Rust compiles to NEON (ARM) or AVX-512 (x86); lane width chosen with `cfg` |
| I/O            | Files live in `/dev/ramfs`           | Disk latency irrelevant; no async needed                            |

