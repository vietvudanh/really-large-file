v1.python:
    54s on i5 desktop
    1m53.464s on i7-4890HQ

    no multiprocess, naive loops

scala.v1: 27s

go.v1: 18s

go.v2: 5.151s
    using goroutine, channel
    sending lines by chunk
    sending entry by chunk
    
rust.v1: 25s
    using thread

rust v1.1: 15s

rust v2: 11s
	using global mutable lock with Arc.

--- 2026-07-09 re-run: Apple M2 (4P+4E), 16GB RAM, SSD 1.71GB/s raw ---
file: itcont.txt 4.29GB / 21,730,731 lines

go v4:   2.80s  pread + work-stealing chunks + zero-alloc parse (new)
rust v3: 2.77s  same design, memchr + FxHashMap (new)
         both I/O-bound at ~90% of raw disk speed; mmap variant was
         6.5s at ~1 core (macOS serializes parallel page faults)
go v2:   3.80s   go v3: 4.29s   go v0 (count only): 3.39s
rust v2: 6.65s   rust v1: 10.47s
scala count-only: 5.48s   scala full: 20.92s
pypy3 count: 9.68s  python3 count: 14.28s  full python either: 27-31s
java (OpenJDK 26): summary-only 41.6s, others 69-100s
