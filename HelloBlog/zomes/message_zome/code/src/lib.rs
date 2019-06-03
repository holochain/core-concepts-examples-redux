#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

use hdk::holochain_core_types::cas::content::AddressableContent;

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

const BLOG_ANCHOR:     &str = "blog_anchor";
const BLOG_ENTRY_NAME: &str = "hello_blog_entry";
const BLOG_LINK_TAG:   &str = "blog_messages";

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct MessageEntry {
    content: String,
}

fn message_entry_definition() -> ValidatingEntryType {
    entry!(
        name: "message_entry",
        description: "A *PUBLIC* message entry",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: | _validation_data: hdk::EntryValidationData<MessageEntry>| {
            Ok(())
        },

        links: [
            from!(
                BLOG_ANCHOR,
                tag: BLOG_LINK_TAG,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            )
        ]
    )
}

pub fn handle_create_message(message_string: String) -> ZomeApiResult<Address> {

    let entry = Entry::App("message_entry".into(), MessageEntry { content: message_string }.into());
    let address = hdk::commit_entry(&entry)?;

    let blog_entry = Entry::App(BLOG_ANCHOR.into(), BLOG_ENTRY_NAME.into());
    let blog_anchor = hdk::commit_entry(&blog_entry)?;

    hdk::link_entries(&blog_anchor,&address,BLOG_LINK_TAG)?;

    Ok(address)
}

pub fn handle_get_all_blog_messages() -> ZomeApiResult<Vec<MessageEntry>> {
    let blog_entry = Entry::App(BLOG_ANCHOR.into(), BLOG_ENTRY_NAME.into());

    hdk::utils::get_links_and_load_type(&blog_entry.address(), BLOG_LINK_TAG)
}


define_zome! {
    entries: [
        message_entry_definition(),
        entry!(
            name: BLOG_ANCHOR,
            description: "https://youtu.be/3SwNXQMoNps",
            sharing: Sharing::Public,
            validation_package: || { hdk::ValidationPackageDefinition::Entry },
            validation: | validation_data: hdk::EntryValidationData<String>| { Ok(()) }
        )
    ]

    genesis: || { Ok(()) }

    functions: [
        create_message: {
            inputs: |message_string: String|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_message
        }
        get_all_blog_messages: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Vec<MessageEntry>>|,
            handler: handle_get_all_blog_messages
        }
    ]

    traits: {
        hc_public [
            create_message,
            get_all_blog_messages
        ]
    }
}
