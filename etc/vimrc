"" <c-j> is best escape

cnoremap <c-j> <c-c>
inoremap <c-j> <esc>
nnoremap <c-j> <esc>
vnoremap <c-j> <esc>

"" plugins

filetype plugin on " turn on filetype plugins

call plug#begin()

" general purpose plugins

Plug 'AndrewRadev/splitjoin.vim'          " split/join single/multi line constructs
Plug 'airblade/vim-rooter'                " automatic cd to project root
Plug 'cargo-limit/cargo-limit'            " cargo-limit integration
Plug 'casey/reap.vim'                     " read, evaluate, annotate, and print python
Plug 'christoomey/vim-tmux-navigator'     " navigate between vim and tmux splits
Plug 'coderifous/textobj-word-column.vim' " column related text objects
Plug 'dense-analysis/ale'                 " async lint 'n' fix engine
Plug 'dstein64/vim-startuptime'           " profile startup time
Plug 'easymotion/vim-easymotion'          " jump to character
Plug 'editorconfig/editorconfig-vim'      " respect .editorconfig
Plug 'emonkak/vim-operator-sort'          " sort operator
Plug 'fidian/hexmode'                     " hex mode editing
Plug 'glts/vim-magnum'                    " bignums
Plug 'glts/vim-radical'                   " convert numbers between bases
Plug 'godlygeek/tabular'                  " align lines according to patterns
Plug 'haya14busa/incsearch.vim'           " improved incremental search
Plug 'hiphish/rainbow-delimiters.nvim'    " rainbow delimiters
Plug 'inside/vim-search-pulse'            " pulse search pattern
Plug 'jez/vim-superman'                   " use vim as a manpage pager
Plug 'junegunn/fzf.vim'                   " fuzzy searching using fzf
Plug 'junegunn/vim-emoji'                 " emoji autocomplete
Plug 'kana/vim-operator-user'             " allow user defined operators
Plug 'kana/vim-textobj-entire'            " text object for the entire buffer
Plug 'kana/vim-textobj-user'              " user defined text objects
Plug 'mattn/webapi-vim'                   " web functionality
Plug 'mbbill/undotree'                    " visualize undo history as tree
Plug 'mhinz/vim-grepper'                  " win at grep
Plug 'mrcjkb/rustaceanvim'                " rust support
Plug 'neovim/nvim-lspconfig'              " nvim language server client configurations
Plug 'nvim-lualine/lualine.nvim'          " status line
Plug 'nvim-tree/nvim-tree.lua'            " file tree
Plug 'nvim-tree/nvim-web-devicons'        " filetype icons
Plug 'romainl/vim-qf'                     " quickfix improvements
Plug 'romgrk/barbar.nvim'                 " bufferline
Plug 'saaguero/vim-textobj-pastedtext'    " text object for last pasted text
Plug 'tmux-plugins/vim-tmux'              " tmux configuration
Plug 'tmux-plugins/vim-tmux-focus-events' " recognize tmux focus events
Plug 'tommcdo/vim-exchange'               " exchange operator
Plug 'tpope/vim-abolish'                  " search and substitute word variations
Plug 'tpope/vim-characterize'             " extensive character info
Plug 'tpope/vim-commentary'               " comment and uncomment operations
Plug 'tpope/vim-eunuch'                   " unix helpers
Plug 'tpope/vim-fugitive'                 " git integration
Plug 'tpope/vim-repeat'                   " repeat support for plugins
Plug 'tpope/vim-rsi'                      " readline bindings
Plug 'tpope/vim-surround'                 " quoting and parenthesizing
Plug 'tpope/vim-unimpaired'               " pairs of bracket mappings
Plug 'tyru/open-browser.vim'              " open urls in browser
Plug 'vim-scripts/AnsiEsc.vim'            " draw ansi escape sequences as colors
Plug 'vim-scripts/Rename'                 " rename the current file
Plug 'vim-utils/vim-man'                  " improved man page viewing
Plug 'wellle/targets.vim'                 " additional text objects
Plug 'xolox/vim-misc'                     " misc funcionality for other plugins

" fuzzy finder
Plug 'nvim-lua/plenary.nvim'
Plug 'nvim-telescope/telescope.nvim', { 'branch': '0.1.x' }

