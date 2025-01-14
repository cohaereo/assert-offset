use assert_offset::AssertOffsets;

#[derive(AssertOffsets)]
#[repr(C)] //
pub struct Foo {
    // Try reordering these fields or changing their types
    pub a: u8,
    #[offset(0x2)]
    pub b: u16,
}

fn main() {}
