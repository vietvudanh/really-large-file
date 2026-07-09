# Process really large file

Inspired by this.

## Tasks

- Write a program that will print out the total number of lines in the file.
- Notice that the 8th column contains a person’s name. Write a program that loads in this data and creates an array with all name strings. Print out the 432nd and 43243rd names.
- Notice that the 5th column contains a form of date. Count how many donations occurred in each month and print out the results.
- Notice that the 8th column contains a person’s name. Create an array with each first name. Identify the most common first name in the data and how many times it occurs.

## Getting the data file

`run.sh` expects the data file at `~/data/misc/itcont.txt`. It is not checked into this repo
(see `.gitignore`).

Download it from the FEC bulk data page:

    https://www.fec.gov/files/bulk-downloads/2018/indiv18.zip

(that URL redirects to an S3 bucket, e.g.
`https://cg-519a459a-0ea3-42c2-b7bc-fa1143481f74.s3-us-gov-west-1.amazonaws.com/bulk-downloads/2018/indiv18.zip`
— the bucket name may change over time, so prefer the `fec.gov` link above).

As of writing the zip is ~1.6GB and unzips to an `itcont.txt` several GB in size. After
downloading, unzip it and move it into place:

    mkdir -p ~/data/misc
    unzip indiv18.zip -d ~/data/misc

# Lesson learnt

    - pypy is fucking slow, what?
    - solution at https://itnext.io/using-java-to-read-really-really-large-files-a6f8a3f44649 is fucking bad, GC overhead all the time

## 2026 full re-run

Hardware matters — all numbers below are from the same box and are not
comparable to the older timings above (different machine, smaller file back
then):

> Apple M2 (4 performance + 4 efficiency cores), 16GB RAM, macOS.
> SSD raw sequential read: **1.71 GB/s** (`dd bs=8m`).
> Data: `itcont.txt`, 4.29GB, 21,730,731 lines. Toolchains: go 1.26.5,
> rustc 1.96.1, scala 3.8.4, OpenJDK 26.0.1, CPython 3.12.9, pypy3 7.3.23.

| impl | time | bottleneck |
|---|---|---|
| **rust v3** | **2.77s** | disk (~1.55GB/s of the 1.71GB/s ceiling) |
| **go v4** | **2.80s** | disk (same) |
| go v0 (count only) | 3.39s | single-thread scan |
| go v2 | 3.80s | single scanner goroutine + per-line allocs |
| go v3 | 4.29s | same + mutex merge |
| go v9_marc | 4.75s | same, sync.Pool variant |
| scala v1 (count only) | 5.48s | single-thread scan |
| rust v2 | 6.65s | reader thread + Arc<Mutex> on every batch |
| rust v1 | 10.47s | same, unbatched locking |
| pypy3 baseline (count only) | 9.68s | interpreter |
| python3 baseline (count only) | 14.28s | interpreter |
| scala main.v1 | 20.92s | single-thread, per-line splits |
| python v1/v2 (either interpreter) | 27–31s | interpreter, per-line splits |
| java StoringOnlySummaryData | 41.6s | single-thread, but sane data structures |
| java BufferedReader / LineIterator / FileInputStream | 69–100s | stores full name list, GC pressure |

Why the new go v4 / rust v3 are fast (see their readmes for details):

1. **No shared anything.** Workers get byte ranges, parse independently with
   per-worker counters/tables, merge once at the end. Every older concurrent
   version paid a channel or mutex per batch.
2. **No allocations per line.** Fields are located with SIMD byte search
   (`bytes.IndexByte` / `memchr`); months land in a flat array indexed by
   `(year*12+month)`; names in a per-worker hash table. The old versions
   allocated a string + a 9-element split per line — 21.7M times.
3. **pread, not mmap — macOS-specific.** An mmap version of the exact same
   code ran 6.5s pinned to ~1 core: macOS serializes concurrent page faults
   on a mapped file behind a per-VM-object lock. `pread` per worker into
   private buffers hits full parallelism. (On Linux, mmap would likely win.)
4. **Work-stealing 128MB chunks**, because 4 P-cores + 4 E-cores means a
   static equal split waits on the slowest E-core.

The remaining constraint is the SSD: 4.29GB at the drive's 1.71GB/s floor is
2.51s, and both implementations run at ~2.8s (~90% of it). The 16GB machine
does not keep the 4.3GB file page-cached between runs (verified: warm reruns
are not faster), so every run pays the full disk read. To go meaningfully
below ~2.8s on this box you'd need faster storage or more RAM — not better
code. Go and Rust tie because both are waiting on the disk; Rust does it
with ~30% less CPU (2.4s vs 3.2s user), so on faster storage rust v3 pulls
ahead.

Old-lesson revisited: on this run **pypy3 beat CPython** on the trivial
line-count baseline (9.7s vs 14.3s) and roughly tied on the full task
(27–31s both) — "pypy is fucking slow" did not reproduce.