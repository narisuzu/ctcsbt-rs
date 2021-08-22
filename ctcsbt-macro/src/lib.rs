use darling::{ast, FromDeriveInput, FromField, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

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
    
    let func = quote! {
        impl Packet for #ident {
            fn decode(data: u16, start: u16, len: u16) -> Self {
                Self {
                    #(#keys: 1),*
                }
            }
        }
    };

    func.into()
}
