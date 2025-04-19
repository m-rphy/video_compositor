# video_recompositor

> A zero‑copy raw‑video compositor with rule‑based cropping, alpha blending,
> multi‑threading (Rayon) and **portable SIMD** acceleration.

---

## Quick start

```bash
git clone …
cd video_recompositor
```

# 0 run pre-built versions

```bash
./src/bin/{{scalar/smid}}
```

# 1 compile the scalar, stable build (always works)

```bash
cargo build --release
./target/release/video_recompositor
```

# 2 compile the SIMD, cross‑arch build (needs nightly)
```bash
rustup toolchain install nightly     # once
rustup override set nightly          # in this repo only
cargo build --release
./target/release/video_recompositor
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

