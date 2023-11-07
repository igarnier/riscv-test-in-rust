# Trying to build a riscv test as inline assembly

- we use the !global_asm to import the assembly code, this seems to work well
- however we need to find a way to specify the memory layout of the code, otherwise the produced binary code is incorrect (rustc produces invalid addresses)

## How to build the assembly file using the gnu toolchain

```
riscv64-unknown-elf-gcc -mabi=lp64 -static -mcmodel=medany -fvisibility=hidden -nostdlib -nostartfiles -Tlink.ld ./src/add.S -o add
```

Then use e.g. `spike` to execute `add`
