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
  hyperfine --warmup 3 --shell=none ./"$2"_debug ./"$2"_release | tee "$2"/results.txt
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
