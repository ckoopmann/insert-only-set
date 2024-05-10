extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Ident};

#[proc_macro_derive(GenerateAddOnlySet)]
pub fn generate_add_only_set(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let set_name = Ident::new(&format!("{}AddOnlySet", name), name.span());

    let fields = if let Data::Enum(ref data_enum) = input.data {
        data_enum.variants.iter().map(|variant| {
            let field_name = Ident::new(&variant.ident.to_string().to_lowercase(), variant.ident.span());
            quote! {
                pub #field_name: std::sync::OnceLock<bool>,
            }
        }).collect::<Vec<_>>()
    } else {
        vec![]
    };

    let insert_methods = if let Data::Enum(ref data_enum) = input.data {
        data_enum.variants.iter().map(|variant| {
            let field_name = Ident::new(&variant.ident.to_string().to_lowercase(), variant.ident.span());
            let variant_name = &variant.ident;
            quote! {
                Type::#variant_name => self.#field_name.set(true).map_err(|_| "Already set"),
            }
        }).collect::<Vec<_>>()
    } else {
        vec![]
    };

    let contains_methods = if let Data::Enum(ref data_enum) = input.data {
        data_enum.variants.iter().map(|variant| {
            let field_name = Ident::new(&variant.ident.to_string().to_lowercase(), variant.ident.span());
            let variant_name = &variant.ident;
            quote! {
                Type::#variant_name => self.#field_name.get().copied().unwrap_or(false),
            }
        }).collect::<Vec<_>>()
    } else {
        vec![]
    };

    let iter_body = if let Data::Enum(ref data_enum) = input.data {
        data_enum.variants.iter().map(|variant| {
            let field_name = Ident::new(&variant.ident.to_string().to_lowercase(), variant.ident.span());
            let variant_name = &variant.ident;
            quote! {
                if self.#field_name.get().copied().unwrap_or(false) {
                    variants.push(Type::#variant_name);
                }
            }
        }).collect::<Vec<_>>()
    } else {
        vec![]
    };

    let expanded = quote! {
        pub struct #set_name {
            #(#fields)*
        }

        impl #set_name {
            pub fn new() -> Self {
                Self {
                    #(#fields: std::sync::OnceLock::new(),)*
                }
            }

            pub fn insert(&self, t: Type) -> Result<(), &'static str> {
                match t {
                    #(#insert_methods)*
                }
            }

            pub fn contains(&self, t: Type) -> bool {
                match t {
                    #(#contains_methods)*
                }
            }

            pub fn iter(&self) -> impl Iterator<Item = Type> + '_ {
                let mut variants = Vec::new();
                #(#iter_body)*
                variants.into_iter()
            }
        }
    };

    TokenStream::from(expanded)
}
