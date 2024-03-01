vim.g.loaded_netrw = 1
vim.g.loaded_netrwPlugin = 1

require'barbar'.setup {
  auto_hide = 1,
  animation = false,
}

require'lspconfig'.rust_analyzer.setup {
  settings = {
    ["rust-analyzer"] = {
      cargo = {
        buildScripts = {
          enable = false,
        },
      },
      checkOnSave = {
        enable = false,
      },
    }
  }
}

vim.lsp.handlers["textDocument/publishDiagnostics"] = vim.lsp.with(
  vim.lsp.diagnostic.on_publish_diagnostics,
  {
    virtual_text = false,
    signs = false,
    update_in_insert = false,
    underline = false,
  }
)

local quickfix = function()
  local len = vim.api.nvim_eval('len(getqflist())')
  if len > 0 then
    return len
  else
    return ''
  end
end

require'trouble'.setup()

require'onedarkpro'.setup {
  highlights = {
    EndOfBuffer = { fg = { onedark_dark = "${blue}" } }
  }
}

vim.cmd('colorscheme onedark_dark')

local colors = require'onedarkpro.helpers'.get_colors('onedark_dark')

local lualine_theme = require'lualine.themes.onedark_dark'

lualine_theme.insert.c = lualine_theme.normal.c
lualine_theme.normal.c = nil

local insert = lualine_theme.insert
local normal = lualine_theme.normal
local visual = lualine_theme.visual

lualine_theme.insert = visual
lualine_theme.normal = insert
lualine_theme.visual = normal

require'lualine'.setup {
  options = {
    icons_enabled = false,
    theme = lualine_theme,
  },
  sections = {
    lualine_b = {
      'branch',
      require('lsp-progress').progress,
      { quickfix, color = { fg = colors.red } },
      'diagnostics',
    },
  },
}

require'lsp-progress'.setup {
  client_format = function(client_name, spinner, series_messages)
    if #series_messages > 0 then
      return spinner
    end
  end,
}

require'nvim-tree'.setup()

require'nvim-treesitter.configs'.setup {
  ensure_installed = { "lua", "rust", "vim", "vimdoc" },
  highlight = {
    enable = true,
    disable = { "vim", "vimdoc" }
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
}

vim.api.nvim_create_augroup("cmdwin_treesitter", {
  clear = true,
})

vim.api.nvim_create_autocmd("CmdwinEnter", {
  pattern = "*",
  command = "TSBufDisable incremental_selection",
  group = "cmdwin_treesitter",
  desc = "Disable treesitter's incremental selection in Command-line window",
})
