# Architectures

## RISC V

### SBI Calls

For riscv, neutron interacts with sbi via the `ecall` instruction. Like so:

```rust
unsafe { asm!(
            "ecall",
            in("a0") arg0, in("a1") arg1,
            in("a6") function, in("a7") extension,
            lateout("a0") error, lateout("a1") value,
        ) }
```

- this uses the values `arg0, arg1, function, extension, error, value`
