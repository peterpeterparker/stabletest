mod controllers;
mod entity;
mod impls;
mod memory;
mod types;

use crate::controllers::{
    get_candid_controllers as get_candid_controllers_store,
    set_candid_controllers as set_candid_controllers_store,
};
use crate::entity::{
    get_candid_entity as get_candid_entity_store, get_stable_entity as get_stable_entity_store,
    get_candid_entities as get_candid_entities_store, get_stable_entities as get_stable_entities_store,
    set_candid_entity as set_candid_entity_store, set_stable_entity as set_stable_entity_store,
};
use crate::memory::{get_upgrades_memory, STATE};
use crate::types::candid::{Controller, ControllerId, Controllers, Entity, Key, State};
use crate::types::stable::Memory;
use candid::{candid_method, export_service};
use ciborium::{from_reader, into_writer};
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use ic_stable_structures::writer::Writer;
use ic_stable_structures::Memory as _;

#[init]
fn init() {
    STATE.with(|state| *state.borrow_mut() = State::default());
}

#[pre_upgrade]
fn pre_upgrade() {
    // Serialize the state.
    // This example is using CBOR, but you can use any data format you like.
    let mut state_bytes = vec![];
    STATE
        .with(|s| into_writer(&*s.borrow(), &mut state_bytes))
        .expect("failed to encode state");

    // Write the length of the serialized bytes to memory, followed by the
    // by the bytes themselves.
    let len = state_bytes.len() as u32;
    let mut memory = get_upgrades_memory();
    let mut writer = Writer::new(&mut memory, 0);
    writer.write(&len.to_le_bytes()).unwrap();
    writer.write(&state_bytes).unwrap()
}

#[post_upgrade]
fn post_upgrade() {
    let memory: Memory = get_upgrades_memory();

    // Read the length of the state bytes.
    let mut state_len_bytes = [0; 4];
    memory.read(0, &mut state_len_bytes);
    let state_len = u32::from_le_bytes(state_len_bytes) as usize;

    // Read the bytes
    let mut state_bytes = vec![0; state_len];
    memory.read(4, &mut state_bytes);

    // Deserialize and set the state.
    let state = from_reader(&*state_bytes).expect("failed to decode state");
    STATE.with(|s| *s.borrow_mut() = state);
}

/// Controllers

#[candid_method(update)]
#[update]
fn set_candid_controllers(id: ControllerId, controller: Controller) {
    set_candid_controllers_store(&id, &controller);
}

#[candid_method(query)]
#[query]
fn get_candid_controllers() -> Controllers {
    get_candid_controllers_store()
}

/// Entity

#[candid_method(update)]
#[update]
fn set_candid_entity(collection: Key, key: Key, controller: Entity) {
    set_candid_entity_store(&collection, &key, &controller);
}

#[candid_method(query)]
#[query]
fn get_candid_entity(collection: Key, key: Key) -> Option<Entity> {
    get_candid_entity_store(&collection, &key)
}

#[candid_method(query)]
#[query]
fn get_candid_entities(collection: Key) -> Vec<Entity> {
    get_candid_entities_store(&collection)
}

#[candid_method(update)]
#[update]
fn set_stable_entity(collection: Key, key: Key, controller: Entity) {
    set_stable_entity_store(&collection, &key, &controller);
}

#[candid_method(query)]
#[query]
fn get_stable_entity(collection: Key, key: Key) -> Option<Entity> {
    get_stable_entity_store(&collection, &key)
}

#[candid_method(query)]
#[query]
fn get_stable_entities(collection: Key) -> Vec<Entity> {
    get_stable_entities_store(&collection)
}

///
/// Generate did files
///

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let dir = dir
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("src")
            .join("stabletest_backend");
        write(dir.join("stabletest_backend.did"), export_candid()).expect("Write failed.");
    }
}
