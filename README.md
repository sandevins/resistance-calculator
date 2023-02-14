# Resistance calculator

This application allows the translation from colour-coded resistances to impedance value.

## Build

To build the application, Rust must be installed in the machine. There are instructions for the installation in [Rust install guide](https://www.rust-lang.org/tools/install). Once this prerequisite has been completed, the application can be compiled with the following command.

```
cargo build -r
```

After that, the application is accesible under `target/release/resistor_color`. To make it more accessible (in Linux) the following command can be used.

```
cp target/release/resistor_color /home/user/.cargo/bin/rescal
```

## Usage

To use the application, simply execute the binary with the three colors as arguments.

```
./resistor_color red brown blue
```

The colors that can be used and their meaning can be consulted in the following image.

![here](https://qph.cf2.quoracdn.net/main-qimg-92cdb5cd78581d8a1dae2064a7e16151)

