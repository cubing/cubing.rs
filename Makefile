.PHONY: test
test:
	cargo test

.PHONY: publish
publish: lint test
	cargo publish

.PHONY: lint
lint:
	cargo clippy

.PHONY: format
format:
	cargo fmt

.PHONY: clean
clean:
	rm -rf ./target
