use proc_macro::TokenStream as TokenStream2;
use proc_macro::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, Data, DeriveInput, Meta, Token};

#[proc_macro_derive(Discriminant)]
pub fn discriminant_derive(t: TokenStream) -> TokenStream {
    let ty = TokenStream2::from(t);
    let ast = syn::parse(ty).unwrap();

    ensure_enum_valid(&ast);
    if find_repr_type(&ast).unwrap() != "i16" {
        panic!("Discriminant can only be derived for enums with repr(i16)");
    }

    impl_discriminant_macro(&ast)
}

fn impl_discriminant_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Discriminant for #name {
            fn discriminant(&self) -> i16 {
                unsafe { *<*const #name>::from(self).cast::<i16>() }
            }
        }
    };
    gen.into()
}

fn ensure_enum_valid(ast: &DeriveInput) {
    if let Data::Enum(data) = &ast.data {
        if data.variants.is_empty() == false {
            return;
        }

        panic!("Can't derive PrimitiveRepr on a zero variant enum");
    }

    panic!("Discriminant can only be derived for enums");
}

fn find_repr_type(ast: &DeriveInput) -> Option<String> {
    for meta in ast
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("repr"))
        .filter_map(|attr| {
            attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .ok()
        })
        .flatten()
    {
        if let Meta::Path(path) = meta {
            if let Some(ident) = path.get_ident() {
                return Some(ident.to_string());
            }
        }
    }

    None
}
