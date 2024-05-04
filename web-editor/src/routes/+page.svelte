<script lang="ts" type="module">
	import { onDestroy, onMount } from 'svelte';
	import * as monaco from 'monaco-editor';
	import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
	import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker';

	import cssWorker from 'monaco-editor/esm/vs/language/css/css.worker?worker';
	import htmlWorker from 'monaco-editor/esm/vs/language/html/html.worker?worker';
	import tsWorker from 'monaco-editor/esm/vs/language/typescript/ts.worker?worker';
	import { clear, messages, stumbrsData } from '../lib/priede';
	import init, { run } from '../pkg/priede_wasm.js';

	let answer = '';

	let is_console_opened = true;

	let editorElement: HTMLDivElement;
	let editor: monaco.editor.IStandaloneCodeEditor;
	let model: monaco.editor.ITextModel;

	function loadCode(code: string, language: string) {
		model = monaco.editor.createModel(code, language);

		editor.setModel(model);
	}

	onMount(async () => {
		init();
		self.MonacoEnvironment = {
			getWorker: function (_: any, label: string) {
				//if (label === 'json') {
				//	return new jsonWorker();
				//}
				//if (label === 'css' || label === 'scss' || label === 'less') {
				//	return new cssWorker();
				//}
				//if (label === 'html' || label === 'handlebars' || label === 'razor') {
				//	return new htmlWorker();
				//}
				//if (label === 'typescript' || label === 'javascript') {
				//	return new tsWorker();
				//}
				return new editorWorker();
			}
		};

		monaco.languages.typescript.typescriptDefaults.setEagerModelSync(true);

		editor = monaco.editor.create(editorElement, {
			automaticLayout: true,
			theme: 'vs-dark'
		});

		loadCode('izvade("Sveika, pasaule!")', 'plaintext');
	});

	onDestroy(() => {
		monaco?.editor.getModels().forEach((model) => model.dispose());
		editor?.dispose();
	});

	let stumbrsExamples = [
		{ id: 1, text: `Cilvēka profils` },
		{ id: 2, text: `Filmas dati` },
		{ id: 3, text: `Apdzīvotas vietas dati` }
	];

	let selectedExample: any;

	function setExampleData(example: string) {
		console.log(example);
		
		switch (example) {
			case 'Cilvēka profils':
				stumbrsData.set(`
{
    teksts vārds
    skaitlis vecums
	teksts profesija
	teksts epasts
	teksts telefona_nummurs
}
{
    "Miķelis"
    2
	"programmētājs"
	"mikelis@mikelis.lv"
	"12345678"
}
				`);
				break;
			case 'Filmas dati':
				stumbrsData.set(`
{
    teksts nosaukums
    skaitlis iznākšanas_gads
	teksts režisors
	teksts zanrs
}
{
    "Limuzīns Jāņu naksts krāsā"
    1981
	"Jānis Streičs"
	"Drāma/Komēdija"
}
				`);
				break;
			case 'Apdzīvotas vietas dati':
				stumbrsData.set(`
{
    teksts nosaukums
    skaitlis iedzīvotāju skaits
}
{
    "Cēsis"
    14713
}
				`);
				break;
		}
	}
</script>

<div class="root">
	<div class="editor" bind:this={editorElement} />
	<div class="panel">
		{#if is_console_opened}
			<button
				class="runBtn"
				on:click={() => {
					init();
					clear();
					run(editor.getValue());
				}}
			>
				PALAIST
				<svg
					width="20"
					height="20"
					viewBox="0 0 15 15"
					fill="none"
					xmlns="http://www.w3.org/2000/svg"
				>
					<path
						d="M3.24182 2.32181C3.3919 2.23132 3.5784 2.22601 3.73338 2.30781L12.7334 7.05781C12.8974 7.14436 13 7.31457 13 7.5C13 7.68543 12.8974 7.85564 12.7334 7.94219L3.73338 12.6922C3.5784 12.774 3.3919 12.7687 3.24182 12.6782C3.09175 12.5877 3 12.4252 3 12.25V2.75C3 2.57476 3.09175 2.4123 3.24182 2.32181ZM4 3.57925V11.4207L11.4288 7.5L4 3.57925Z"
						fill="currentColor"
						fill-rule="evenodd"
						clip-rule="evenodd"
					></path>
				</svg>
			</button>
			<div class="console">
				{#each $messages as message, index}
					<div class="consoleMessage">
						{#if message.typ == 'out'}
							{message.text}
						{:else}
							<input bind:value={answer} style="margin: 5px;" type="text" />
							<button>-></button>
						{/if}
					</div>
				{/each}
			</div>
			<button
				class="switchBtn"
				on:click={() => {
					is_console_opened = false;
				}}>Pārslēgties uz Stumbra datiem!</button
			>
		{:else}
			<label for="examplePicker">Stumbra datu piemēri:</label>
			<select
				id="examplePicker"
				bind:value={selectedExample}
				on:change={() => {
					answer = '';
					setExampleData(selectedExample.text);
				}}
			>
				{#each stumbrsExamples as example}
					<option value={example}>
						{example.text}
					</option>
				{/each}
			</select>
			<textarea bind:value={$stumbrsData} name="stumbrs" id="stumbrsEditor"></textarea>
			<button
				class="switchBtn"
				on:click={() => {
					is_console_opened = true;
				}}>Pārslēgties uz konsoli!</button
			>
		{/if}
	</div>
</div>
