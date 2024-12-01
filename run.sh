release() {
  cargo build --release --package "$2"
  cp target/release/"$2" ./"$2_release"
  ./"$2"_release
}

debug() {
  cargo build --package "$2"
  cp target/debug/"$2" ./"$2_debug"
  ./"$2"_debug
}

bench() {
  debug "$@"
  release "$@"
  hyperfine --warmup 10 --shell=none ./"$2"_debug ./"$2"_release
}

clean() {
  rm "$2"_release "$2"_debug
}

if [[ "$1" == "r" ]]; then
  release "$@"
fi

if [[ "$1" == "d" ]]; then
  debug "$@"
fi

if [[ "$1" == "b" ]]; then
  bench "$@"
fi

if [[ "$1" == "c" ]]; then
  clean "$@"
fi
