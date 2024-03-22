How to install Rust package from current directory: <https://doc.rust-lang.org/cargo/commands/cargo-install.html#examples>

```sh
cargo install --path .
```

Also, you must install to path that is in your PATH environment variable. For example, if you want to install to `~/.cargo/bin`, you must add `~/.cargo/bin` to your PATH environment variable.

For csug machines add to .bashrc and you might need to restart terminal

```.bashrc
export PATH="/u/jyao15/.cargo/bin:$PATH"
```

Now you can run
```sh
dvcs-wye -h
```
