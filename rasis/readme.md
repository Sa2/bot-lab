
[参考にした記事](https://developers.facebook.com/blog/post/2020/09/30/build-discord-bot-with-rust-and-serenity/?locale=ja_JP)


## ref.

### タスクランナー

cargo-make

```
$ cargo install --force cargo-make
```


### クロスコンパイラ

cross

`rustup install stable-aarch64-unknown-linux-gnu` linux用のビルド環境をインストールする

```
$ cargo install cross 
$ rustup install stable-aarch64-unknown-linux-gnu
```

build for mac
```
$ cargo build --target x86_64-apple-darwin
```

build for raspi
```
$ cargo build --target aarch64-unknown-linux-gnu
```