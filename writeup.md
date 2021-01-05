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
