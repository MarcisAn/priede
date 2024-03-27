import React, { useEffect, useRef } from "react";
import ReactDOM from "react-dom";
import init, { run } from "./pkg/priede_wasm.js";
import Editor from "@monaco-editor/react";

export function priede_print(a) {
  console.log(a);
}

export default function App() {
  const editorRef = useRef(null);

  function handleEditorDidMount(editor, monaco) {
    editorRef.current = editor;
  }

  function showValue() {
    //alert(editorRef.current.getValue());
    return editorRef.current.getValue();
  }

  useEffect(() => {
    init();
  }, []);
  return (
    <>
      <Editor
        height="100vh"
        width="70vw"
        theme="vs-dark"
        defaultValue="drukāt('Sveika pasaule')"
        onMount={handleEditorDidMount}
      />
      <div className="panel">
        <button
          className="runBtn"
          onClick={() => {
            run(showValue());
            console.log(showValue());
          }}>
          PALAIST
          <svg
            width="20"
            height="20"
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg">
            <path
              d="M3.24182 2.32181C3.3919 2.23132 3.5784 2.22601 3.73338 2.30781L12.7334 7.05781C12.8974 7.14436 13 7.31457 13 7.5C13 7.68543 12.8974 7.85564 12.7334 7.94219L3.73338 12.6922C3.5784 12.774 3.3919 12.7687 3.24182 12.6782C3.09175 12.5877 3 12.4252 3 12.25V2.75C3 2.57476 3.09175 2.4123 3.24182 2.32181ZM4 3.57925V11.4207L11.4288 7.5L4 3.57925Z"
              fill="currentColor"
              fill-rule="evenodd"
              clip-rule="evenodd"></path>
          </svg>
        </button>
        <div className="console">aa</div>
      </div>
    </>
  );
}
