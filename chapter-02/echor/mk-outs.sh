#!/bin/bash

__main() {
  unset -f __main

  local out_dir="tests/expected"

  [ ! -d "$out_dir" ] && mkdir -p "$out_dir"

  echo "Hello there" > "$out_dir/hello1.txt"
  echo "Hello"  "there" > "$out_dir/hello2.txt"
  echo -n "Hello  there" > "$out_dir/hello1.n.txt"
  echo -n "Hello" "there" > "$out_dir/hello2.n.txt"
}
__main "$@"
