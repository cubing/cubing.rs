CD_TEST_SIMD = cd ./test/simd/

.PHONY: build
build:
	cargo build --release

.PHONY: test
test: test-nextext test-doctests test-simd

.PHONY: test-nextext
test-nextext:
	cargo tool-run-bin cargo-nextest run --all

.PHONY: test-doctests
test-doctests:
	@# `cargo-nextest` doesn't support doctests.
	cargo test --doc --all

.PHONY: test-simd
test-simd:
	${CD_TEST_SIMD} && cargo test

.PHONY: lint
lint:
	cargo clippy --workspace --all-targets -- --deny warnings
	cargo fmt --check
	${CD_TEST_SIMD} && cargo clippy --workspace --all-targets -- --deny warnings
	${CD_TEST_SIMD} && cargo fmt --check

.PHONY: format
format:
	cargo clippy --workspace --all-targets --fix --allow-dirty -- --deny warnings
	cargo fmt
	${CD_TEST_SIMD} && cargo clippy --workspace --all-targets --fix --allow-dirty -- --deny warnings
	${CD_TEST_SIMD} && cargo fmt

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
