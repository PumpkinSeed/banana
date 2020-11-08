demo:
	secretcli tx compute store /root/code/target/wasm32-unknown/unknown/release/banana.wasm
	secretcli tx compute instantiate 1 "{}" --from a --label "banana-test" -y --keyring-backend test
	secretcli query compute list-contract-by-code 1 # list the contracts save the address to the CONTRACT
	secretcli tx compute execute $CONTRACT "{\"amount\": 300}" --from a --keyring-backend test

all:
	RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown
	cp ./target/wasm32-unknown-unknown/release/*.wasm ./contract.wasm
	## The following line is not necessary, may work only on linux (extra size optimization)
	# wasm-opt -Os ./contract.wasm -o ./contract.wasm
	cat ./contract.wasm | gzip -9 > ./contract.wasm.gz

clean:
	cargo clean
	-rm -f ./contract.wasm ./contract.wasm.gz
