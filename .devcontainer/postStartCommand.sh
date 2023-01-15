#!/usr/bin/fish

# コンテナ開始時に毎回実行されるので冪等な操作しか書いてはいけない

# dotfiles のインストールで場所が移動してしまうので復元する
cp ~/.config.org/cargo-atcoder.toml ~/.config/

abbr --add cal cargo atcoder login
abbr --add can cargo atcoder new
abbr --add cat cargo atcoder test
abbr --add cas cargo atcoder submit
abbr --add cast cargo atcoder status
abbr --add car cargo atcoder result