" conditionally loaded plugins
Plug 'mattn/gist-vim',   {'on': 'Gist'    } " create gists
Plug 'tyru/capture.vim', {'on': 'Capture' } " capture command output in a buffer

" language specific plugins
Plug 'rust-lang/rust.vim'      " rust
Plug 'pangloss/vim-javascript' " javascript
Plug 'preservim/vim-markdown'  " markdown
Plug 'tikhomirov/vim-glsl'     " glsl

" wgsl syntax
Plug 'DingDean/wgsl.vim', {'branch': 'main'}

"  just support
Plug 'NoahTheDuke/vim-just', {'branch': 'main'}

" toml syntax
Plug 'cespare/vim-toml', {'branch': 'main'}

" open lazygit in editor
Plug 'kdheepak/lazygit.nvim', {'branch': 'main'}

" tree sitter
Plug 'nvim-treesitter/nvim-treesitter', {'do': ':TSUpdate'}

" show git hunks in gutter
Plug 'airblade/vim-gitgutter', {'branch': 'main'}

" lsp progress
Plug 'linrongbin16/lsp-progress.nvim', {'branch': 'main'}

" fzf binary and runtime path files
Plug 'junegunn/fzf', {'dir': '~/.fzf', 'do': './install --all'}

" git status icons
Plug 'lewis6991/gitsigns.nvim', { 'branch': 'main' }

" diagnostics list
Plug 'folke/trouble.nvim', { 'branch': 'main' }

" delay diagnostics when in insert mode
Plug 'https://gitlab.com/yorickpeterse/nvim-dd.git', {'branch': 'main'}

" catppuccin colorscheme
Plug 'catppuccin/nvim', { 'as': 'catppuccin', 'branch': 'main' }

call plug#end()

"" sources

source ~/.vim/functions.vim
" py3file ~/.vim.py

"" settings

set autoindent     " auto indent new lines
set autoread       " autoload unchanged files
set backup         " backup files
set breakindent    " show soft-wrapped text with leading indent
set expandtab      " use soft tabs
set hidden         " allow modified buffers in background
set hlsearch       " hilight previous search matches
set ignorecase     " ignore case in searches
set incsearch      " hilight search matches while typing
set lazyredraw     " don't redraw window when running untyped commands
set linebreak      " break on characters in 'breakat'
set nomodeline     " don't parse files for modelines
set ruler          " show position in file
set shiftround     " round < and > to multiples of shiftwidth
set showcmd        " show partial command and other useful stuff at bottom of screen
set showmatch      " highlight opening delimiters
set showmode       " show current mode
set smartcase      " don't ignore search case if it contains capital letters
set smartindent    " smart indenting on new line
set smarttab       " insert blanks according to shiftwidth
set splitbelow     " open horizontal splits below current buffer
set splitright     " open vertical splits to the right of current buffer
set notildeop      " make `~` toggle case of single character
set title          " set a window title
set ttyfast        " make updates smoother but use more characters
set wildmenu       " show completion menu

set background    =dark                  " use a dark background
set backspace     =indent,eol,start      " backspace kills everything
set backupdir     =~/.vim/backup         " backup directory location
set clipboard     =unnamed               " use the system clipboard
set completefunc  =emoji#complete        " auto complete emoji
set dictionary    =/usr/share/dict/words " dictionary completion mode words
set directory     =~/.vim/swap//         " swap directory location
set errorfile     =.errors.txt           " set error file name
set errorformat   =%f:%l:%c:%m           " set errorformat
set history       =10000                 " lines of history to remember
set laststatus    =2                     " always show statusline
set makeprg       =just                  " use just as the makeprg
set mouse         =a                     " enable mouse use in all modes
set numberwidth   =3                     " use a minimum of three spaces for numbers in gutter
set previewheight =30                    " set preview window height
set scrolloff     =5                     " keep three context lines above and below cursor
set shortmess     =actsI                 " shorten some messages
set sidescrolloff =3                     " keep three context columns left and right of cursor
set signcolumn    =auto                  " always show sign column
set spellcapcheck =                      " turn off checking for capitalized sentences
set spelllang     =en_us                 " use american english as the spellcheck language
set tabpagemax    =50                    " open a maximum of 30 tabs when vim is started with -p
set textwidth     =0                     " disable wrapping by default
set undolevels    =10000                 " remember last 10000 changes
set updatetime    =250                   " set update time to 250ms
set virtualedit   =block                 " visual block mode not confined to character boundaries
set whichwrap     =b,s,h,l,~,[,],<,>     " all movement keys wrap
set wildchar      =<Tab>                 " use tab for completions
set wildignore    =*.pyc                 " ignore python bytecode files
set wildmode      =list:longest          " list all matches and complete longest common string

