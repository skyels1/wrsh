# rust shell

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

 - [x] external commands
 - [x] get a working `ls` command for linux and windows
 - [x] get a working `pwd` command
 - [x] get a working `cd` command
 - [x] get working `cat` command
 - [ ] get `grep` to work
 - [ ] get nano to work
 - [ ] piping between commands
 - [ ] move to a full terminal emulator

## Acknowledgements

used for syntax and learning basics and quick reference
 - https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html

This project was inspired by the “Build Custom Shell in Rust” tutorial by Codezup
 - https://codezup.com/build-custom-shell-rust-tutorial/