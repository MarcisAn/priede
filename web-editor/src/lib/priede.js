import { writable } from 'svelte/store';
export const messages = writable([]);
/**
 * @param {any} a
 */

export function wasm_print(a) {
	console.log(a);
	// @ts-ignore
	//alert(a);
	messages.update((currentData) => {
		return [{ typ: 'out', text: a }, ...currentData];
	});
}
export function clear() {
	messages.set([]);
}
export function wasm_input() {
	// @ts-ignore
	messages.update((currentData) => {
		return [{ typ: 'in' }, ...currentData];
	});
	setTimeout(() => {
		return 'cav';
	}, 2000);
}
export function reply_to_input(text) {}
