function! airline#extensions#tabline#formatters#f#format(bufnr, buffers)
  return FormatBufferForTabLine(a:bufnr, a:buffers)
endfunction
