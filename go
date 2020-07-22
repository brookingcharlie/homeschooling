#!/usr/bin/env bash

docker_run() {
    docker run \
    --rm \
    -i \
    --user "$(id -u)":"$(id -g)" \
    -e USER=$USER \
    -v "$PWD":"/usr/src/myapp" \
    -v "$PWD/.cargo/cache":"/usr/local/cargo/cache" \
    -v "$PWD/.cargo/index":"/usr/local/cargo/index" \
    -v "$PWD/.cargo/registry":"/usr/local/cargo/registry" \
    -w /usr/src/myapp \
    rust:1.45.0 \
    $@
}

case "$1" in
  run) shift; docker_run cargo run --release --quiet $@ ;;
  test) shift; docker_run cargo test $@ ;;
  perf) shift; docker_run cargo test --release performance -- --ignored ;;
  clean) shift; docker_run cargo clean $@ ;;
  sh) shift; docker_run $@ ;;
  *)
    echo "Usage:"
    echo "./go run < input.txt: build and run application"
    echo "./go test: run unit tests"
    echo "./go perf: run perf test partitioning large input (tip: run twice since it builds first time)"
    echo "./go clean: remove build files"
    ;;
esac
