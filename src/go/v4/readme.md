# go v4 — 2.80s (vs v2's 3.80s)

Share-nothing parallel workers over `pread`, work-stealing 128MB chunks,
zero allocations per line (SIMD `bytes.IndexByte` field hops, flat month
array, arena-backed open-addressing name table). See the header comment in
`main.go` for the full design rationale.

Key macOS lesson: an mmap version of the same code ran at 6.5s pinned to
~1 core, because macOS serializes concurrent page faults on one mapped file
behind a per-VM-object lock. `pread` into private buffers parallelizes fine.

At 2.80s this is I/O-bound: the file streams at ~1.55GB/s vs the SSD's
1.71GB/s raw sequential ceiling (Apple M2, 16GB RAM — the 4.3GB file does
not stay page-cached, so every run pays the disk).
