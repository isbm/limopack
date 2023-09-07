#!/usr/bin/bash

for d in "vendor" ".cargo"; do
    if [ -d "$d" ]; then
	rm -rf $d
    fi
done

mkdir -p .cargo
cargo vendor > .cargo/config.toml

# Remove junk
for tgt in "winapi" "windows" "wasm" "wasm-bindgen" "android" "bsd" ; do
    find ./vendor -type d -wholename "*/$tgt*" -prune -exec rm -rf {}/src \; -exec mkdir -p {}/src \; -exec touch {}/src/lib.rs \; -exec rm -rf {}/lib \;
done
