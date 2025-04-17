.PHONY: build
build:
	cargo build --release

.PHONY: test
test: test-nextext test-doctests

.PHONY: test-nextext
test-nextext:
	cargo tool-run-bin cargo-nextest run --all

.PHONY: test-doctests
test-doctests:
	@# `cargo-nextest` doesn't support doctests.
	cargo test --doc --all

.PHONY: lint
lint:
	cargo clippy

.PHONY: format
format:
	cargo fmt

.PHONY: clean
clean:
	rm -rf ./target

.PHONY: reset
reset: clean
	rm -rf ./.bin

.PHONY: publish
publish: lint # `cargo test` is run automatically.
	cargo publish --package cubing_core
	cargo publish --package cubing_macros
	cargo publish --package cubing
	cargo publish --package alg-cli

.PHONY: bump-version-minor
bump-version-minor:
	cargo tool-run-bin cargo-workspaces version --no-git-commit --exact minor

.PHONY: bump-version-patch
bump-version-patch:
	cargo tool-run-bin cargo-workspaces version --no-git-commit --exact patch
