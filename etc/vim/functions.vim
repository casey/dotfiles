" create an alias
function! Alias(from, to)
  execute 'cnoreabbrev <expr> ' . a:from
    \ .' ((getcmdtype() is# ":" && getcmdline() is# "'.a:from.'")'
    \ .'? ("'. a:to .'") : ("'. a:from .'"))'
endfunction

" execute <command> in all buffers and return to original buffer
function! BufDo(command)
  let current=bufnr("%")
  execute 'bufdo ' . a:command
  execute 'buffer ' . current
endfunction

" format a buffer name for the tabline
function! FormatBufferForTabLine(bufnr, buffers)
  if getbufvar(a:bufnr, '&mod')
    let l:mod = '+'
  else
    let l:mod = ' '
  endif

  let full = fnamemodify(bufname(a:bufnr), ':.')
  let file = fnamemodify(l:full, ':t')
  if l:file == 'mod.rs' || l:file == 'up.sql' || l:file == 'down.sql'
    let dir = fnamemodify(l:full, ':h')
    if l:dir != '.'
      return fnamemodify(l:dir, ':t') . '/' . l:file . l:mod
    endif
  endif
  return l:file . l:mod
endfunction

" current date as string
function! Date()
  let s = strftime('%FT%T%z')
  let a = s[:len(s)-3]
  let b = s[len(s)-2:]
  return a . b
endfunction

function! ServerStart()
  let escaped_path = substitute(trim(getcwd(), "/") , "/", "%", "g")
  let g:server_pipe_path = $TMPDIR . "nvim-" . escaped_path

  if filereadable(g:server_pipe_path)
    return
  endif

  call serverstart(g:server_pipe_path)
endfunction

function! Quickfix(previous)
  if !filereadable('.errors.txt')
    echo 'Error file not found.'
    return
  endif

  if !exists("s:errorfile_hash")
    let s:errorfile_hash = ""
  endif

  let l:errorfile_hash = system('cat .errors.txt | openssl dgst -sha256')

  if l:errorfile_hash != s:errorfile_hash
    execute 'cfile'
    let s:errorfile_hash = l:errorfile_hash
    return
  endif

  let l:quickfix_window_is_open = 0

  for n in range(1, winnr('$'))
    if getwinvar(n, '&syntax') == 'qf'
      let l:quickfix_window_is_open = 1
    endif
  endfor

  if !l:quickfix_window_is_open
    execute "normal \<plug>(qf_qf_toggle_stay)"
    execute 'cc 1'
  else
    if a:previous
      execute "normal \<plug>(qf_qf_previous)"
    else
      execute "normal \<plug>(qf_qf_next)"
    endif
  endif
endfunction

" toggle syntax highlighting
function! ToggleColor()
  if exists('g:syntax_on')
    syntax off
    highlight NonText ctermfg=11
  else
    syntax on
    highlight NonText ctermfg=11
  endif
endfun

" toggle visible whitespace and color column
function! ToggleWhitespace()
  if &colorcolumn != 0
    set colorcolumn=0
    set nolist
  else
    set colorcolumn=51,73,80,101
    set list
  endif
endfun

" toggle line numbers
function! ToggleLineNumbers()
  bufdo set number!         " toggle line numbers
  bufdo set relativenumber! " toggle relative line numbers
endfun

function! ToggleRelativeNumbers()
  BufDo set relativenumber! " toggle relative line numbers

  if !&relativenumber
    let g:relative_numbers_off = 1
  else
    let g:relative_numbers_off = 0
  endif
endfun

let s:interface = 0
function! ToggleInterface()
    if s:interface == 0
        let s:interface = 1
        set noshowmode
        set noruler
        set laststatus=0
        set noshowcmd
    else
        let s:interface = 0
        set showmode
        set ruler
        set laststatus=2
        set showcmd
    endif
endfunction

" toggle line wrapping
function! ToggleLineWrap()
  if &textwidth != 0
    set textwidth=0
  else
    set textwidth=79
  endif
endfun

" toggle rustfmt autosave
function! ToggleALE()
  if g:ale_enabled
    let g:ale_fix_on_save  = 0
    let g:ale_lint_on_save = 0
    let g:ale_enabled      = 0
    echo 'ALE disabled'
  else
    let g:ale_fix_on_save  = 1
    let g:ale_lint_on_save = 1
    let g:ale_enabled      = 1
    echo 'ALE enabled'
  endif
endfun

function! Stops(n)
  if a:n == 0
    set noexpandtab
  else
    set expandtab
    execute "set shiftwidth  =".a:n
    execute "set softtabstop =".a:n
    execute "set tabstop     =".a:n
  endif
endfunction

" get number of listed buffers
function! ListedBufferCount()
  let cnt = 0
  for nr in range(1,bufnr('$'))
    if buflisted(nr) && ! empty(bufname(nr))
      let cnt += 1
    endif
  endfor
  return cnt
endfunction

" close window, close buffer, or quit
function! Leave()
  let window_count = 0
  windo let window_count = window_count + 1
  if window_count > 1
    :q
    return
  endif

  if  ListedBufferCount() <= 1
    if &modified
      echo 'Unsaved changes!'
      return
    endif
    :qall
  else
    :bd
  endif
endfunction

" check command status of last shell command
function! CheckCommandStatus()
  if v:shell_error != 0
    echo 'Command failed: ' . v:shell_error
  endif
endfunction

" executed on VimEnter event
function! VimEnter()
  " take <c-j> back from incsearch.vim
  IncSearchNoreMap <c-j> <c-c><esc>

  " don't display line numbers in man pages
  if get(b:, 'current_syntax', '') == "man"
    " turn off airline
    AirlineToggle

    " turn off line numbers
    setlocal nonumber

    " turn off relative line numbers
    setlocal norelativenumber

    " hide status bar
    setlocal laststatus=0
  endif
endfunction

" replace :name: with emoji
function! FixEmoji()
  s/:\([^:]\+\):/\=emoji#for(submatch(1), submatch(0))/g
endfunction

" kill from cursor to end of command line
function! KillCmdline()
  return strpart(getcmdline(),0,getcmdpos()-1)
endfunction

function! Selection()
  let [line_start, column_start] = getpos("'<")[1:2]
  let [line_end, column_end] = getpos("'>")[1:2]
  let lines = getline(line_start, line_end)
  if len(lines) == 0
      return ''
  endif
  let lines[-1] = lines[-1][: column_end - (&selection == 'inclusive' ? 1 : 2)]
  let lines[0] = lines[0][column_start - 1:]
  return join(lines, "\n")
endfunction

function! ShowDocumentation()
  if (index(['vim','help'], &filetype) >= 0)
    execute 'h '.expand('<cword>')
  endif
endfunction

function! OpenWiktionary()
  call system('xdg-open "https://en.wiktionary.org/wiki/'.Selection().'"')
endfunction

function! DetectCargoScript()
  if getline(1) == '#!/usr/bin/env run-cargo-script'
    set ft=rust
  endif
endfun

function! CorrectSpellingError()
  execute 'normal! [s1z=<c-o>'
endfun

let s:zoom = 0
function! ToggleZoom()
    if s:zoom  == 0
        let s:zoom = 1
        set laststatus=0
        set noruler
        set noshowcmd
        set noshowmode
        set signcolumn=no
    else
        let s:zoom = 0
        set laststatus=2
        set ruler
        set showcmd
        set showmode
        set signcolumn=yes
    endif
endfunction
