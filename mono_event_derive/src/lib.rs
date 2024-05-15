extern crate proc_macro;

use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, LitInt, ReturnType};

#[proc_macro_attribute]
pub fn event(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemStruct);
    let name: &syn::Ident = &input.ident;
    let listeners = (1..100 + 1).map(|i| {
        let listener: Ident = syn::parse_str(format!("__Listener{i}").as_str()).unwrap();
        quote! {
            <#name as mono_event::EventListener::<#name, mono_event::#listener>>::__listen(self)?;
        }
    });
    let expanded = quote! {
        #input

        impl #name {
            fn dispatch(&mut self) -> std::io::Result<()> {
                #(#listeners)*
                Ok(())
            }
        }

        impl<T> mono_event::EventListener<#name, T> for #name {
            default fn __listen(event: &mut #name) -> std::io::Result<()> {
                Ok(())
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn listen(attr: TokenStream, item: TokenStream) -> TokenStream {
    static mut LISETNER_COUNT_CACHE: Lazy<Mutex<HashMap<String, i32>>> =
        Lazy::new(|| Mutex::new(HashMap::new()));

    let event = syn::parse_macro_input!(attr as syn::Ident);
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let block = input.block;
    let return_value = match input.sig.output {
        ReturnType::Default => quote! { Ok(()) },
        ReturnType::Type(_, _) => quote! {},
    };

    let mut mutex = unsafe { LISETNER_COUNT_CACHE.lock().unwrap() };
    let listener_count: i32 = if let Some(value) = mutex.get_mut(&event.to_string()) {
        *value += 1;
        value.clone()
    } else {
        mutex.insert(event.to_string(), 1);
        1
    };
    let listener: Ident = syn::parse_str(format!("__Listener{listener_count}").as_str()).unwrap();
    let expanded = quote! {
        impl mono_event::EventListener<#event, mono_event::#listener> for #event {
            default fn __listen(event: &mut #event) -> std::io::Result<()> {
                #block
                #return_value
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn listeners_capacity(item: TokenStream) -> TokenStream {
    let int = syn::parse_macro_input!(item as LitInt);
    let amount: usize = int.base10_parse().unwrap();
    let output: String = (1..amount + 1)
        .map(|i| format!("pub struct __Listener{i};\n",))
        .collect();
    output.parse().unwrap()
}
