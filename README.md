# npm-runzf

A zsh widget to fuzzy find npm run commands within a monorepo 

## Getting setup 
Build the cargo package 
```
cargo build --release
```

Add it to your path via `cp` or symlink
```
ln -sf "$(pwd)/target/release/npm-runzf ~/.local/bin/  # or somewhere in your PATH
```

Add the zsh script

```
ln -sf "$(pwd)/npm-runzf.zsh" ~/.config/zsh/functions # or wherever you want the function 
```

Add to your `~/.zshrc` 
```
source ~/.config/zsh/functions
```

## Usage
After `npm run`, press your keybind, in my case on a Mac, option+t, and the fzf menu will pop-up to fuzzy search your commands! 
