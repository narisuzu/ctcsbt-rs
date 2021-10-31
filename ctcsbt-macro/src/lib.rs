use darling::{ast, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

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

use crate::yaml::FieldVecs;

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
            let FieldVecs {
                var_vec,
                len_vec,
                arr_len_vec,
                explanation_vec,
                name_vec,
            } = fields.into();

            let field_vars = var_vec.iter().map(|v| Ident::new(v.as_str(), v.span()));
            
            let cloned_arr_len_vec = arr_len_vec.clone();
            let filed_sizes = len_vec.iter().enumerate().map(move |(i, &len)| {
                let &arr_len = cloned_arr_len_vec.get(i).unwrap();
                eval_ty(len, arr_len)
            });
            let kind_ident = Ident::new(kind_lower.as_str(), kind_lower.span());

            quote! {
                #[derive(Packet)]
                #[packet(#kind_ident = #packet_id)]
                struct #struct_name {
                    #(
                        #[var(len = #len_vec)]
                        #field_vars: #filed_sizes
                    ),*
                }
            }
        }
        _ => panic!("Expected a string literal"),
    }
    .into()
}

fn eval_ty(len: usize, arr_len: Option<usize>) -> proc_macro2::TokenStream {
    if let Some(arr_len) = arr_len {
        quote! {
            List<usize, #arr_len>
        }
    } else {
        match len {
            0..=8 => quote! { u8 },
            9..=16 => quote! { u16 },
            17..=32 => quote! { u32 },
            33..=64 => quote! { u64 },
            65..=128 => quote! { u128 },
            _ => panic!("len overflow"),
        }
    }
}

mod yaml;
