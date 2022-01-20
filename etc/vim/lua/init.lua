-- configure rust analyzer
require('lspconfig').rust_analyzer.setup({
  settings = {
    ["rust-analyzer"] = {
      diagnostics = { disabled = { 'inactive-code' } },
    },
  }
})
