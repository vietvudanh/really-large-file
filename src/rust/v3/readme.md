# rust v3 — 2.77s (vs v2's 6.65s)

Share-nothing parallel workers over `pread` (`FileExt::read_at`),
work-stealing 128MB chunks, near-zero allocations per line (SIMD `memchr`
field hops, flat month array, per-worker `FxHashMap`). Built with `lto=fat`,
`codegen-units=1`, `target-cpu=native`. See the header comment in
`src/main.rs` for the full design rationale.

Key macOS lesson: an mmap version of the same design runs pinned to ~1 core,
because macOS serializes concurrent page faults on one mapped file behind a
per-VM-object lock. `pread` into private buffers parallelizes fine.

At 2.77s this is I/O-bound: ~1.55GB/s vs the SSD's 1.71GB/s raw sequential
ceiling (Apple M2, 16GB RAM). CPU cost is ~30% below go v4 (2.4s vs 3.2s
user), so on faster storage rust pulls ahead; on this disk they tie.
