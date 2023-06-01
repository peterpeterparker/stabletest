use crate::types::candid::{Collection, DbData, Entity, Key, NewDb};
use crate::types::stable::StableKey;
use crate::STATE;

/// Candid

pub fn set_candid_entity(collection: &Key, key: &Key, controller: &Entity) {
    STATE.with(|state| {
        set_candid_entity_impl(
            collection,
            key,
            controller,
            &mut state.borrow_mut().stable.db,
        )
    })
}

fn set_candid_entity_impl(collection: &Key, key: &Key, entity: &Entity, state: &mut DbData) {
    let col = state.db.get_mut(collection);

    match col {
        None => {
            let mut new_col = Collection::new();
            new_col.insert(key.clone(), entity.clone());
            state.db.insert(collection.clone(), new_col);
        }
        Some(existing_col) => {
            existing_col.insert(key.clone(), entity.clone());
        }
    }
}

pub fn get_candid_entity(collection: &Key, key: &Key) -> Option<Entity> {
    STATE.with(|state| get_candid_entity_impl(collection, key, &state.borrow_mut().stable.db))
}

fn get_candid_entity_impl(collection: &Key, key: &Key, state: &DbData) -> Option<Entity> {
    let col = state.db.get(collection);

    match col {
        None => None,
        Some(col) => {
            let entity = col.get(key);
            match entity {
                None => None,
                Some(entity) => Some(entity.clone()),
            }
        }
    }
}

/// Stable

pub fn set_stable_entity(collection: &Key, key: &Key, controller: &Entity) {
    STATE.with(|state| {
        set_stable_entity_impl(collection, key, controller, &mut state.borrow_mut().new_db)
    })
}

fn set_stable_entity_impl(collection: &Key, key: &Key, entity: &Entity, new_db: &mut NewDb) {
    let stable_key = StableKey {
        collection_key: collection.clone(),
        entity_key: key.clone(),
    };

    new_db.insert(stable_key, entity.clone());
}

pub fn get_stable_entity(collection: &Key, key: &Key) -> Option<Entity> {
    STATE.with(|state| get_stable_entity_impl(collection, key, &state.borrow_mut().new_db))
}

fn get_stable_entity_impl(collection: &Key, key: &Key, new_db: &NewDb) -> Option<Entity> {
    let stable_key = StableKey {
        collection_key: collection.clone(),
        entity_key: key.clone(),
    };

    new_db.get(&stable_key)
}
