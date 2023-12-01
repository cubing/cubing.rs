.PHONY: test
test:
	cargo test

.PHONY: lint
lint:
	cargo clippy

.PHONY: format
format:
	cargo fmt

.PHONY: clean
clean:
	rm -rf ./target

.PHONY: publish
publish: lint # `cargo test` is run automatically.
	cargo publish --package cubing_core
	cargo publish --package cubing_macros
	cargo publish --package cubing
