#!/usr/bin/env bash
CWD="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

if [[ ! -d "$CWD/dist" ]]; then
    mkdir "$CWD/dist"
fi

rm "$CWD/dist/"*

# build
## go
GO=(
     $CWD/src/go/v1
    )

for go_src in "${GO[@]}"; do
    echo "building $go_src"
    cd "$go_src" || exit
    go build -o go.bin
    cd "$CWD" || exit
    bname=$(basename "$go_src")
    cp "$go_src/go.bin" "dist/go.$bname.bin"
done

##
PY=(
    #  $CWD/src/python/python.v1.py
)

for py_src in "${PY[@]}"; do
    echo "building $py_src"
    cp "$py_src" dist/
done

## scala
SCALA=(
#    $CWD/src/scala/scala.v1.scala
)

for scala_script in "${SCALA[@]}"; do
    echo "building $scala_script"
    cp "$scala_script" dist/
done

## rust
RUST_DIR=(
    $CWD/src/rust/v1/
)
for rust_dir in "${RUST_DIR[@]}"; do
    echo "building $rust_dir"
    cd "$rust_dir"
    cargo build --release
    find target/release -perm +111 -depth 1 -type f | xargs -I {} cp {} $CWD/dist
    cd -
done
