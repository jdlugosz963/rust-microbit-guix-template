(use-modules (gnu packages embedded)
	     (rustup build toolchain))

(concatenate-manifests
 (list (packages->manifest
	(list (rustup #:targets (list "thumbv6m-none-eabi"))
	      (make-arm-none-eabi-toolchain-7-2018-q2-update)
	      (make-gdb-arm-none-eabi)))
       (specifications->manifest
	(list "gcc-toolchain"
	      "minicom"
	      "rust-analyzer"
	      "emacs"
	      "emacs-rust-mode"
	      "emacs-dap-mode"
	      "emacs-flycheck-rust"))))

