import { writable } from 'svelte/store';
export const messages = writable([]);
/**
 * @param {any} a
 */

export function wasm_print(a) {
	console.log(a);
	// @ts-ignore
	messages.update((currentData) => {
		return [{ typ: 'out', text: a }, ...currentData];
	});
}
export function clear() {
	messages.set([]);
}

export async function wasm_input() {
	// @ts-ignore
	messages.update((currentData) => {
		return [{ typ: 'in' }, ...currentData];
	});
	
	return "cav";

}
