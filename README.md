# mips_asm

Compile raw MIPS assembly using only the Rust toolchain

## Installation

- Rust installed.
  
- Rust LLVM tools distribution (you can also use any llvm or gnu counterpart)

- [Cargo binutils](https://github.com/rust-embedded/cargo-binutils) for easy integration.

```shell
rustup component add llvm-tools 
cargo install cargo-binutils
```

## How it works

This crate provides a scaffolding for compiling assembly programs for the big endian MIPS.

Since Rust only supports ``mipsle-unknown-none`` (the little endian version) out of the box, we provide a target description for a big endian MIPS in the ``mips.json`` file.
We also provide a simple linker script in ``memory.x``.

This crate is set up to automatically use the provided target description and linker script. On ``cargo build``, the assembly contained in ``asm.s`` is emitted as a global assembly block.

## Example

To edit the assembly to be compiled, edit the ``asm.s`` file in the root of this repo.

To build a binary.

```shell
cargo build 
```

To disassemble the generated binary.

```shell
cargo objdump -- --disassemble --print-imm-hex
```

To get size information and layout for sections and symbols.

```shell
cargo nm
```
