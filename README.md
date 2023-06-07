<div align="center">

# pfapack-rs

Rust port of the Fortran library _pfapack_ (arXiv:1102.3440) published on [crates.io](https://crates.io/crates/pfapack). Pfapack is a library for numerically computing the Pfaffian of a real or complex skew-symmetric matrix. This is based on computing the tridiagonal form of the matrix under unitary congruence transformations - _Micheal Wimmer_.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

![Build](https://img.shields.io/github/actions/workflow/status/Duumbo/pfapack-rs/rust.yml?color=%23a3d1af&style=for-the-badge) ![release](https://img.shields.io/github/v/tag/Duumbo/pfapack-rs?color=blue&style=for-the-badge)
![PULLREQUESTS](https://img.shields.io/github/issues-pr-closed/Duumbo/pfapack-rs?color=pink&style=for-the-badge) ![CRATES](https://img.shields.io/crates/v/pfapack?style=for-the-badge)

</div>

## Table of contents

<details>
  <summary><a href="#requirements">Requirements</a></summary>
    <OL, TYPE="square">
    <li><a href="#rust">Rust</a></li>
    <li><a href="#lapackandblas">LAPACK and BLAS</a></li>
  </OL>
</details>

<details>
  <summary><a href="#installation">Installation</a></summary>
</details>

<details>
  <summary><a href="#usage">Usage</a></summary>
</details>

<details>
  <summary><a href="#todo">Todo</a></summary>
</details>

## Requirements

### Rust

We recommend having a version of `cargo >= 1.70.0` and [Rust](https://www.rust-lang.org/) compiler `rustc >= 1.70.0` to use this crate. If you don't know what version you have, run the following commands
```bash
$ cargo version
```
```bash
$ rustc --version
```
If you want to update it anyways, just run the command
```bash
$ rustup update
```
and it will update/upgrade the version of your Rust compiler.

### LAPACK and BLAS

Users must also have a version of [LAPACK](https://www.netlib.org/lapack/) (Linear Algebra PACKage) and [BLAS](https://www.netlib.org/blas/) (Basic Linear Algebra Subprograms) on their computers. Pfapack uses directly those Fortran libraries.

## Installation

Using `cargo`, just add the following to your `Cargo.toml`
```toml
[dependencies]
pfapack = "0.2.0"
```
and the following lines inside the main function of the `build.rs` script (present at the root of your project).
```rust
println!("cargo:rustc-link-lib=pfapack");
println!("cargo:rustc-link-lib=cpfapack");
println!("cargo:rustc-link-lib=lapack");
println!("cargo:rustc-link-lib=blas");
```

Then you should be able to build your project using the command
```bash
$ cargo build
```
also in release mode using `-r` compiler flag.

## Usage

## TODO

- [ ] Complete the `README.md`
- [ ] Finish testing
- [x] Document code base
