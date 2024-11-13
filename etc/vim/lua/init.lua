vim.g.loaded_netrw = 1
vim.g.loaded_netrwPlugin = 1

require'barbar'.setup {
  auto_hide = 1,
  animation = false,
  icons = {
    filetype = {
      enabled = false,
    },
  },
}

vim.g.rustaceanvim = {
  server = {
    default_settings = {
      ['rust-analyzer'] = {
        cargo = {
          buildScripts = {
            enable = true,
          },
        },
        checkOnSave = {
          enable = true,
        },
        diagnostics = {
          enable = false,
        },
      },
    },
  },
}

if vim.g.trouble ~= 0 then
  local trouble = require'trouble'

  trouble.setup {
    auto_close = false,
    auto_open = true,
    height = 8,
    multiline = false,
  }

  vim.keymap.set('n', '-',
    function()
      trouble.toggle()
    end
  )

  vim.keymap.set('n', '+',
    function()
      if #vim.diagnostic.get() == 0 then
        return
      end

      if not trouble.is_open() then
        trouble.open()
      else
        trouble.next({skip_groups = true, jump = true});
      end
    end
  )

  vim.keymap.set('n', '_',
    function()
      if #vim.diagnostic.get() == 0 then
        return
      end

      if not trouble.is_open() then
        trouble.open()
      else
        trouble.previous({skip_groups = true, jump = true});
      end
    end
  )
end


vim.lsp.handlers['textDocument/publishDiagnostics'] = vim.lsp.with(
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

require'catppuccin'.setup {
  custom_highlights = function(colors)
    return {
      BufferChanged = { fg = colors.yellow },
      BufferCurrent = { fg = colors.blue },
      BufferCurrentSign = { fg = colors.blue, bg = colors.grey },
      EndOfBuffer = { fg = colors.blue },
    }
  end
}

vim.cmd.colorscheme 'catppuccin-mocha'

local colors = require'catppuccin.palettes.mocha'

require'lualine'.setup {
  options = {
    icons_enabled = false,
  },
  sections = {
    lualine_b = {
      'branch',
      require'lsp-progress'.progress,
      { quickfix, color = { fg = colors.red } },
    },
  },
}


require'lspconfig'.clangd.setup { }

require'lsp-progress'.setup {
  client_format = function(client_name, spinner, series_messages)
    if #series_messages > 0 then
      return spinner
    end
  end,
}

require'nvim-tree'.setup()

require'nvim-treesitter.configs'.setup {
  ensure_installed = { 'lua', 'rust', 'vim', 'vimdoc' },
  highlight = {
    enable = true,
    disable = { 'vim', 'vimdoc' }
  },
  incremental_selection = {
    enable = true,
    keymaps = {
      init_selection = '<cr>',
      node_incremental = '<cr>',
      scope_incremental = '<tab>',
      node_decremental = '<bs>',
    },
  },
}

require'telescope'.setup {
  defaults = {
    layout_config = {
      flex = {
        flip_columns = 180,
      },
    },
  },
  pickers = {
    find_files = {
      layout_strategy = 'flex',
      disable_devicons = true,
    },
  },
}

vim.api.nvim_create_augroup('cmdwin_treesitter', {
  clear = true,
})

vim.api.nvim_create_autocmd('CmdwinEnter', {
  pattern = '*',
  command = 'TSBufDisable incremental_selection',
  group = 'cmdwin_treesitter',
  desc = "Disable treesitter's incremental selection in Command-line window",
})
