import { writable, get } from 'svelte/store';
export const messages = writable([]);
export const stumbrsData = writable("");
/**
 * @param {any} a
 */

export function wasm_print(a) {
	console.log(a);
	// @ts-ignore
	messages.update((currentData) => {
		return [...currentData, { typ: 'out', text: a } ];
	});
}
/**
 * @param {any} a
 * @param {any} line
 * @param {any} col
 * @param {any} span
 */
export function code_replace(a, line, col, span) {
	console.log(a, line, col, span);
	// @ts-ignore
	// messages.update((currentData) => {
	// 	return [{ typ: 'out', text: a }, ...currentData];
	// });
}
export function clear() {
	messages.set([]);
}
export function get_stumbrs_data() {
	return get(stumbrsData);
}

export async function wasm_input() {
	// @ts-ignore
	messages.update((currentData) => {
		return [{ typ: 'in' }, ...currentData];
	});
	
	return "cav";

}
