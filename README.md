# assert-offset

`assert-offset` is a simple Rust derive macro for asserting memory offsets within structures. 
This can be useful for low level FFI and/or embedded development.

⚠ This crate does not change field offsets, it merely asserts that they are at the expected offsets.

## Examples

### Usage

```rust
use assert_offset::AssertOffsets;

#[derive(AssertOffsets)]
#[repr(C)]
pub struct Foo {
    // Try reordering these fields or changing their types
    pub a: u8,
    #[offset(0x2)]
    pub b: u16,
}
```

### Failing Example

```rust
use assert_offset::AssertOffsets;

// Note that we're not using #[repr(C)] here
#[derive(AssertOffsets)]
pub struct Foo {
    pub a: u8,

     // Rust (usually) puts this field at the start of the struct,
     // because u16 has a higher alignment requirement than u8
    #[offset(0x2)] // Changing this offset to 0x0 (should) fix the error
    pub b: u16,
}
```

#### Compiler Error

```text
error[E0080]: evaluation of constant value failed
 --> src\main.rs:3:10
  |
3 | #[derive(AssertOffsets)]
  |          ^^^^^^^^^^^^^ the evaluated program panicked at 'Field `Foo::b` is not at expected offset 0x2'
```

## License

This project is licensed under the MIT License.