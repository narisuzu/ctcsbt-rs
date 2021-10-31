use darling::{ast, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{DeriveInput, parse_macro_input, spanned::Spanned};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(packet))]
struct PacketReceiver {
    ident: syn::Ident,
    data: ast::Data<(), VariableField>,

    #[darling(default)]
    etcs: Option<u32>,
    #[darling(default)]
    ctcs: Option<u32>,
}

#[derive(Debug, FromField)]
#[darling(attributes(var))]
struct VariableField {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    len: u32,
    #[darling(default)]
    name: Option<String>,
}

/// implement the method decode from a string
#[proc_macro_derive(Packet, attributes(packet, var))]
pub fn etcs_packet(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as DeriveInput);
    let PacketReceiver {
        ident,
        data,
        etcs,
        ctcs,
    } = PacketReceiver::from_derive_input(&parsed).unwrap();

    let keys = data
        .take_struct()
        .unwrap()
        .map(|vf| vf.ident.unwrap())
        .into_iter();

    let output = quote! {
        impl Packet for #ident {
            fn decode(data: u16, start: u16, width: u16) -> Self {
                Self {
                    #(#keys: 1),*
                }
            }
        }
    };

    output.into()
}

use yaml::Packet;
use crate::yaml::into_multi;

#[proc_macro]
pub fn packet_from_yaml(input: TokenStream) -> TokenStream {

    let parsed = parse_macro_input!(input as syn::Lit);

    match parsed {
        syn::Lit::Str(s) => {
            let Packet {
                packet_kind,
                packet_id,
                fields,
                ..
            } = Packet::from_yaml(&s.value());

            let (kind_camel, kind_lower) = packet_kind.into();
            
            let struct_name = kind_camel + packet_id.to_string().as_str();
            let struct_name = Ident::new(struct_name.as_str(), struct_name.span());
            let (field_vars, field_lens) = into_multi(fields);
           
            let field_vars = field_vars.iter().map(|v| Ident::new(v.as_str(), v.span()));
            let filed_sizes= field_lens.iter().map(|&e| optimized_size(e));
            let kind_ident = Ident::new(kind_lower.as_str(), kind_lower.span());
            quote! {
                use ctcsbt_macro::Packet;

                #[derive(Packet)]
                #[packet(#kind_ident = #packet_id)]
                struct #struct_name {
                    #(
                        #[var(len = #field_lens)]
                        #field_vars: #filed_sizes
                    ),*
                }
            }
        }
        _ => panic!("Expected a string literal"),
    }
    .into()
}

fn optimized_size(len: usize) -> proc_macro2::TokenStream {
    match len {
        0..=8 => quote! { u8 },
        9..=16 => quote! { u16 },
        17..=32 => quote! { u32 },
        33..=64 => quote! { u64 },
        65..=128 => quote! { u128 },
        _ => panic!("Too many fields"),
    }
}

mod yaml;
