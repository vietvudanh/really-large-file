# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this repo is

A personal benchmark playground comparing languages/approaches for processing a huge pipe-delimited
FEC campaign-contribution data file (`itcont.txt`, one of the bulk files from
https://www.fec.gov/files/bulk-downloads/2018/indiv18.zip). Every implementation solves the same
four tasks against that file:

1. Count the total number of lines.
2. Column 8 (0-indexed: column index 7) is a person's name (`LAST, FIRST ...`). Build an array of
   names and print the 433rd and 43244th entries (1-indexed counters, so indices 432/43243 in
   0-indexed terms — implementations differ slightly on off-by-one, see existing code before "fixing").
3. Column 5 (index 4) is a date like `201801180300185828`; the month is the first 6 digits
   (`YYYYMM`). Count donations per month.
4. From the column-8 names, extract first names (text before `", "`) and find the most common one
   plus its count.

There is no single "app" — `src/<language>/v<N>/` are successive, independent attempts at the same
problem, each one iterating on performance (more concurrency, batching, pooling, fewer allocations).
Higher version numbers are not supersets of lower ones; treat each `vN` directory as a self-contained
program. When asked to "improve performance" or "add a feature," clarify which language/version is
in scope rather than assuming the latest.

## Data file

The input file is **not** in the repo (`.gitignore` excludes `**/data`, `**/dist`, `**/target`,
`**/build`). `run.sh` expects it at `~/data/misc/itcont.txt`. The file is pipe (`|`) delimited with
no header row. Any code change should be validated by eye against the field-splitting logic already
in each implementation, since there's no committed sample/fixture to run tests against.

## Repo layout

```
src/go/v0..v4       Go attempts, in increasing sophistication:
                      v0 - naive line count only
                      v1 - full 4 tasks, single-threaded
                      v2 - goroutines + channels, chunked line processing
                      v3 - goroutines + a shared mutex instead of a fan-in channel
                      v4 - pread + work-stealing chunks, zero-alloc parse; I/O-bound
                           (fastest; see its readme for the macOS mmap lesson)
src/go/v9_marc       A later/alternate Go attempt (object pooling with sync.Pool for
                      line/entry slices, atomic counters); "v9" is not part of the v0-v3 sequence.
src/rust/v0..v3       Rust attempts: v0 spawns threads per batch but doesn't reduce results,
                      v1 introduces batching, v2 uses Arc<Mutex<...>> shared state across
                      worker threads, v3 (fastest) is pread + work-stealing chunks with
                      memchr/FxHashMap and cargo release tuning (see its readme).
src/rust/test_read.rs Standalone scratch/experiment file, not part of any crate.
src/scala/            scala.v1.scala / main.v1.scala - single-threaded Scala version;
                      run via run_template.sh (invokes `scala SRC_NAME <file>`).
src/java/read-file-java/  A full Gradle project (separate from the rest, has its own README)
                      comparing BufferedReader vs FileInputStream vs commons-io LineIterator
                      vs a "summary data only" approach.
src/python/           python.v1.py (naive single-process loop), python.v2.py
                      (multiprocessing import present but unused — still a plain loop).
baseline_count.py     Minimal line-counting baseline used for comparing against other impls.
build.sh, run.sh      See "Build & run" below.
writeup.md            Informal timing notes/results across versions (i5/i7 desktop timings).
readme.md             Task description + "lessons learnt" notes (e.g. "pypy is fucking slow").
```

## Build & run

There's no single unified build; `build.sh` and `run.sh` are hand-edited to select which
implementations to include (arrays like `GO=(...)`, `RUST_DIR=(...)` are commented in/out) —
check these files before assuming "the build" covers everything.

- `bash build.sh` — builds whichever entries are uncommented into `dist/`:
  - Go: `cd <dir> && go build -o go.bin`, copied to `dist/go.<basename>.bin`
  - Rust: `cd <dir> && cargo clean && cargo build --release`, binary copied to
    `dist/<binname>_<dirname>`
  - Python/Scala: just copied into `dist/` as scripts (no compilation)
- `bash run.sh` — runs `build.sh` first, then runs every binary/script in `dist/` against
  `~/data/misc/itcont.txt`, timing each with `time` and appending output to `log.log`. `.scala`
  files are run via `scala <file> <data>`; everything else is executed directly.
- Per-language manual commands, since `build.sh`/`run.sh` don't cover everything:
  - Go: `cd src/go/vN && go build -o go.bin && ./go.bin <path-to-itcont.txt>`
  - Rust: `cd src/rust/vN && cargo run --release -- <path-to-itcont.txt>`
  - Python: `python3 src/python/python.v1.py <path-to-itcont.txt>` (per user's global tool
    preference, use `uvx --with requests python script.py`-style invocation if a Python
    interpreter tool is needed, though these scripts have no third-party deps)
  - Scala: `scala src/scala/scala.v1.scala <path-to-itcont.txt>`
  - Java: `cd src/java/read-file-java && ./gradlew assemble`, then
    `java -cp ./build/libs/readFileJava-0.0.1-SNAPSHOT.jar com.example.readFile.readFileJava.<ClassName> [path-to-file]`
    (class names: `ReadFileJavaApplicationBufferedReader`, `ReadFileJavaApplicationFileInputStream`,
    `ReadFileJavaApplicationLineIterator`, `ReadFileJavaApplicationStoringOnlySummaryData`); omitting
    the path falls back to the small bundled `src/main/resources/config/test.txt`.
- `bash test.sh` is a placeholder (`echo 1`) — there is no real test suite in this repo.

## Notes on existing code

- Some directories contain committed build artifacts (e.g. `src/go/v0/go.bin`, `src/go/v0/large-file`,
  `src/go/v9_marc/v9_marc`) despite `.gitignore` excluding `**/dist`/`**/target`/`**/build` — these
  slipped through because the binary names themselves aren't covered by those patterns. Don't assume
  `.gitignore` keeps this repo binary-free.
- Concurrency patterns across versions are the interesting part of this codebase: chunked line
  batches handed to goroutines/threads, then merged either via a fan-in channel (Go v2), a shared
  mutex (Go v3, Rust v2), or `sync.Pool` + atomics (Go v9_marc). When adding a new attempt, follow
  the existing pattern of a new `vN` directory rather than modifying an older version in place —
  the versions are meant to stay as historical comparison points.
