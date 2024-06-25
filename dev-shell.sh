#!/usr/bin/env -S guix shell -m manifest.scm -- bash

export PATH="$PATH:$(realpath ~/)/.cargo/bin"

if ! which probe-rs; then
    guix shell cmake make pkg-config eudev -- cargo install probe-rs-tools;
fi

nohup emacs &
bash

