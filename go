#!/usr/bin/env bash

docker_run() {
    docker run --rm --user "$(id -u)":"$(id -g)" -e USER=$USER -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:1.45.0 $@
}

case "$1" in
  clean)
    docker_run cargo clean
    ;;
  build)
    docker_run cargo build
    ;;
  run)
    docker_run cargo run
    ;;
  test)
    docker_run cargo test
    ;;
  sh)
    shift
    docker_run $@
    ;;
  *)
    echo "Usage"
    echo "./go clean"
    echo "./go build"
    echo "./go run"
    ;;
esac
