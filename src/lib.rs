use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Lit};

#[proc_macro_derive(AssertOffsets, attributes(offset))]
pub fn derive_assert_offsets(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("AssertOffsets only supports structs with named fields"),
        },
        _ => panic!("AssertOffsets can only be used with structs"),
    };

    let mut assertions = vec![];

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let mut expected_offset = None;

        for attr in &field.attrs {
            if attr.path().is_ident("offset") {
                if let Ok(Lit::Int(lit_int)) = attr.parse_args() {
                    expected_offset = Some(
                        lit_int
                            .base10_parse::<usize>()
                            .expect("Offset must be an integer"),
                    );
                }
            }
        }

        if let Some(expected_offset) = expected_offset {
            assertions.push(quote! {
                const _: () = assert!(
                    std::mem::offset_of!(#struct_name, #field_name) == #expected_offset,
                    concat!(
                        "Offset of field ",
                        stringify!(#struct_name),
                        "::",
                        stringify!(#field_name),
                        " is not equal to the expected offset ",
                        stringify!(#expected_offset)
                    )
                );
            });
        }
    }

    let expanded = quote! {
        #(#assertions)*;
    };

    TokenStream::from(expanded)
}
