# Introduction to Cargo

To generate an application in a new directory we run:

```sh
  cargo new <name> --(bin|lib)
```

If we want to create a new cargo application in the current directory though, then we would run:

```sh
  cargo init --name <name> --(bin|lib)
```

The `bin` option is for a regular binary application and the `lib` option is for building a library for use within a binary application, for example [Rocket](https://rocket.rs/) would be an example of a `lib` for use in your `bin` app.

## Running code

To run the code in the `src` directory, with `main.rs` as the entry point, we can use the following command in the root directory:

```sh
  cargo run
```

The `run` command is effectively going to run the `check` and `build` commands before executing the generated `<name>.exe` file for you assuming both of the previous commands exit successfully.

## Compiling code

To compile the code in the `src` directory we can use the following command in the root directory:

```sh
  cargo build
```

The `<name>.exe` file will be generated within the `target/debug` directory during compilation.

## Checking code and cargo configurations

To check the code in the `src` directory and the cargo configuration files too, we can use the following command in the root directory:

```sh
  cargo check
```

The main difference between `check` and `build` is that `check` won't generate a new `.exe`. Instead, it will instead run the compiler without output so that it can lint the code and then it will follow up with checking cargo specific configurations too so that you can be sure that if you `build` or `run`, things should work as expected.