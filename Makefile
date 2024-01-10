.PHONY: build
build:
	cargo build --release

.PHONY: test
test:
	cargo bin cargo-nextest run --all
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

.PHONY: publish
publish: lint # `cargo test` is run automatically.
	cargo publish --package cubing_core
	cargo publish --package cubing_macros
	cargo publish --package cubing

.PHONY: bump-version-minor
bump-version-minor:
	cargo workspaces version --no-git-commit --exact minor

.PHONY: bump-version-patch
bump-version-patch:
	cargo workspaces version --no-git-commit --exact patch
