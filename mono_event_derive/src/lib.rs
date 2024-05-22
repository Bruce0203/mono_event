extern crate proc_macro;

use std::{collections::HashMap, fmt::Display, ops::Range, sync::Mutex};

use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, parse_str, Ident, ItemFn, ItemStruct, Meta, ReturnType};

const LISTENER_NAME: &'static str = "__Listener";
const LISTENER_CAPACITY: usize = 250;
const LISTENER_RANGE: Range<usize> = 1..LISTENER_CAPACITY + 1;

#[proc_macro_attribute]
pub fn event(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name: &Ident = &input.ident;
    quote! {
        #input

        impl #name {
            pub fn dispatch(&mut self) -> core::result::Result<(), ()> {
                mono_event::dispatch::<Self, Self>(self)
            }
        }

        impl<T> mono_event::EventListener<#name, T> for #name {
            default fn __listen(event: &mut #name) -> core::result::Result<(), ()> {
                Ok(())
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn listen(attr: TokenStream, item: TokenStream) -> TokenStream {
    let event = parse_macro_input!(attr as Ident);
    let mut input = parse_macro_input!(item as ItemFn);
    let block = input.block;
    let return_value = match input.sig.output {
        ReturnType::Default => quote! { Ok(()) },
        ReturnType::Type(_, _) => quote! {},
    };
    let listener_priority = {
        let index = 0;
        let attrs = input.attrs.clone();
        let mut filtered = attrs.iter().filter_map(|attr| match &attr.meta {
            Meta::Path(path) => {
                let attr_name = path.to_token_stream().to_string();
                if let Ok(priority) = TryInto::<EventPriority>::try_into(attr_name) {
                    input.attrs.remove(index);
                    Some(priority)
                } else {
                    None
                }
            }
            Meta::List(_) => None,
            Meta::NameValue(_) => None,
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
        "listeners count reaced at the capacity {LISTENER_CAPACITY}"
    );
    let lisetner_index = listener_priority.get_listener_index() + listener_count;
    let listener: Ident = format_listener_name(lisetner_index).unwrap();
    let attrs = input.attrs;
    quote! {
        impl mono_event::EventListener<#event, mono_event::#listener> for #event {
            #(#attrs)*
            default fn __listen(event: &mut #event) -> core::result::Result<(), ()> {
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
        #[allow(non_camel_case_types)]
        pub(crate) enum EventPriority {
            $(
                $priority,
            )*
        }

        const PRIORITIES_AMOUNT: usize = [$($priority),*].len();

        impl EventPriority {
            const fn get_listener_index(self) -> i32 {
                let value = self as i32;
                value * LISTENER_CAPACITY as i32 / PRIORITIES_AMOUNT as i32
            }
        }


        impl TryFrom<String> for EventPriority {
            type Error = std::io::Error;

            fn try_from(value: String) -> core::result::Result<Self, Self::Error> {
                Ok(match value.as_str() {
                    $(
                    stringify!($priority) => EventPriority::$priority,
                    stringify!(mono_event::$priority) => EventPriority::$priority,
                    )*
                    _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "not a priority"))?,
                })
            }
        }
        $(
            #[proc_macro_attribute]
            pub fn $priority(_attr: TokenStream, input: TokenStream) -> TokenStream {
                let mut item = parse_macro_input!(input as ItemFn);
                let attrs = &item.attrs.clone();
                item.attrs.clear();
                quote! {
                    #(#attrs)*
                    #[mono_event::$priority]
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
pub fn expand_listeners_and_dispatching_function(_item: TokenStream) -> TokenStream {
    let listeners: Vec<Ident> = LISTENER_RANGE
        .map(|i| format_listener_name(i).unwrap())
        .collect();
    quote! {
    #(pub struct #listeners;)*

    pub fn dispatch<T, V>(v: &mut V) -> core::result::Result<(), ()>
    where
        #(T: EventListener<V, #listeners>,)*
    {
        #(<T as EventListener<V, #listeners>>::__listen(v)?;)*
        Ok(())
    }
    }
    .into()
}

#[inline(always)]
fn format_listener_name<T: Display>(i: T) -> Result<Ident, syn::Error> {
    parse_str(format!("{LISTENER_NAME}{i}").as_str())
}
