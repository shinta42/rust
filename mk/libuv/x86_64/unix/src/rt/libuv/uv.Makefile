# This file is generated by gyp; do not edit.

export builddir_name ?= mk/libuv/x86_64/unix/./src/rt/libuv/out
.PHONY: all
all:
	$(MAKE) -C ../../.. uv run-tests run-benchmarks
