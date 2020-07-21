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
  clean) docker_run cargo clean ;;
  build) docker_run cargo build ;;
  test) docker_run cargo test ;;
  run) docker_run cargo run ;;
  sh) shift docker_run $@ ;;
  *)
    echo "Usage:"
    echo "./go clean"
    echo "./go build"
    echo "./go test"
    echo "./go run < input.txt"
    ;;
esac
