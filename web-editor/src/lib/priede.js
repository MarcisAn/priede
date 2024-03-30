import { writable } from 'svelte/store';
export const messages = writable([]);
/**
 * @param {any} a
 */

export function wasm_print(a) {
	console.log(a);
	// @ts-ignore
	messages.update((currentData) => {
		return [a, ...currentData];
	});
}
