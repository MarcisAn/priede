import { parser } from "./gram.js";
import {
  foldNodeProp,
  foldInside,
  indentNodeProp,
  language,
} from "@codemirror/language";
import { styleTags, tags as t } from "@lezer/highlight";
import { LanguageSupport } from "@codemirror/language";
import { LRLanguage } from "@codemirror/language";
import { basicSetup } from "codemirror";
import init, { run } from "./pkg/priede_wasm.js";
import { completeFromList } from "@codemirror/autocomplete";

init();
var btn = document.getElementById("btn");

let parserWithMetadata = parser.configure({
  props: [
    styleTags({
      Identifier: t.variableName,
      Boolean: t.bool,
      String: t.string,
      LineComment: t.lineComment,
      "( )": t.paren,
    }),
    indentNodeProp.add({
      Application: (context) =>
        context.column(context.node.from) + context.unit,
    }),
    foldNodeProp.add({
      Application: foldInside,
    }),
  ],
});
export const exampleLanguage = LRLanguage.define({
  parser: parserWithMetadata,
  languageData: {
    commentTokens: { line: ";" },
  },
});
export const exampleCompletion = exampleLanguage.data.of({
  autocomplete: completeFromList([
    { label: "defun", type: "keyword" },
    { label: "defvar", type: "keyword" },
    { label: "let", type: "keyword" },
    { label: "cons", type: "function" },
    { label: "car", type: "function" },
    { label: "cdr", type: "function" },
  ]),
});
export function console_log(a) {
  console.log(a);
}
export function example() {
  return new LanguageSupport(exampleLanguage, [exampleCompletion]);
}

export const editor = CodeMirror.fromTextArea(document.querySelector("#code"), {
  extensions: [basicSetup],
  lineNumbers: true,
  tabSize: 2,
  value: 'drukāt("cav")',
  theme: "ayu-dark",
});
//console.log(editor.options.value);
btn.onclick = function () {
  run(editor.getValue());
};

export function web_print(a) {
  console.log(a);
}
