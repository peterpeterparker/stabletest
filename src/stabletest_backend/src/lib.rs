mod types;
mod store;

use std::cell::RefCell;
use crate::types::candid::{ControllerId, Entity, StableState, State};
use ic_cdk::storage::{stable_restore, stable_save};
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use crate::store::set_controllers as set_controllers_store;
use candid::{candid_method, export_service};

thread_local! {
    static STATE: RefCell<State> = RefCell::default();
}

#[init]
fn init() {
    STATE.with(|state| {
        *state.borrow_mut() = State::default()
    });
}

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|state| stable_save((&state.borrow().stable,)).unwrap());
}

#[post_upgrade]
fn post_upgrade() {
    let (stable, ): (StableState, ) = stable_restore().unwrap();

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