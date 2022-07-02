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

require('nvim-treesitter.configs').setup({
  ensure_installed = { "rust" },

  highlight = {
    enable = true,
  },

  incremental_selection = {
    enable = true,
    keymaps = {
      init_selection = "<cr>",
      node_incremental = "<cr>",
      scope_incremental = "<tab>",
      node_decremental = "<bs>",
    },
  },
})