set completeopt  =menuone " show the completion menu even when there is only one item
set completeopt +=longest " auto-complete the longest text common to all matches
set completeopt +=preview " show additional information in preview window

set formatoptions +=2 " use paragraph second line indent
set formatoptions +=j " merge comments when joining lines
set formatoptions +=q " allow formatting comments with `gq`
set formatoptions +=r " insert comment leader after <enter> in insert mode
set formatoptions +=t " auto-wrap at textwidth
set formatoptions -=a " disable automatic formatting of paragraphs
set formatoptions -=c " don't auto-wrap comments at textwidth
set formatoptions -=l " format long lines when inserting

set viminfo ='100  " previously edited files
set viminfo +=/10  " search pattern items
set viminfo +=f1   " save file marks
set viminfo +=h    " disable hlsearch
set viminfo +=s10  " maximum number of KB to save from a register

"" colors

if &t_Co > 2 || has("gui_running")
  syntax on
  set t_Co=256
  set termguicolors
  hi NonText ctermfg=11
  hi SpellBad cterm=underline
  hi SpellCap cterm=italic
  hi SpellLocal cterm=italic
  hi SpellRare cterm=italic
  hi link EasyMotionIncSearch     Search
  hi link EasyMotionMoveHL        Search
  hi link EasyMotionShade         Comment
  hi link EasyMotionTarget        ErrorMsg
  hi link EasyMotionTarget2First  Search
  hi link EasyMotionTarget2Second Search
  hi gitcommitSummary guifg=#55b5db
endif

" turn on line numbers on pairing machine
if $HOST == 'x'
  set number
  set norelativenumber
  let g:relative_numbers_off = 1
endif

" use the unnamedplus clipboard if supported
if has('unnamedplus')
  set clipboard=unnamedplus
endif

" use two space soft tabs
call Stops(2)

"" lettings

let c_no_curly_error = 1 " {} inside [] or () is not an error
let g:CargoLimitVerbosity = 2 " suppress cargo limit startup message
let g:eunuch_no_maps = 1 " disable auto-shebang
let g:gist_post_private = 1 " make gists private by default
let g:gitgutter_enabled = 0 " enable gitgutter by default
let g:hexmode_patterns = '*.wasm' " open wasm files in hexmode
let g:incsearch#auto_nohlsearch = 1 " turn off hlsearch
let g:investigate_use_dash = 1 " make investigate use dash on macos
let g:pastedtext_select_key = 'gl' " use gl for last pasted text mappings
let g:qf_bufname_or_text = 2 " filter only on quickfix text
let g:rust_recommended_style = 0 " use 2 space instead of 4 space tabs
let g:rustfmt_autosave = 0 " run rustfmt on save
let g:surround_indent = 1 " indent with '=' after surrounding
let g:tmux_navigator_no_mappings = 1 " disable default tmux navigator mappings
let g:trouble = 0 " enable trouble
let g:undotree_WindowLayout = 2 " put undotree on left with full width diff
let g:vim_markdown_auto_insert_bullets = 0 " don't insert list bullets
let g:vim_markdown_folding_disabled = 1 " disable folding in markdown
let g:vim_markdown_new_list_item_indent = 2 " make indent smaller
let g:vim_search_pulse_mode = 'pattern' " pulse pattern not line
let mapleader = "\<space>" " space is my leader

let g:EasyMotion_do_mapping = 0                             " disable default mappings
let g:EasyMotion_keys       = 'ASDGHKLQWERTYUIOPZXCVBNMFJ;' " targets
let g:EasyMotion_smartcase  = 1                             " smartcase for easymotion
let g:EasyMotion_use_upper  = 1                             " allow typing targets in lowercase

let g:grepper        = {}
let g:grepper.prompt = 1
let g:grepper.tools  = ['rg', 'git']

