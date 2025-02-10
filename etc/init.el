(defun display-startup-echo-area-message () (message "")) ; disable startup message
(setq evil-want-keybinding nil) ; fix mysterious evil-collection issue
(setq inhibit-startup-screen t) ; disable startup screen
(setq initial-scratch-message "") ; disable scratch buffer message

(require 'package)
(require 'use-package)

; add melpa to package archives
(add-to-list 'package-archives '("melpa" . "https://melpa.org/packages/") t)

; load packages
(package-initialize)

; packages
(use-package evil :ensure t)
(use-package evil-collection :ensure t)
(use-package magit :ensure t)

(evil-collection-init) ; additional evil mode bindings
(evil-mode 1) ; evil mode
(evil-set-undo-system 'undo-redo) ; evil mode undo system
(menu-bar-mode -2) ; disable menu bar
