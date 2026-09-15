# Introduction to Cargo Crates


## Installing crates

To install a crate we can go to [crates.io](https://crates.io) and search for a crate of our liking. From there we can copy the line on the crate overview page and add it to our `cargo.toml` dependencies section.

Alternatively we can use the `install` command. As an example, if we wanted to install a `bin` crate, we could run the following from the root directory:

```sh
  cargo install <name>
```

We couldn't use that for `lib` crates though and for these we either go with the `cargo.toml` route or we can install `cargo-edit` and use it to install any crate we like. To be able to use this to install the `rand` crate for example, we would run:

```sh
  cargo install cargo-edit
  cargo add rand
```

> An important thing to note is that **every** Cargo project manages and compiles a separate set of dependencies, here is [some background info](https://stackoverflow.com/questions/37471391/can-i-prevent-cargo-from-rebuilding-libraries-with-every-new-project/37472558#37472558) on that. Thus it doesn't make sense to install a compiled library since the source code for each version of a library will be cached locally, avoiding downloading it multiple times.

This means that the best way to do things is to just use the `cargo.toml` route for `lib` crates and cargo install for `bin` crates.

## Upgrading crates

To upgrade our dependencies, we can run:

```sh
  cargo update
```

This will update the dependencies in the `Cargo.lock` file to the latest version. It requires that the `Cargo.lock` file already exists which you can generate using `cargo build` for example.