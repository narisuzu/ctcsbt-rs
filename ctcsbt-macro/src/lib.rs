use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{parse_macro_input, spanned::Spanned, Lit};
use yaml::{Field, Packet};

use crate::yaml::FieldVecs;

#[proc_macro]
pub fn packet_from_yaml(input: TokenStream) -> TokenStream {
    if let Lit::Str(s) = parse_macro_input!(input as Lit) {
        let Packet {
            kind, id, fields, ..
        } = Packet::from_yaml(&s.value());

        let (kind_camel, kind_lower) = kind.into();
        let struct_name = format_ident!("{}", kind_camel + &id.to_string());
        let FieldVecs {
            var_vec,
            len_vec,
            arr_len_vec,
            explanation_vec,
            name_vec,
        } = fields.clone().into();

        let field_vars = var_vec.iter().map(|v| format_ident!("{}", v));
        let field_types = fields.iter().map(|f| eval_ty(f.clone()));
        let kind_ident = format_ident!("{}", kind_lower);

        let token = quote! {
            struct #struct_name {
                #(#field_vars: #field_types),*
            }

            impl Packet for #struct_name {
                fn decode(data: &Telegram, start: u16, len: u16) -> Self {
                    todo!()
                }

                fn encode(&self, builder: &mut TelegramBuilder){

                }
            }
        };

        return token.into();
    }
    panic!("expect a string")
}

fn eval_ty(field: Field) -> proc_macro2::TokenStream {
    if let Some(condition) = &field.condition {
        let field = Field {
            condition: None,
            ..field
        };
        let ele_ty = eval_ty(field);
        return quote! {
            Option<#ele_ty>
        };
    }
    if let Some(arr_len) = field.array_len {
        let field = Field {
            array_len: None,
            ..field
        };
        let ele_ty = eval_ty(field);
        return quote! {
            List<#ele_ty, #arr_len>
        };
    }
    match field.length {
        1..=8 => quote! { u8 },
        9..=16 => quote! { u16 },
        17..=32 => quote! { u32 },
        33..=64 => quote! { u64 },
        65..=128 => quote! { u128 },
        _ => panic!("len overflow"),
    }
}

mod yaml;
