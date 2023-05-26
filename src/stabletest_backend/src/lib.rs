mod impls;
mod store;
mod types;

use crate::store::set_controllers as set_controllers_store;
use crate::types::candid::{ControllerId, Entity, StableState, State};
use candid::{candid_method, export_service};
use ic_cdk::storage::{stable_restore, stable_save};
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{Cell, DefaultMemoryImpl};
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type StateCell = Cell<State, Memory>;

const MEMORY_ID: MemoryId = MemoryId::new(0);

thread_local! {
    static STATE: RefCell<State> = RefCell::default();

    // Migration to stable memory
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static STABLE_STATE: RefCell<StateCell> = MEMORY_MANAGER.with(|memory_manager|
        RefCell::new(StateCell::init(memory_manager.borrow().get(MEMORY_ID), State::default()).expect("failed to initialize stable log")));
}

#[init]
fn init() {
    // pre_upgrade/post_upgrade memory
    STATE.with(|state| *state.borrow_mut() = State::default());
}

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|state| stable_save((&state.borrow().stable,)).unwrap());
}

#[post_upgrade]
fn post_upgrade() {
    let (stable,): (StableState,) = stable_restore().unwrap();

    STATE.with(|state| *state.borrow_mut() = State { stable });
}

#[candid_method(query)]
#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[candid_method(update)]
#[update]
fn set_controllers(id: ControllerId, controller: Entity) {
    set_controllers_store(&id, &controller);
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
