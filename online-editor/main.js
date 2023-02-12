import init, { run } from "./pkg/priede_wasm.js";
import { parser } from "./gram.js";
import { foldNodeProp, foldInside, indentNodeProp } from "@codemirror/language";
import { styleTags, tags as t } from "@lezer/highlight";
import { LRLanguage } from "@codemirror/language";
import { LanguageSupport } from "@codemirror/language";
import { EditorView, basicSetup } from "codemirror";
import { EditorState, Compartment } from "@codemirror/state";

let parserWithMetadata = parser.configure({
  props: [
    styleTags({
      Identifier: t.variableName,
      Boolean: t.bool,
      String: t.string,
      LineComment: t.lineComment,
      sk: t.typeName,
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
    commentTokens: { line: "//" },
  },
});

function example() {
  return new LanguageSupport(exampleLanguage);
}
const languageConf = new Compartment();

const queryString = window.location.search;
const urlParams = new URLSearchParams(queryString);
const product = urlParams.get("preset");
let preset = "";
switch (product) {
  case "sveikapasaule":
    preset = 'drukāt("Sveika, pasaule!")';
    break;

  case "vars":
    preset =
      "būls a -> 0\nsk b -> 2 \nlsk c -> 2 \nnatsk d -> 2 \nlnatsk e -> 2 \nteksts f -> 'cav'\ndrukāt(a)";
    break;

  default:
    preset = 'drukāt("Sveika, pasaule!")';
    break;
}

const editor = new EditorView({
  doc: preset,
  extensions: [basicSetup, languageConf.of(example())],
  parent: document.querySelector("#code"),
});

init();
var btn = document.getElementById("btn");
btn.onclick = function () {
  let code = "";
  editor.state.doc.text.forEach((element) => {
    code = code + element + "\n";
  });
  run(code);
};

import { Terminal } from "xterm";
import "xterm/css/xterm.css";
var term = new Terminal();
term.open(document.getElementById("terminal"));

console.log = (...args) => {
  term.write(...args);
};
alert = (...args) => {
  term.writeln(...args);
};
