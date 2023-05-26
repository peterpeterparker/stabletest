pub mod candid {
    use candid::CandidType;
    use candid::Principal;
    use serde::Deserialize;
    use std::collections::{BTreeMap, HashMap};

    #[derive(Default, Clone, CandidType, Deserialize)]
    pub struct State {
        pub stable: StableState,
    }

    #[derive(Default, CandidType, Deserialize, Clone)]
    pub struct StableState {
        pub controllers: Controllers,
        pub db: DbStableState,
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
}
