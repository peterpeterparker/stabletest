pub mod candid {
    use candid::CandidType;
    use serde::Deserialize;
    use std::collections::{BTreeMap, HashMap};
    use candid::Principal;

    #[derive(Default, Clone)]
    pub struct State {
        pub stable: StableState,
    }

    pub type ControllerId = Principal;
    pub type Key = String;
    pub type Collection = BTreeMap<Key, Entity>;

    #[derive(CandidType, Deserialize, Clone)]
    pub struct Entity {
        pub created_at: u64,
        pub updated_at: u64,
    }

    pub type Controllers = HashMap<ControllerId, Entity>;
    pub type Rules = HashMap<Key, Entity>;
    pub type Db = HashMap<Key, Collection>;

    #[derive(Default, CandidType, Deserialize, Clone)]
    pub struct DbStableState {
        pub db: Db,
        pub rules: Rules,
    }

    #[derive(Default, CandidType, Deserialize, Clone)]
    pub struct StableState {
        pub controllers: Controllers,
        pub db: DbStableState,
    }
}