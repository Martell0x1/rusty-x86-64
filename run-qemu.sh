#!/bin/sh
qemu-system-x86_64 \
  -serial stdio \
  -drive format=raw,file="$1"
