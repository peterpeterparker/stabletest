mod impls;
mod store;
mod types;

use crate::store::{
    get_candid_controllers as get_candid_controllers_store,
    get_stable_controllers as get_stable_controllers_store,
    set_candid_controllers as set_candid_controllers_store,
    set_stable_controllers as set_stable_controllers_store,
};
use crate::types::candid::{ControllerId, Controllers, Entity, StableState, State};
use crate::types::stable::MyPrincipal;
use candid::{candid_method, export_service};
use ic_cdk::storage::{stable_restore, stable_save};
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{Cell, DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;
use ic_cdk::print;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type StateCell = Cell<StableState, Memory>;
type ControllersState = StableBTreeMap<MyPrincipal, Entity, Memory>;

// const STABLE_MEMORY_ID: MemoryId = MemoryId::new(0);
const CONTROLLERS_MEMORY_ID: MemoryId = MemoryId::new(0);

thread_local! {
    static CANDID_STATE: RefCell<State> = RefCell::default();

    // Migration to stable memory
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // static STABLE_STATE: RefCell<StateCell> = MEMORY_MANAGER.with(|memory_manager|
//        RefCell::new(StateCell::init(memory_manager.borrow().get(STABLE_MEMORY_ID), StableState::default()).expect("failed to initialize stable log")));

    static CONTROLLERS_STATE: RefCell<ControllersState> = RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|memory_manager| memory_manager.borrow().get(CONTROLLERS_MEMORY_ID)),
        ))
}

#[init]
fn init() {
    // pre_upgrade/post_upgrade memory
    CANDID_STATE.with(|state| *state.borrow_mut() = State::default());
}

#[pre_upgrade]
fn pre_upgrade() {
    CANDID_STATE.with(|state| stable_save((&state.borrow().stable,)).unwrap());
}

#[post_upgrade]
fn post_upgrade() {
    // let (stable,): (StableState,) = stable_restore().unwrap();
    // CANDID_STATE.with(|state| *state.borrow_mut() = State { stable });
}

#[candid_method(update)]
#[update]
fn set_candid_controllers(id: ControllerId, controller: Entity) {
    set_candid_controllers_store(&id, &controller);
}

#[candid_method(query)]
#[query]
fn get_candid_controllers() -> Controllers {
    get_candid_controllers_store()
}

#[candid_method(update)]
#[update]
fn set_stable_controllers(key: MyPrincipal, controller: Entity) {
    set_stable_controllers_store(&key, &controller);
}

#[candid_method(query)]
#[query]
fn get_stable_controllers() -> Vec<(MyPrincipal, Entity)> {
    print("Get stable controllers.");
    get_stable_controllers_store()
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
