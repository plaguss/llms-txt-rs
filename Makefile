sources = python/llms_txt tests

.PHONY: .uv  ## Check that uv is installed
.uv:
	@uv -V || echo 'Please install uv: https://docs.astral.sh/uv/getting-started/installation/'

.PHONY: .pre-commit  ## Check that pre-commit is installed
.pre-commit:
	@pre-commit -V || echo 'Please install pre-commit: https://pre-commit.com/'

.PHONY: install
install: .uv .pre-commit
	uv pip install -U wheel
	uv sync --frozen --group all
	uv pip install -v -e .
	pre-commit install

.PHONY: build-dev
build-dev:
	uv run maturin develop

.PHONY: build-prod
build-prod:
	uv maturin develop --release

.PHONY: format
format:
	uv run ruff check --fix $(sources)
	uv run ruff format $(sources)
	cargo fmt

.PHONY: lint-python
lint-python:
	uv run ruff check $(sources)
	uv run ruff format --check $(sources)

.PHONY: lint-rust
lint-rust:
	cargo fmt --version
	cargo fmt --all -- --check
	cargo clippy --version
	# cargo clippy --tests -- -D warnings

.PHONY: mypy
mypy:
	uv run mypy python/llms_txt_rs

.PHONY: lint
lint: lint-python lint-rust

.PHONY: test-python
test-python:
	uv run pytest

.PHONY: test-rust
test-rust:
	cargo test
