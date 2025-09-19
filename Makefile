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
	cargo clippy -- --deny warnings
	cargo fmt --check

.PHONY: format
format:
	cargo clippy --fix --allow-no-vcs
	cargo fmt

.PHONY: clean
clean:
	rm -rf ./target

.PHONY: reset
reset: clean
	rm -rf ./.bin

.PHONY: publish
publish: lint # `cargo test` is run automatically.
	cargo publish --workspace

.PHONY: bump-version-minor
bump-version-minor:
	cargo tool-run-bin cargo-workspaces version --no-git-commit --exact minor

.PHONY: bump-version-patch
bump-version-patch:
	cargo tool-run-bin cargo-workspaces version --no-git-commit --exact patch