let g:rooter_resolve_links                          = 0         " follow symlinks
let g:rooter_change_directory_for_non_project_files = 'current' " autochdir if not a project file
let g:rooter_silent_chdir                           = 1         " not message when chdiring

let g:rooter_patterns = ['.git', '.git/', 'justfile', 'Cargo.toml'] " stop patterns

let g:ale_completion_enabled           = 0 " disable completion
let g:ale_fix_on_save                  = 1 " fix on save
let g:ale_lint_on_enter                = 0 " lint when entering a new file
let g:ale_lint_on_filetype_changed     = 0 " don't lint on filetype changed
let g:ale_lint_on_insert_leave         = 0 " don't lint when leaving insert mode
let g:ale_lint_on_save                 = 0 " lint on save
let g:ale_lint_on_text_changed         = 0 " don't lint when text is changed
let g:ale_rust_cargo_check_all_targets = 1 " run cargo check with all targets
let g:ale_set_highlights               = 0 " don't set highlights for lints
let g:ale_set_loclist                  = 1 " set loclist for lints
let g:ale_set_signs                    = 1 " set signs for lints

" ale fixers
let g:ale_fixers = {
\ '*': ['remove_trailing_lines', 'trim_whitespace'],
\ 'mail': ['remove_trailing_lines'],
\ 'rust': ['rustfmt'],
\}

" ale fixers
let g:ale_linters = { 'rust': ['rls'], 'fish': [] }

" cursor styling
let &t_SI.="\e[5 q" " cursor is blinking bar in insert mode
let &t_SR.="\e[4 q" " cursor is solid underscore in replace mode
let &t_EI.="\e[1 q" " cursor is blinking block in normal mode

"" abbreviations
cabbrev <expr> %% expand('%:p:h')

"" mappings

" recursive mappings
map  #  <plug>(incsearch-nohl-#)|             " incsearch: word under cursor backward
map  *  <plug>(incsearch-nohl-*)|             " incsearch: word under cursor forward
map  /  <plug>(incsearch-forward)|            " incsearch: search forward
map  ?  <plug>(incsearch-backward)|           " incsearch: search backward
map  N  <plug>(incsearch-nohl-N)<Plug>Pulse|  " incsearch: repeat last search backward
map  g# <plug>(incsearch-nohl-g#)<Plug>Pulse| " incsearch: word under cursor backward non-words
map  g* <plug>(incsearch-nohl-g*)<Plug>Pulse| " incsearch: word under cursor forward non-words
map  g/ <plug>(incsearch-stay)<Plug>Pulse|    " incsearch: forward without moving cursor
map  n  <plug>(incsearch-nohl-n)<Plug>Pulse|  " incsearch: repeat last search forward
map  s  <plug>(easymotion-s)|                 " easymotion
nmap K  <plug>(Man)|                          " look up man page for word under cursor
nmap gx <plug>(openbrowser-smart-search)|     " open URL under cursor
vmap gx <plug>(openbrowser-smart-search)|     " open URL under cursor

" repeat last motion
nmap <leader>. <plug>(easymotion-repeat)

" operators
map <leader>od :call OpenWiktionary()<cr>| " look up in wiktionary
map <leader>of gq|                         " fix/format operator
map <leader>og <plug>(GrepperOperator)|    " grepper operator
map <leader>or <plug>(quickrun-op)|        " run
map <leader>os <plug>(operator-sort)|      " sort operator

" buffers
nnoremap <silent> <c-h> <cmd>BufferPrevious<cr>
nnoremap <silent> <c-i> <cmd>BufferLast<cr>
nnoremap <silent> <c-l> <cmd>BufferNext<cr>
nnoremap <silent> <m-s> <cmd>BufferPickDelete<cr>
nnoremap <silent> S <cmd>BufferPick<cr>

" objects
" free g?:        byzjk
" free a? and i?: dfghijkmoqruvxyz

