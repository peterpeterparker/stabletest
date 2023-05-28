mod controllers;
mod entity;
mod impls;
mod types;

use crate::controllers::{
    get_candid_controllers as get_candid_controllers_store,
    get_stable_controllers as get_stable_controllers_store,
    set_candid_controllers as set_candid_controllers_store,
    set_stable_controllers as set_stable_controllers_store,
};
use crate::entity::{
    get_candid_entity as get_candid_entity_store, get_stable_entity as get_stable_entity_store,
    set_candid_entity as set_candid_entity_store, set_stable_entity as set_stable_entity_store,
};
use crate::types::candid::{
    Controller, ControllerId, Controllers, Entity, Key, StableState, State,
};
use crate::types::stable::{MyPrincipal, StableKey};
use candid::{candid_method, export_service};
use ic_cdk::print;
use ic_cdk::storage::{stable_restore, stable_save};
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;
// type StateCell = Cell<StableState, Memory>;
type ControllersState = StableBTreeMap<MyPrincipal, Controller, Memory>;
type DbState = StableBTreeMap<StableKey, Entity, Memory>;

// const STABLE_MEMORY_ID: MemoryId = MemoryId::new(0);
const CONTROLLERS_MEMORY_ID: MemoryId = MemoryId::new(0);
const DB_MEMORY_ID: MemoryId = MemoryId::new(1);

thread_local! {
    static CANDID_STATE: RefCell<State> = RefCell::default();

    // Migration to stable memory
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // static STABLE_STATE: RefCell<StateCell> = MEMORY_MANAGER.with(|memory_manager|
//        RefCell::new(StateCell::init(memory_manager.borrow().get(STABLE_MEMORY_ID), StableState::default()).expect("failed to initialize stable log")));

    static CONTROLLERS_STATE: RefCell<ControllersState> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|memory_manager| memory_manager.borrow().get(CONTROLLERS_MEMORY_ID)),
    ));

    static DB_STATE: RefCell<DbState> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|memory_manager| memory_manager.borrow().get(DB_MEMORY_ID)),
    ));
}

#[init]
fn init() {
    // pre_upgrade/post_upgrade memory
    CANDID_STATE.with(|state| *state.borrow_mut() = State::default());
}

// 0. pre and post upgrade
// 1. migrate from candid to stable
// 2. once migrated remove hooks

#[pre_upgrade]
fn pre_upgrade() {
    // TODO: 1. comment to migrate
    CANDID_STATE.with(|state| stable_save((&state.borrow().stable,)).unwrap());
}

#[post_upgrade]
fn post_upgrade() {
    // TODO: 2. comment once migrated
    let (stable,): (StableState,) = stable_restore().unwrap();
    // TODO: 1. comment to migrate
    CANDID_STATE.with(|state| *state.borrow_mut() = State { stable });

    // TODO: 1. uncomment to migrate
    // TODO: 2. comment once migrated
    // for (controller_id, entity) in stable.controllers {
    //  set_stable_controllers_store(&MyPrincipal::from(&controller_id), &entity);
    //  }
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

#[candid_method(update)]
#[update]
fn set_stable_controllers(key: MyPrincipal, controller: Controller) {
    set_stable_controllers_store(&key, &controller);
}

#[candid_method(query)]
#[query]
fn get_stable_controllers() -> Vec<(MyPrincipal, Controller)> {
    print("Get stable controllers.");
    get_stable_controllers_store()
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
