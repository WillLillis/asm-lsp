// @ts-check
const { LanguageClient } = require("vscode-languageclient/node");
const tmpdir = require("os").tmpdir();

module.exports = {
  /** @param {import("vscode").ExtensionContext} context*/
  activate(context) {
    /** @type {import("vscode-languageclient/node").ServerOptions} */
    const serverOptions = {
      run: {
        command: "asm-lsp",
      },
      debug: {
        command: "asm-lsp",
      },
    };

    /** @type {import("vscode-languageclient/node").LanguageClientOptions} */
    const clientOptions = {
      documentSelector: [{ scheme: "file", language: "lldb.dissassembly" }],
    };

    const client = new LanguageClient(
      "asm-lsp",
      "Assembly Language Server",
      serverOptions,
      clientOptions
    );

    client.start();
  },
};
