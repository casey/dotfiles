-- configure rust analyzer
require('lspconfig').rust_analyzer.setup({})

-- hide diagnostic messages
vim.lsp.handlers["textDocument/publishDiagnostics"] = vim.lsp.with(
  vim.lsp.diagnostic.on_publish_diagnostics,
  {
    virtual_text = false,
    signs = false,
    update_in_insert = false,
    underline = false,
  }
)
