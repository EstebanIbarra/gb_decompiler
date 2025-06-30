@PHONY: rust perl dependencies download-gb-opcodes prepare-arm-opcodes setup dev-dependencies install-githooks dev-setup

clean:
	@rm -rf data

rust:
	@echo "🦀 Rust 🦀"
	@if command -v rustup >/dev/null 2>&1; then \
		echo "🦀 rustup found. Updating Rust toolchain..."; \
		rustup update; \
	else \
		echo "🦀 rustup not found. Installing Rust toolchain..."; \
		curl https://sh.rustup.rs -sSf | sh -s -- -y; \
		echo "⚠️ Please run 'source $$HOME/.cargo/env' or restart your shell to update your PATH."; \
	fi

perl:
	@echo "🐪 Perl 🐪"
	@if command -v perl >/dev/null 2>&1; then \
		echo "🐪 Perl found. Updating Perl..."; \
		if command -v apt-get >/dev/null 2>&1; then \
			sudo apt-get update && sudp apt-get install --only-upgrade -y perl; \
		elif command -v yum >/dev/null 2>&1; then \
			sudo yum update -y perl; \
		elif command -v brew >/dev/null 2>&1; then \
			brew update && brew upgrade perl; \
		else \
			echo "⚠️ No supported package manager detected. Please update Perl manually."; \
			exit 1; \
	  fi \
	else \
		echo "🐪 Perl not found. Installing Perl..."; \
		if command -v apt-get >/dev/null 2>&1; then \
			sudo apt-get update && sudp apt-get install -y perl; \
		elif command -v yum >/dev/null 2>&1; then \
			sudo yum install -y perl; \
		elif command -v brew >/dev/null 2>&1; then \
			brew update && brew install perl; \
		else \
			echo "⚠️ No supported package manager detected. Please install Perl manually."; \
			exit 1; \
	  fi \
	fi

dependencies:
	@cargo install cargo-tarpaulin
	@curl -L https://cpanmin.us | perl - --sudo App::cpanminus
	@cpanm Data::Dump
	@cpanm JSON::XS

download-gb-opcodes:
	@echo "Downloading GB Opcodes"
	@mkdir data
	@curl -L -o ./data/gb_opcodes.json https://gbdev.io/gb-opcodes/Opcodes.json

prepare-arm-opcodes:
	@echo "Preparing ARM Opcodes"
	@git clone https://github.com/MahdiSafsafi/opcodesDB.git
	@mkdir json
	@perl -I. -Ilib opcodesDB/aarch32.pl
	@python3 ./arm_opcode_parser.py
	@rm -rf json opcodesDB

setup: clean rust perl dependencies download-gb-opcodes prepare-arm-opcodes

dev-dependencies:
	@echo "🦀 Installing Rust dev dependencies..."
	@cargo install cargo-watch

install-githooks:
	@echo "Installing Git Hooks..."
	@mkdir -p .git/hooks
	@cp .githooks/* .git/hooks
	@chmod +x .git/hooks/*

dev-setup: setup dev-dependencies install-githooks

