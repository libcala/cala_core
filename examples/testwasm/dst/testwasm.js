let wasm;
let rust;
let utf8 = new TextDecoder('utf-8');
function say(size, data) {
	var buffer = new Uint8Array(rust.buffer, data, size);
	console.info(utf8.decode(buffer));
};
export function start() { wasm.start(); }
export function wake(promise, result) { wasm.wake(promise, result); }
async function load(mod, imports) {
	if('instantiateStreaming' in window.WebAssembly) {
		return await WebAssembly.instantiateStreaming(mod, imports);
	} else if(typeof Response === 'function' && mod instanceof Response) {
		const bytes = await mod.arrayBuffer();
		return await WebAssembly.instantiate(bytes, imports);
	} else {
		const instance = await WebAssembly.instantiate(mod, imports);
		if(instance instanceof WebAssembly.Instance) {
			return { instance, mod };
		} else {
			return instance;
		}
	}
}
async function init(input) {
	input = fetch(import.meta.url.replace(/\.js$/, '.wasm'));
	const imports = { env: {
		say: say
	} };
	const { instance, mod } = await load(await input, imports);
	wasm = instance.exports;
	rust = instance.exports.memory;
	init.__wbindgen_wasm_mod = mod;
	return wasm;
}
export default init;
