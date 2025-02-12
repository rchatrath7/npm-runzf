# npm-runzf

Build cargo package 
```
cargo build --release
cp target/release/npm-workspace-complete ~/.local/bin/  # or somewhere in your PATH
```

Add `completions/_npm` to your zsh completions, `~/.config/zsh/completions/_npm` for me 

Add to your `~/.zshrc` 
```
fpath=(~/.config/zsh/completions $fpath)
autoload -Uz compinit && compinit
```
