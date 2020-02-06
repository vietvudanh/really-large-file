#!/usr/bin/env bash
CWD="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

rm $CWD/dist/*

# build
## go
GO=(
   $CWD/src/go/v1
    )

for go_src in ${GO[@]}; do
    echo "building $go_src"
    cd $go_src
    go build -o go.bin
    cd $CWD
    bname=$(basename $go_src)
    cp $go_src/go.bin dist/go.$bname.bin
done

## python
PY=(
   $CWD/src/python/python.v1.py
)

for py_src in ${PY[@]}; do
    echo "building $py_src"
    cp $py_src dist/
done

## scala
SCALA=(
  $CWD/src/scala/main.v1.scala
  )

for scala_src in ${SCALA[@]}; do
  echo "building $scala_src"
  cat $CWD/src/scala/run_template.sh | sed "s~SRC_NAME~$scala_src~g" > $CWD/dist/$(basename $scala_src).sh
done
