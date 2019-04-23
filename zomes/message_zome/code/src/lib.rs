#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

use hdk::{
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    error::HolochainError,
    json::JsonString,
};

// see https://developer.holochain.org/api/0.0.12-alpha1/hdk/ for info on using the hdk library

pub fn handle_get_message() -> ZomeApiResult<String> {
    Ok(String::from("Hello, Holo!"))
}

define_zome! {
    entries: [
    ]

    genesis: || { Ok(()) }

    functions: [
        get_message: {
            inputs: | |,
            outputs: |result: ZomeApiResult<String>|,
            handler: handle_get_message
        }
    ]

    traits: {
        hc_public [
            get_message
        ]
    }
}