" non-recursive mappings
inoremap <c-s> <c-g>u<Esc>[s1z=`]a<c-g>u|              " fix last spelling error
inoremap <c-x><c-x> <c-x><c-o>                         " omnifunc completions
nnoremap <c-b> <c-o>|                                  " jump back
nnoremap <c-f> <c-i>|                                  " jump forward
nnoremap <c-k> :AllLines<cr>|                          " search lines
nnoremap <c-p> :Telescope find_files<cr>|              " search files
nnoremap <c-s> :call CorrectSpellingError()<cr>|       " correct spelling error or search symbols
nnoremap <leader> <nop>|                               " reserve for three-key mappings
nnoremap <leader>" :vsplit<cr>|                        " vertical split
nnoremap <leader>.. :cd ..<cr>|                        " go up one directory
nnoremap <leader><cr> :nohlsearch<cr>|                 " clear search highlights
nnoremap <leader><space> :wall<cr>|                    " save all changed buffers
nnoremap <leader>= :split<cr>|                         " horizontal split
nnoremap <leader>a# :Tabularize /#<cr>|                " align on #
nnoremap <leader>a: :Tabularize :<cr>|                 " align on :
nnoremap <leader>a<bar> :Tabularize \|<cr>|            " align on |
nnoremap <leader>a= :Tabularize =<cr>|                 " align on =
nnoremap <leader>aa :A<cr>|                            " alternate article
nnoremap <leader>al :SidewaysLeft<cr>|                 " move function argument left
nnoremap <leader>ar :SidewaysRight<cr>|                " move function argument right
nnoremap <leader>bd :bd<cr>|                           " buffer delete
nnoremap <leader>cx :!chmod u+x %<cr>|                 " chmod +x current file
nnoremap <leader>fe :call FixEmoji()<cr>|              " fix emoji
nnoremap <leader>ff gqae<cr>|                          " fix formatting
nnoremap <leader>fp gqip|                              " fix/format paragraph
nnoremap <leader>ft :retab<cr>|                        " fix tabs
nnoremap <leader>fw ::%s/\s\+$//e<cr>|                 " fix whitespace
nnoremap <leader>gc :!git commit -v<cr><cr>|           " git commit
nnoremap <leader>gd :!git diff<cr><cr>|                " git diff
nnoremap <leader>gg :LazyGit<cr>|                      " open lazygit
nnoremap <leader>gm :!git commit -av<cr><cr>|          " git mash
nnoremap <leader>gs :!git status<cr>|                  " git status
nnoremap <leader>hd :Capture digraphs<cr>|             " display digraphs
nnoremap <leader>hm :Capture map<cr>|                  " display mappings
nnoremap <leader>iO O<esc>|                            " insert line above cursor
nnoremap <leader>id "=Date()<cr>p|                     " insert date
nnoremap <leader>io o<esc>|                            " insert line below cursor
nnoremap <leader>jj :!just<cr>|                        " run just
nnoremap <leader>ls :source ~/.vim/session<cr>|        " load session
nnoremap <leader>oj o<esc>|                            " insert line below cursor
nnoremap <leader>ok O<esc>|                            " insert line above cursor
nnoremap <leader>op :silent !ope %<cr>|                " open current file
nnoremap <leader>pc :pwd<cr>|                          " print cwd
nnoremap <leader>pu :PlugUpdate<cr>|                   " plug update
nnoremap <leader>py :PyEval<cr>|                       " evaluate current line with python3
nnoremap <leader>qe :cq<cr>|                           " quit with error code
nnoremap <leader>qq :q<cr>|                            " quit
nnoremap <leader>ra :!cargo add <c-r><c-w><cr>|        " add crate name under cursor
nnoremap <leader>rb :Quickrun<cr>|                     " run buffer
nnoremap <leader>rd :!cargo doc --all --open<cr>|      " open docs
nnoremap <leader>rg :Grepper<cr>|                      " recursive grep
nnoremap <leader>rp Ipub(crate) <esc>|                 " change visibility to pub(crate)
nnoremap <leader>sb :Buffers<cr>|                      " search buffers
nnoremap <leader>sc :Commands<cr>|                     " search commands
nnoremap <leader>sf :Files<cr>|                        " search files
nnoremap <leader>sg :Commits<cr>|                      " search git
nnoremap <leader>sh :History:<cr>|                     " search command history
nnoremap <leader>sl :Lines<cr>|                        " lines in current buffer
nnoremap <leader>ss :call SynStack()<cr>|              " show highlight group names
nnoremap <leader>st :Tags<cr>|                         " search ctags
nnoremap <leader>sv :source $MYVIMRC<cr>:nohl<cr>|     " source ~/.vimrc
nnoremap <leader>ta :call ToggleALE()<cr>|             " toggle ale
nnoremap <leader>tb :set wrap!<cr>|                    " toggle line breaks
nnoremap <leader>tc :call ToggleColor()<cr>|           " toggle syntax highlighting
nnoremap <leader>tg :GitGutterToggle<cr>|              " toggle gitgutter
nnoremap <leader>th :Hexmode<cr>|                      " toggle hex mode
nnoremap <leader>ti :call ToggleInterface()<cr>|       " toggle interface elements
nnoremap <leader>tl :call ToggleLineWrap()<cr>|        " toggle line wrapping
nnoremap <leader>tn :call ToggleLineNumbers()<cr>|     " toggle line numbers
nnoremap <leader>tr :call ToggleRelativeNumbers()<cr>| " toggle line numbers
nnoremap <leader>ts :setlocal spell!<cr>|              " toggle spelling
nnoremap <leader>tu :UndotreeToggle<cr>|               " toggle undotree
nnoremap <leader>tw :call ToggleWhitespace()<cr>|      " toggle whitespace
nnoremap <leader>tz :call ToggleZoom()<cr>|            " toggle zoom
nnoremap <leader>wa :wa<cr>|                           " write all buffers
nnoremap <leader>wo <c-w>o|                            " close all windows but current
nnoremap <leader>zz :wqa<cr>|                          " save all files and exit
nnoremap <m-p> :FindRepoFiles<cr>|                     " search files in repo directory
nnoremap <silent> <c-q> :call Leave()<cr>|             " close window, close buffer or quit
nnoremap X D|                                          " cut until end of line
nnoremap Y y$|                                         " yank to end of line
nnoremap j gj|                                         " go down one screen line
nnoremap k gk|                                         " go up one screen line
vnoremap <leader>s :sort<cr>|                          " sort lines
vnoremap <leader>t: :Tabularize :<cr>                  " align on :
vnoremap <leader>t= :Tabularize =<cr>                  " align on =
vnoremap j gj|                                         " go to next screen line
vnoremap k gk|                                         " go to previous screen line
xnoremap x d|                                          " cut operation

" show documentation in preview window.
nnoremap <silent> K :call ShowDocumentation()<CR>

" edit named files
nnoremap <leader>ea :e ~/.zsh/aliases.zsh<cr>
nnoremap <leader>ee :e ~/.config/alacritty/alacritty.toml<cr>
nnoremap <leader>ef :e ~/.vim/functions.vim<cr>
nnoremap <leader>ej :e ~/.user.justfile<cr>
nnoremap <leader>el :e ~/.vim/lua/init.lua<cr>
nnoremap <leader>em :e ~/.mutt/muttrc<cr>
nnoremap <leader>en :e notes.md<cr>:BufferMoveStart<cr>
nnoremap <leader>es :e ~/.zshrc<cr>
nnoremap <leader>et :e ~/.tmux.conf<cr>
nnoremap <leader>ev :e ~/.vimrc<cr>

"" aliases

call Alias('Tab',  'Tabularize')
call Alias('Grep', 'Grepper'   )

"" digraphs

digraphs fH 9753|  " reversed horizontal fleuron
digraphs fh 10087| " horizontal fleuron
digraphs fv 10086| " vertical fleuron

"" commands

command! -bang -nargs=* AllLines
  \ call fzf#vim#grep(
  \   'rg --column --line-number --no-heading --color=always --smart-case -- '.shellescape(<q-args>),
  \   1,
  \   fzf#vim#with_preview({'options': ['--delimiter=:', '--nth=4..']}), <bang>0)

command! -nargs=+ -complete=command BufDo call BufDo(<q-args>)

command! -narg=* PyEval python3 EvaluateCurrentLine(<f-args>)

command! FindRepoFiles lua require('telescope.builtin').find_files({cwd = vim.fn.expand(vim.fn.system('git rev-parse --show-toplevel 2> /dev/null'):gsub('\n', ''))})

nnoremap <leader>fg :TelescopeGitRoot<CR>


"" autocommands

augroup vimrc
  autocmd!

  " jump to the last cursor position after loading a file
  autocmd BufReadPost * if line("'\"") > 0 && line("'\"") <= line("$") | exe "normal! g'\"" | endif

  " turn off relative numbers when focus is lost
  autocmd FocusLost * if &number | let g:restore_relativenumber = &relativenumber | set norelativenumber | endif

  " turn on relative numbers when focus is gained
  autocmd FocusGained * if &number && exists('g:restore_relativenumber') && g:restore_relativenumber | set relativenumber | endif

  " don't highlight trailing whitespace in python files
  autocmd FileType python syn clear pythonSpaceError

  " don't highlight trailing whitespace in markdown files
  autocmd FileType markdown syn clear mkdLineBreak

  " call custom function on VimEnter
  autocmd VimEnter * call VimEnter()

  " turn off misbehaving asciidoc plugin
  autocmd BufReadPost *.adoc set filetype=text

  " use local and stdlib rusty-tags.vi as rust source tags file
  autocmd BufRead *.rs setlocal tags=./.rusty-tags.vi;/,$RUST_SRC_PATH/.rusty-tags.vi

  " highlight suggested commit title length
  autocmd FileType gitcommit set colorcolumn=51

  " highlight suggested commit description length
  autocmd FileType gitcommit set colorcolumn+=73

  " close preview window when completion done
  autocmd CompleteDone * if pumvisible() == 0 | pclose | endif

  " disable indentation in HTML files
  autocmd FileType html,jinja.html setlocal noautoindent | setlocal nobreakindent | setlocal nosmartindent | setlocal indentexpr=

  " fix vim script user command syntax highlighting
  " (should be unnecessary when https://github.com/vim/vim/issues/6587 is fixed)
  autocmd Syntax vim syn match vimUsrCmd '^\s*\zs\u\%(\w*\)\@>(\@!'

  " close terminal when shell exits
  autocmd TermClose * execute 'bd!'

  " enter insert mode in terminal
  autocmd TermOpen * startinsert
augroup end

if g:trouble == 0
  map  -  <plug>(qf_qf_toggle_stay)| " toggle quickfix window
  nnoremap + :call QuickfixNext(0)<cr>| " go to next error
  nnoremap <leader>qc :call setqflist([])<cr>| " clear quickfix list
  nnoremap _ :call QuickfixNext(1)<cr>| " go to previous error
endif

" vi flavor specific configuration
if has('nvim')
  " set viminfo file name for neovim
  set viminfo +=n~/.config/nvim/viminfo

  " backup directory location
  set undodir =~/.config/nvim/undo

  " tmux navigator bindings
  nnoremap <silent> <m-h> :TmuxNavigateLeft<cr>
  nnoremap <silent> <m-k> :TmuxNavigateUp<cr>
  nnoremap <silent> <m-j> :TmuxNavigateDown<cr>
  nnoremap <silent> <m-l> :TmuxNavigateRight<cr>

  " lsp bindings
  nnoremap <silent> <m-a> <cmd>lua require('lspconfig').rust_analyzer.setup({})<cr>
  nnoremap <silent> <m-d> <cmd>lua vim.lsp.buf.definition()<cr>
  nnoremap <silent> <m-i> <cmd>lua vim.lsp.buf.type_definition()<cr>
  nnoremap <silent> <m-n> <cmd>lua vim.diagnostic.goto_next()<cr>
  " nnoremap <silent> <m-p> <cmd>lua vim.diagnostic.goto_prev()<cr>
  nnoremap <silent> <m-r> <cmd>lua vim.lsp.buf.references()<cr>
  nnoremap <silent> <m-t> <cmd>lua vim.lsp.buf.hover()<cr>
  nnoremap <silent> <m-R> <cmd>lua vim.lsp.buf.rename()<cr>

  " load lua configuration
  lua require('init')
else
  " settings
  set viminfo +=n~/.vim/viminfo " set viminfo file name
  set undodir =~/.vim/undo " backup directory location

  " tmux navigator bindings
  nnoremap <silent> h :TmuxNavigateLeft<cr>
  nnoremap <silent> k :TmuxNavigateUp<cr>
  nnoremap <silent> j :TmuxNavigateDown<cr>
  nnoremap <silent> l :TmuxNavigateRight<cr>
endif
