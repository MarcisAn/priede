"use strict";

import * as vscode from "vscode";

export function activate(context) {

    vscode.commands.registerCommand("extension.helloWorld", () => {
      const { activeTextEditor } = vscode.window;

      if (
        activeTextEditor &&
        activeTextEditor.document.languageId === "priede"
      ) {
        const { document } = activeTextEditor;
        const firstLine = document.lineAt(0);

        if (firstLine.text !== "42") {
          const edit = new vscode.WorkspaceEdit();
          edit.insert(document.uri, firstLine.range.start, "42\n");

          return vscode.workspace.applyEdit(edit);
        }
      }
    });

  vscode.languages.registerDocumentFormattingEditProvider(
    {
      scheme: "file",
      language: "priede",
    },
    {
      provideDocumentFormattingEdits(
        document: vscode.TextDocument
      ) {
        const firstLine = document.lineAt(0);
        if (firstLine.text !== "42") {
          return [vscode.TextEdit.insert(firstLine.range.start, "42\n")];
        }
      },
    }
  );
}
