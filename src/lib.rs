extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Ident};

/// A procedural macro to generate an insert-only set for any enum.
///
/// This macro generates a struct with `insert`, `contains`, and `iter` methods for an enum.
/// The struct uses `OnceLock` for thread-safe, one-time insertion of enum variants.
///
/// # Examples
///
/// ```rust
/// use insert_only_set::InsertOnlySet;
///
/// #[derive(InsertOnlySet, Debug, PartialEq)]
/// pub enum Type {
///     Customer,
///     Employee,
/// }
///
/// fn main() {
///     let set = Type::InsertOnlySet();
///
///     assert!(!set.contains(Type::Customer));
///     assert!(!set.contains(Type::Employee));
///
///     set.insert(Type::Customer);
///     assert!(set.contains(Type::Customer));
///     assert!(!set.contains(Type::Employee));
///
///     set.insert(Type::Employee);
///     assert!(set.contains(Type::Customer));
///     assert!(set.contains(Type::Employee));
///
///     for variant in set.iter() {
///         println!("{:?}", variant);
///     }
/// }
/// ```
#[proc_macro_derive(InsertOnlySet)]
pub fn generate_add_only_set(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let set_name = Ident::new(&format!("{}InsertOnlySet", name), name.span());

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

    let new_fields_init = if let Data::Enum(ref data_enum) = input.data {
        data_enum.variants.iter().map(|variant| {
            let field_name = Ident::new(&variant.ident.to_string().to_lowercase(), variant.ident.span());
            quote! {
                #field_name: std::sync::OnceLock::new(),
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
                #name::#variant_name => {
                    if self.#field_name.set(true).is_ok() {
                        true
                    } else {
                        false
                    }
                },
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
                #name::#variant_name => self.#field_name.get().copied().unwrap_or(false),
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
                    variants.push(#name::#variant_name);
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
                    #(#new_fields_init)*
                }
            }

            pub fn insert(&self, t: #name) -> bool {
                match t {
                    #(#insert_methods)*
                }
            }

            pub fn contains(&self, t: #name) -> bool {
                match t {
                    #(#contains_methods)*
                }
            }

            pub fn iter(&self) -> impl Iterator<Item = #name> + '_ {
                let mut variants = Vec::new();
                #(#iter_body)*
                variants.into_iter()
            }
        }

        impl #name {
            /// Creates a new, empty insert-only set for this enum.
            pub fn InsertOnlySet() -> #set_name {
                #set_name::new()
            }
        }
    };

    TokenStream::from(expanded)
}
