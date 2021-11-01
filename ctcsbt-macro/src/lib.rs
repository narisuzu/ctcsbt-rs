use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Lit};
use yaml::Packet;

use crate::yaml::FieldVecs;

#[proc_macro]
pub fn packet_from_yaml(input: TokenStream) -> TokenStream {
    if let Lit::Str(s) = parse_macro_input!(input as Lit) {
        let Packet {
            kind,
            id,
            fields,
            ..
        } = Packet::from_yaml(&s.value());

        let (kind_camel, kind_lower) = kind.into();

        let struct_name = kind_camel + id.to_string().as_str();
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

        let token = quote! {
            struct #struct_name {
                #(#field_vars: #filed_sizes),*
            }

            impl Packet for #struct_name {
                fn decode(data: &[u32; 32], start: u16, len: u16) -> Self{

                }

                fn encode(&self){

                }
            }
        };

        return token.into();
    }
    panic!("expect a string")
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
