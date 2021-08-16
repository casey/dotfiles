if exists("did_load_filetypes")
  finish
endif

augroup filetypedetect
  autocmd!
  autocmd BufNewFile,BufRead *                                  call DetectCargoScript()
  autocmd BufNewFile,BufRead *.applescript                      setf applescript
  autocmd BufNewFile,BufRead *.as                               setf actionscript
  autocmd BufNewFile,BufRead *.glsl                             setf glsl
  autocmd BufNewFile,BufRead *.go                               setf go
  autocmd BufNewFile,BufRead *.go                               setlocal noet ts=2 sw=2 sts=2
  autocmd BufNewFile,BufRead *.ino                              setf cpp
  autocmd BufNewFile,BufRead *.justfile                         setf just
  autocmd BufNewFile,BufRead *.lalrpop                          setf rust
  autocmd BufNewFile,BufRead *.mm,*.m++                         setf objcpp
  autocmd BufNewFile,BufRead *.s++                              setf cpp
  autocmd BufNewFile,BufRead *.ts                               setf typescript
  autocmd BufNewFile,BufRead *.yaml,*.yml                       setf yaml
  autocmd BufNewFile,BufRead BUCK                               setf python
  autocmd BufNewFile,BufRead SConstruct,sconstruct              setf python
  autocmd BufNewFile,BufRead Vagrantfile                        setf ruby
  autocmd BufNewFile,BufRead gitconfig                          setf gitconfig
  autocmd BufNewFile,BufRead sconscript                         setf python
  autocmd BufNewFile,BufRead ~/.zsh/functions/*                 setf zsh
  autocmd BufNewFile,BufRead ~/src/dotfiles/etc/zsh/functions/* setf zsh
augroup END
