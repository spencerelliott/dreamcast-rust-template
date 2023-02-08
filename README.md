# Dreamcast Rust Template

This project aims to create a standard template that anybody can use to develop on the
Sega Dreamcast using the Rust programming language. This is achieved through the use of
[GCCRS](https://github.com/Rust-GCC/gccrs) and the KallistiOS toolchain.

## Getting Started

### Toolchain setup

GCCRS will not be merged into the mainline GCC builds until GCC13 so, for now, in order to
enable Rust support in the Dreamcast GCC toolchain, it will need to be compiled explicitly. 
To get started, we will essentially be using the [standard steps for setting up KallistiOS](https://dreamcast.wiki/Getting_Started_with_Dreamcast_development)
 with a few modifications. First, we must first check out a patched version of KallistiOS instead of the standard version:

```shell
git clone -b gcc-rs https://github.com/darcagn/KallistiOS.git
```

Once this is complete, navigate to the `utils/dc-chain` folder and run the following command:

```shell
mv config.mk.latest.sample config.mk
```

Now, run the **./download.sh** and **./unpack.sh** scripts. When those have completed, open `config.mk` and change:

```shell
sh_gcc_ver=12.2.0
```

to

```shell
sh_gcc_ver=rs
```

Finally, clone the GCCRS toolchain in the `dc-chain` directory:

```shell
git clone https://github.com/Rust-GCC/gccrs gcc-rs
```

Once all the previous steps have been completed, we should be able to run `make` and have a compiled
toolchain the supports Rust!

### Using this Template

Now that we have a working toolchain, we can use this template to generate valid Dreamcast ELF
binaries. Clone this repository and navigate to the folder in your preferred terminal. All of the
available KallistiOS Rust bindings have been included in this repository thanks to the [kos-rust-bindings](https://github.com/spencerelliott/kos-rust-bindings)
package. In order for our program to be able to access these bindings, a `kos.rs` file must be generated.
 This file can be generated using the Makefile located in the `out/` folder or by using the `init.sh` script
at the root of the project. 

After running the `init.sh` script, a `kos.rs` file should appear in the `src/` directory
of the project. This file will automatically be generated each time a build is run so, please do not modify the generated
file as it will be overwritten.

### Building your Binary

Now that we've written some code, you'll want to generate an ELF that can be run on a Dreamcast system. It's as
simple as running the `build.sh` file at the root of the project or running `make -C out/ all`. The provided Makefile
will collect all of your Rust source files and compile them using GCCRS. The standard KallistiOS linking step is used
to generate the final binary. A resulting `dc_rust.elf` should be output to the `out/` directory and it is ready to run!
Please, check out the [documentation for how to run your binary on an actual system](https://dreamcast.wiki/Getting_Started_with_Dreamcast_development#Compiling_and_running_an_example_program).

## Limitations of GCCRS

As GCCRS is in its infancy, a lot of the standard Rust features are not available. Currently, the ability to use
crates is non-existant. This is the reason for the use of our `kos.rs` bindings file since we currently cannot link
against external crates. We are also not using the `Cargo.toml` file for any sort of processing. This only exists to 
allow us to get code completion in IDEs such as CLion and Visual Studio Code. There is also no implementation of Rust's
borrow checker currently but, there are plans to support it in the future through [Polonius](https://github.com/rust-lang/polonius). 
Our plan is to update this template as support for more Rust features become available in GCCRS.
