function fish_user_key_bindings
  fish_hybrid_key_bindings
  fzf_key_bindings
  bind -M insert -m default \cj force-repaint
  bind -M default -m default \cj force-repaint

  bind -M insert \cg fzf-cd-widget
  bind -M insert \cx fzf-history-widget
  bind -M insert \cz fzf-file-widget

  bind -M insert \ck selection-cut
  bind -M insert \cy selection-paste

  bind -M insert \eK clipboard-cut
  bind -M insert \eY clipboard-paste
end
