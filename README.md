# `RUST for 32F411EDISCOVERY`

> Quickly set up a [`probe-rs`] + [`defmt`] + [`flip-link`] embedded project

[`probe-rs`]: https://crates.io/crates/probe-rs
[`defmt`]: https://github.com/knurling-rs/defmt
[`flip-link`]: https://github.com/knurling-rs/flip-link

## Dependencies

### 1. `flip-link`:

```bash
cargo install flip-link
```

### 2. `probe-rs`:

Install probe-rs by following the instructions at <https://probe.rs/docs/getting-started/installation/>.

### 3. [`cargo-generate`]:

```bash
cargo install cargo-generate
```

[`cargo-generate`]: https://crates.io/crates/cargo-generate

> *Note:* You can also just clone this repository instead of using `cargo-generate`, but this involves additional manual adjustments.

## Setup

### 1. Initialize the project template

```bash
cargo generate \
    --git https://github.com/levietduc0712/cortex-m-rust \
    --branch main \
    --name my-app
```

If you look into your new `my-app` folder, you'll find that there are a few `TODO`s in the files marking the properties you need to set.

Let's walk through them together now.

### 2. Set `probe-rs` chip

Pick a chip from ` probe-rs chip list` and enter it into `.cargo/config.toml`.

If, for example, you have a 32F411EDISCOVERY, replace `{{chip}}` with `STM32F411VE`.

```diff
 # .cargo/config.toml
-runner = ["probe-rs", "run", "--chip", "$CHIP", "--log-format=oneline"]
+runner = ["probe-rs", "run", "--chip", "STM32F411VE", "--log-format=oneline"]
```

### 3. Adjust the compilation target

In `.cargo/config.toml`, pick the right compilation target for your board.

```diff
 # .cargo/config.toml
 [build]
-target = "thumbv6m-none-eabi"    # Cortex-M0 and Cortex-M0+
-# target = "thumbv7m-none-eabi"    # Cortex-M3
-# target = "thumbv7em-none-eabi"   # Cortex-M4 and Cortex-M7 (no FPU)
-# target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)
+target = "thumbv7em-none-eabihf" # Cortex-M4F (with FPU)
```

Add the target with `rustup`.

```bash
rustup target add thumbv7em-none-eabihf
```

### 4. Add a HAL as a dependency

In `Cargo.toml`, list the Hardware Abstraction Layer (HAL) for your board as a dependency.

For the 32F411EDISCOVERY you'll want to use the [`stm32f4xx-hal`].

[`stm32f4xx-hal`]: https://crates.io/crates/stm32f4xx-hal

```diff
 # Cargo.toml
 [dependencies]
-# some-hal = "1.2.3"
+stm32f4xx-hal = "0.23.0"
```

### 5. Import your HAL

Now that you have selected a HAL, fix the HAL import in `src/lib.rs`

```diff
 // my-app/src/lib.rs
-// use some_hal as _; // memory layout
+use stm32f4xx_hal as _; // memory layout
```

### (6. Get a linker script)

Some HAL crates require that you manually copy over a file called `memory.x` from the HAL to the root of your project. For stm32f4xx-hal, this is done automatically so no action is needed. For other HAL crates, see their documentation on where to find an example file.

The `memory.x` file should look something like:

```text
MEMORY
{
  FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 512K
  RAM (rwx)  : ORIGIN = 0x20000000, LENGTH = 128K
}
```

The `memory.x` file is included in the `cortex-m-rt` linker script `link.x`, and so `link.x` is the one you should tell `rustc` to use (see the `.cargo/config.toml` file where we do that).

### 7. Run!

You are now all set to `cargo-run` your first `defmt`-powered application!

```console
$ # `rb` is an alias for `run --bin`
$ cargo rb led_blink
```

If you're running out of memory (`flip-link` bails with an overflow error), you can decrease the size of the device memory buffer by setting the `DEFMT_RTT_BUFFER_SIZE` environment variable. The default value is 1024 bytes, and powers of two should be used for optimal performance:

```console
$ DEFMT_RTT_BUFFER_SIZE=64 cargo rb hello
```

### (8. Set `rust-analyzer.linkedProjects`)

If you are using [rust-analyzer] with VS Code for IDE-like features you can add following configuration to your `.vscode/settings.json` to make it work transparently across workspaces. Find the details of this option in the [RA docs].

```json
{
    "rust-analyzer.linkedProjects": [
        "Cargo.toml",
        "firmware/Cargo.toml",
    ]
}
```

[RA docs]: https://rust-analyzer.github.io/manual.html#configuration
[rust-analyzer]: https://rust-analyzer.github.io/

## Support

`app-template` is part of the [Knurling] project, [Ferrous Systems]' effort at
improving tooling used to develop for embedded systems.

If you think that our work is useful, consider sponsoring it via [GitHub
Sponsors].

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

[Knurling]: https://knurling.ferrous-systems.com
[Ferrous Systems]: https://ferrous-systems.com/
[GitHub Sponsors]: https://github.com/sponsors/knurling-rs
