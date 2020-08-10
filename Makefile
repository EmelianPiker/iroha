.PHONY: debug release clean test

RUN := $(shell which nix-shell > /dev/null && echo 'nix-shell --run ' || echo 'sh -c ')

debug:
	${RUN} "cargo build"

release:
	${RUN} "cargo build --release"

clean:
	${RUN} "cargo clean"

test:
	${RUN} "cargo test"
