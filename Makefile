.PHONY: build check ci clean fmt install link lint release run test

BIN_NAME = marcup
CARGO = $(shell which cargo)

build:
	@$(CARGO) build

check:
	$(CARGO) check --release

ci: lint build test
	@echo "Everything's OK 🤘"

clean:
	rm -rf ./target

clippy:
	@$(CARGO) clippy

fmt: format

format:
	@$(CARGO) fmt

install:
	@cp ./target/release/$(BIN_NAME) /usr/local/bin/$(BIN_NAME)

link:
	@ln -sf ./target/debug/$(BIN_NAME) .

lint:
	@$(CARGO) fmt --all -- --check
	@echo "Lint OK 👌"

package:
	@$(CARGO) package --allow-dirty

publish:
	@$(CARGO) publish

release:
	@$(CARGO) build --release

test:
	@$(CARGO) test -- --nocapture && echo "Tests OK 👌"

testbt:
    @RUST_BACKTRACE=1 $(CARGO) test -- --nocapture && echo "Tests OK 👌"
