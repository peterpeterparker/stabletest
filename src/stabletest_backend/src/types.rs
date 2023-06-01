pub mod candid {
    use crate::memory::init_stable_data;
    use crate::types::stable::{Memory, StableKey};
    use candid::CandidType;
    use candid::Principal;
    use ic_stable_structures::StableBTreeMap;
    use serde::{Deserialize, Serialize};
    use std::collections::{BTreeMap, HashMap};

    pub type NewDb = StableBTreeMap<StableKey, Entity, Memory>;

    #[derive(Serialize, Deserialize)]
    pub struct State {
        pub stable: StableState,

        #[serde(skip, default = "init_stable_data")]
        pub new_db: NewDb,
    }

    #[derive(Default, CandidType, Serialize, Deserialize, Clone)]
    pub struct StableState {
        pub controllers: Controllers,
        pub db: DbData,
    }

    #[derive(CandidType, Serialize, Deserialize, Clone)]
    pub struct Controller {
        pub created_at: u64,
        pub updated_at: u64,
    }

    pub type ControllerId = Principal;
    pub type Controllers = HashMap<ControllerId, Controller>;

    #[derive(CandidType, Serialize, Deserialize, Clone)]
    pub struct Entity {
        pub created_at: u64,
        pub updated_at: u64,
        pub data: Vec<u8>,
    }

    pub type Key = String;
    pub type Collection = BTreeMap<Key, Entity>;
    pub type Db = HashMap<Key, Collection>;

    #[derive(Default, CandidType, Serialize, Deserialize, Clone)]
    pub struct DbData {
        pub db: Db,
    }
}

pub mod stable {
    use crate::types::candid::Key;
    use candid::CandidType;
    use candid::Principal;
    use ic_stable_structures::memory_manager::VirtualMemory;
    use ic_stable_structures::DefaultMemoryImpl;
    use serde::Deserialize;

    #[derive(CandidType, Deserialize, Clone, PartialOrd, Ord, Eq, PartialEq)]
    pub struct MyPrincipal(pub(crate) Principal);

    #[derive(CandidType, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StableKey {
        pub collection_key: Key,
        pub entity_key: Key,
    }

    pub type Memory = VirtualMemory<DefaultMemoryImpl>;
}
