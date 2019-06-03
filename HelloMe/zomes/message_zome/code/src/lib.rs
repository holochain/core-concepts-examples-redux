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
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    entry::Entry,
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString
};

// see https://developer.holochain.org/api/0.0.12-alpha1/hdk/ for info on using the hdk library

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct MessageEntry {
    content: String,
}

fn message_entry_definition() -> ValidatingEntryType {
    entry!(
        name: "message_entry",
        description: "A *PRIVATE* message entry",
        sharing: Sharing::Private,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: | _validation_data: hdk::EntryValidationData<MessageEntry>| {
            Ok(())
        }
    )
}

pub fn handle_create_message(message_string: String) -> ZomeApiResult<Address> {
    let entry = Entry::App("message_entry".into(), MessageEntry { content: message_string }.into());
    let address = hdk::commit_entry(&entry)?;

    Ok(address)
}

pub fn handle_get_message(address: Address) -> ZomeApiResult<MessageEntry> {
    hdk::utils::get_as_type(address)
}

define_zome! {
    entries: [
        message_entry_definition()
    ]

    genesis: || { Ok(()) }

    functions: [
        create_message: {
            inputs: |message_string: String|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_message
        }
        get_message: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<MessageEntry>|,
            handler: handle_get_message
        }
    ]

    traits: {
        hc_public [
            create_message,
            get_message
        ]
    }
}
