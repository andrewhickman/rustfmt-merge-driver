# rustfmt-merge-driver

A [custom merge driver](https://git-scm.com/docs/gitattributes#_defining_a_custom_merge_driver) for Git which runs rustfmt on files before trying to merge them. This helps resolve conflicts caused by one user modifying some code while another runs rustfmt.

## Installation

The application can be installed through cargo

```
cargo install --git https://github.com/andrewhickman/rustfmt-merge-driver
```

To configure the merge driver for a repository, add it to the config with

```
git config merge.rust.name "rustfmt merge driver"
git config merge.rust.driver "rustfmt-merge-driver %A %O %B --marker-size %L"
```

and then tell git to merge `.rs` files with it by adding this line to your [`.gitattributes`](https://git-scm.com/docs/gitattributes) file

```
*.rs merge=rust
```
