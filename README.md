# Rust Chip-8

Cross-platform regular Chip-8 interpreter (not SUPER-CHIP or XO-CHIP).

## Installation

1. Clone the repository
2. `cargo build --release`
3. The resulting binary will be in `./release/emulator`

## Usage

Point the emulator to a ROM path and it should work.

```sh
emulator ./path/to/my/ROM.ch8
```

For more detailed help, run `emulator --help`.

### Custom configuration

You can see the configuration options by generating a configuration file.

```sh
emulator generate-config ./emulator-config.toml
```

## Architecture

The codebase is split into 2 crates:

- libchip8 - core reusable Chip-8 logic
- emulator - GUI integration with libchip8 (main application)

## Testing

https://github.com/Timendus/chip8-test-suite test suite was used for testing.

| Test        | Status |
| ----------- | ------ |
| Chip-8 logo | ✅     |
| IBM logo    | ✅     |
| corax+      | ✅     |
| flags       | ✅     |

TODO

| quirks | ✅ |
| keypad | ✅ |
| beep | ✅ |
