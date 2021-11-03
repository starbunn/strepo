# strepo
A tool for automatically setting up repositories with commitlint, code formatting/linting, and a guide on how to do good branch names.
## Installation Guide
If you're on Windows, we recommend that you use Ubuntu in WSL.
### Ubuntu 21.04 in WSL
First, install `rustc` and `cargo`.
```
user@computer:/mnt/c/Users/User$ sudo apt install rustc cargo
[sudo] password for user:
Reading package lists... Done
Building dependency tree... Done
Reading state information... Done
The following additional packages will be installed:
  gcc gcc-10 libasan6 libgcc-10-dev libstd-rust-1.51 libstd-rust-dev rust-gdb
Suggested packages:
  cargo-doc gcc-multilib make autoconf automake libtool flex bison gcc-doc gcc-10-multilib gcc-10-doc gcc-10-locales
  gdb-doc rust-doc rust-src lld-11
The following NEW packages will be installed:
  cargo gcc gcc-10 libasan6 libgcc-10-dev libstd-rust-1.51 libstd-rust-dev rust-gdb rustc
0 upgraded, 9 newly installed, 0 to remove and 0 not upgraded.
Need to get 96.2 MB of archives.
After this operation, 436 MB of additional disk space will be used.
Do you want to continue? [Y/n]
```
Then, you need to clone the git repository at `https://github.com/virtualout/strepo`.
```
user@computer:/mnt/c/Users/User$ git clone https://github.com/virtualout/strepo.git
Cloning into 'strepo'...
remote: Enumerating objects: 29, done.
remote: Counting objects: 100% (29/29), done.
remote: Compressing objects: 100% (21/21), done.
remote: Total 29 (delta 7), reused 23 (delta 6), pack-reused 0
Receiving objects: 100% (29/29), 7.18 KiB | 432.00 KiB/s, done.
Resolving deltas: 100% (7/7), done.
```
Since there is no code yet in `main`, you need to checkout to the dev branch.
```
user@computer:/mnt/c/Users/User$ cd strepo
user@computer:/mnt/c/Users/User/strepo$ git checkout dev
Branch 'dev' set up to track remote branch 'dev' from 'origin'.
Switched to a new branch 'dev'
```
To run, you need to run it using cargo.
```
user@computer:/mnt/c/Users/User/strepo$ cd code
user@computer:/mnt/c/Users/User/strepo/code$ cargo run
```
After that, it should be built and give you a binary or run it. 
