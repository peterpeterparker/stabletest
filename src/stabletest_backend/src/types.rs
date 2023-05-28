pub mod candid {
    use candid::CandidType;
    use candid::Principal;
    use serde::Deserialize;
    use std::collections::{BTreeMap, HashMap};

    #[derive(Default, Clone)]
    pub struct State {
        pub stable: StableState,
    }

    #[derive(Default, CandidType, Deserialize, Clone)]
    pub struct StableState {
        pub controllers: Controllers,
        pub db: DbData,
    }

    #[derive(CandidType, Deserialize, Clone)]
    pub struct Controller {
        pub created_at: u64,
        pub updated_at: u64,
    }

    pub type ControllerId = Principal;
    pub type Controllers = HashMap<ControllerId, Controller>;

    #[derive(CandidType, Deserialize, Clone)]
    pub struct Entity {
        pub created_at: u64,
        pub updated_at: u64,
        pub data: Vec<u8>,
    }

    pub type Key = String;
    pub type Collection = BTreeMap<Key, Entity>;
    pub type Db = HashMap<Key, Collection>;

    #[derive(Default, CandidType, Deserialize, Clone)]
    pub struct DbData {
        pub db: Db,
    }
}

pub mod stable {
    use crate::types::candid::Key;
    use candid::CandidType;
    use candid::Principal;
    use serde::Deserialize;

    #[derive(CandidType, Deserialize, Clone, PartialOrd, Ord, Eq, PartialEq)]
    pub struct MyPrincipal(pub(crate) Principal);

    #[derive(CandidType, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StableKey {
        pub collection_key: Key,
        pub entity_key: Key,
    }
}
