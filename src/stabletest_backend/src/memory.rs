use crate::types::candid::NewDb;
use crate::types::candid::State;
use crate::types::stable::Memory;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::DefaultMemoryImpl;
use ic_stable_structures::StableBTreeMap;
use std::cell::RefCell;

const UPGRADES: MemoryId = MemoryId::new(0);
const STABLE_BTREE: MemoryId = MemoryId::new(1);

thread_local! {
    pub static STATE: RefCell<State> = RefCell::default();

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

}

fn get_stable_btree_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(STABLE_BTREE))
}

pub fn get_upgrades_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(UPGRADES))
}

pub fn init_stable_data() -> NewDb {
    StableBTreeMap::init(get_stable_btree_memory())
}
