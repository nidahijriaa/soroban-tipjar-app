default: build

all: test

test: build
	cargo test --workspace --features testutils

build:
	stellar contract build

fmt:
	cargo fmt --all

clean:
	cargo clean
	rm -f target/wasm32-unknown-unknown/release/*.wasm

deploy-testnet:
	stellar contract deploy \
		--wasm target/wasm32-unknown-unknown/release/soroban_tipjar_contract.wasm \
		--network testnet \
		--source alice

invoke-init:
	stellar contract invoke \
		--id $(CONTRACT_ID) \
		--network testnet \
		--source alice \
		-- initialize \
		--owner $(OWNER_ADDRESS)

invoke-tip:
	stellar contract invoke \
		--id $(CONTRACT_ID) \
		--network testnet \
		--source $(SOURCE) \
		-- send_tip \
		--from $(FROM) \
		--token_address $(TOKEN) \
		--amount $(AMOUNT) \
		--message "$(MESSAGE)"

get-total:
	stellar contract invoke \
		--id $(CONTRACT_ID) \
		--network testnet \
		--source alice \
		-- get_total

get-tips:
	stellar contract invoke \
		--id $(CONTRACT_ID) \
		--network testnet \
		--source alice \
		-- get_tips