extern crate proc_macro;

use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Ident, ItemFn, ReturnType};

const LISTENER_CAPACITY: usize = 1000;
const SINGLE_PRIORITY_LISETNER_CAPACITY: usize = LISTENER_CAPACITY / PRIORITIES_AMOUNT;

#[proc_macro_attribute]
pub fn event(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemStruct);
    let name: &syn::Ident = &input.ident;
    let listeners = (1..LISTENER_CAPACITY + 1).map(|i| {
        let listener: Ident = syn::parse_str(format!("__Listener{i}").as_str()).unwrap();
        quote! {
            <#name as mono_event::EventListener::<#name, mono_event::#listener>>::__listen(self)?;
        }
    });
    quote! {
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
    }
    .into()
}

#[proc_macro_attribute]
pub fn listen(attr: TokenStream, item: TokenStream) -> TokenStream {
    let event = syn::parse_macro_input!(attr as syn::Ident);
    let mut input = syn::parse_macro_input!(item as syn::ItemFn);
    let block = input.block;
    let return_value = match input.sig.output {
        ReturnType::Default => quote! { Ok(()) },
        ReturnType::Type(_, _) => quote! {},
    };
    let listener_priority = {
        let index = 0;
        let attrs = input.attrs.clone();
        let mut filtered = attrs.iter().filter_map(|attr| match &attr.meta {
            syn::Meta::Path(path) => {
                let attr_name = path.to_token_stream().to_string();
                if let Ok(priority) = TryInto::<EventPriority>::try_into(attr_name) {
                    input.attrs.remove(index);
                    Some(priority)
                } else {
                    None
                }
            }
            syn::Meta::List(_) => None,
            syn::Meta::NameValue(_) => None,
        });
        let listener_priority = filtered
            .next()
            .unwrap_or_else(|| EventPriority::normal_priority);
        assert!(filtered.next().is_none(), "multiple priority defined");
        listener_priority
    };

    static LISETNER_COUNT_CACHE: Lazy<Mutex<HashMap<String, i32>>> =
        Lazy::new(|| Mutex::new(HashMap::new()));
    let listener_count = *LISETNER_COUNT_CACHE
        .lock()
        .unwrap()
        .entry(event.to_string())
        .and_modify(|val| *val += 1)
        .or_insert(1);
    assert!(
        listener_count <= LISTENER_CAPACITY as i32,
        "listeners count reaced at the capacity {SINGLE_PRIORITY_LISETNER_CAPACITY}"
    );
    let lisetner_index = listener_priority.get_listener_index() + listener_count;
    let listener: Ident = syn::parse_str(format!("__Listener{lisetner_index}").as_str()).unwrap();
    let attrs = input.attrs;
    quote! {
        impl mono_event::EventListener<#event, mono_event::#listener> for #event {
            #(#attrs)*
            default fn __listen(event: &mut #event) -> std::io::Result<()> {
                #block
                #return_value
            }
        }
    }
    .into()
}

macro_rules! priories {
    ($($priority:ident),*) => {
        #[repr(i32)]
        pub(crate) enum EventPriority {
            $(
                $priority,
            )*
        }

        const PRIORITIES_AMOUNT: usize = [$($priority),*].len();

        impl EventPriority {
            const fn get_listener_index(self) -> i32 {
                let value = self as i32;
                value * SINGLE_PRIORITY_LISETNER_CAPACITY as i32
            }
        }


        impl TryFrom<String> for EventPriority {
            type Error = std::io::Error;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Ok(match value.as_str() {
                    $(
                    stringify!($priority) => EventPriority::$priority,
                    )*
                    _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "not a priority"))?,
                })
            }
        }
        $(
            #[proc_macro_attribute]
            pub fn $priority(_attr: TokenStream, input: TokenStream) -> TokenStream {
                let mut item = syn::parse_macro_input!(input as ItemFn);
                let attrs = &item.attrs.clone();
                item.attrs.clear();
                quote! {
                    #(#attrs)*
                    #[$priority]
                    #item
                }
                .into()
            }
        )*
    };
}

priories!(
    lowest_priority,
    low_priority,
    normal_priority,
    high_priority,
    highest_priority
);

#[proc_macro]
pub fn expand_listener_structs(_item: TokenStream) -> TokenStream {
    let output: String = (1..LISTENER_CAPACITY + 1)
        .map(|i| format!("pub struct __Listener{i};\n",))
        .collect();
    output.parse().unwrap()
}
