#![recursion_limit="128"]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::DeriveInput;
use syn::IntSuffix::U16;

#[proc_macro_derive(GrpcClient, attributes(grpc))]
pub fn grpc_client(input: TokenStream) -> TokenStream {

    let input : DeriveInput = syn::parse(input).unwrap();
    let mut svc_name = "".to_string();
    let mut ip = "".to_string();
    let mut port = 0;

    for meta_items in input.attrs.iter().filter_map(get_rpc_items) {
        for meta_item in meta_items {

            // parse #[grpc(name = "Auth")]
            match meta_item {
                syn::NestedMeta::Meta(syn::Meta::NameValue(ref m)) if m.ident == "tgt_svc_name" => {
                    if let syn::Lit::Str(ref lit) = m.lit {
                        svc_name = lit.value();

                        println!("get svc_name : {}", svc_name);
                    }
                }

                // parse #[grpc(ip = "0.0.0.0")]
                syn::NestedMeta::Meta(syn::Meta::NameValue(ref m)) if m.ident == "ip" => {
                    if let syn::Lit::Str(ref lit) = m.lit {
                        ip = lit.value();

                        println!("get ip : {}", ip);
                    }
                }

                // parse #[grpc(port = "30303")]
                syn::NestedMeta::Meta(syn::Meta::NameValue(ref m)) if m.ident == "port" => {
                    if let syn::Lit::Str(ref lit) = m.lit {
                        port = lit.value().parse::<u64>().unwrap();

                        println!("get port : {}", port);
                    }
                }
                _ => {}
            }
        }
    }

    if ip == "" {
        panic!("ip must set!");
    }

    if port == 0 {
        panic!("port must set!")
    }

    let name = input.ident;
    let ip = syn::LitStr::new(&ip, proc_macro2::Span::call_site());
    let port = syn::LitInt::new(port, U16, proc_macro2::Span::call_site());
    let svc_name_client = svc_name + "Client";

    let svc_name_client = syn::Ident::new(&svc_name_client.to_string(), proc_macro2::Span::call_site());
    let output = {
        quote!(
            impl #name {
                pub fn new() -> #svc_name_client {
                    #svc_name_client::new_plain(#ip, #port, Default::default()).unwrap()
                }
            }
        )
    };

    output.into()

}

fn get_rpc_items(attr: &syn::Attribute) -> Option<Vec <syn::NestedMeta>> {
    if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "grpc" {
        match attr.interpret_meta() {
            Some(syn::Meta::List(ref meta)) => Some(meta.nested.iter().cloned().collect()),
            _ => {
                None
            }
        }
    } else {
        None
    }
}
