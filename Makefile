test-wasm:
	wasm-pack test --node --chrome # use tests/*.rs

test:
	cargo test # use src/*.rs

build:
	wasm-pack build 
	# sed -i "" "s/\* as //" pkg/rs_algos_bg.js
	# echo "export { wasm }" >> pkg/rs_algos_bg.js
	rm pkg/.gitignore