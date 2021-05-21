if exists("did_load_filetypes")
  finish
endif

augroup filetypedetect
  autocmd!
  au BufNewFile,BufRead *.as                            setf actionscript
  au BufNewFile,BufRead *.glsl                          setf glsl
  au BufNewFile,BufRead *.go                            setf go
  au BufNewFile,BufRead *.go                            setlocal noet ts=2 sw=2 sts=2
  au BufNewFile,BufRead *.ino                           setf cpp
  au BufNewFile,BufRead *.lalrpop                       setf rust
  au BufNewFile,BufRead *.mm,*.m++                      setf objcpp
  au BufNewFile,BufRead *.s++                           setf cpp
  au BufNewFile,BufRead *.ts                            setf typescript
  au BufNewFile,BufRead *.yaml,*.yml                    setf yaml
  au BufNewFile,BufRead *.applescript                   setf applescript
  au BufNewFile,BufRead BUCK                            setf python
  au BufNewFile,BufRead SConstruct,sconstruct           setf python
  au BufNewFile,BufRead sconscript                      setf python
  au BufNewFile,BufRead ~/.zsh/functions/*              setf zsh
  au BufNewFile,BufRead ~/src/local/etc/zsh/functions/* setf zsh
  au BufNewFile,BufRead *                               call DetectCargoScript()
augroup END
