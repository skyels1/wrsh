# wrsh - Windows Replacement Shell

project to learn the basics of rust

turned into an obsession to make the full terminal

## Usage

type any system command and hit enter

all commands should work the same as any linux terminal or git bash

to quit type `q` and hit enter
can change dir with cd and change to other drives with
`cd X:`
where X is the drive you want to switch to

## Installation and guide

`git clone https://github.com/skyels1/rustShell.git`\
`cargo build`\
`cargo run`\
or hit run inside vscode if you have code runner installed

should be same on both windows and linux

## Plans for features

 - *v0.1.0* - [x] external commands
 - *v0.2.0* - [x] get a working `ls` command for linux and windows
 - *v0.3.0* - [x] get a working `pwd` command
 - *v0.4.0* - [x] get a working `cd` command
 - *v0.5.0* - [x] get working `cat` command
 - *v0.6.0* - [x] get `grep` to work
     grep pattern file
 - *v0.7.0* - [x] get `nano` to work (kind of)
     only works on linux/git bash, on windows just do `notepad file` or if you have nvim that also works
 - *v0.8.0* - [x] working `fetch` command
     if neofetch or any other fetch downloaded it is supported
 - *v0.9.0* - [x] piping between commands
     works for external commands only at the moment
 - *v0.9.1* - [x] move to a full terminal emulator
     works for shell if you follow guide below
 - *v1.0.0* - [x] stable fully working shell
 - [ ] get working nano fully made from scratch
 - [ ] get working piping native to windows
 - [ ] fill out fetch more
 - [ ] add more grep options

## how to use in terminal

if you want to use this shell in terminal
1. git clone the repo
2. cargo build --release
3. open the terminal settings toml or yaml
4. set file path to the /target/release/wrsh.exe
5. run the terminal

## Acknowledgements

used for syntax and learning basics and quick reference
 - https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html

This project was inspired by the “Build Custom Shell in Rust” tutorial by Codezup
 - https://codezup.com/build-custom-shell-rust-tutorial/
