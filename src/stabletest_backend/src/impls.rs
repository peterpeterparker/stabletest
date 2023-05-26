use crate::types::candid::{Controllers, Entity, StableState};
use crate::types::stable::MyPrincipal;
use candid::{decode_one, encode_one};
use ic_stable_structures::{BoundedStorable, Storable};
use std::borrow::Cow;

impl Storable for StableState {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        decode_one(&bytes).unwrap()
    }
}

impl Storable for Entity {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        decode_one(&bytes).unwrap()
    }
}

impl BoundedStorable for Entity {
    const MAX_SIZE: u32 = 10 * 1024 * 1024; // 10 MB
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for MyPrincipal {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        decode_one(&bytes).unwrap()
    }
}

impl BoundedStorable for MyPrincipal {
    const MAX_SIZE: u32 = 29;
    const IS_FIXED_SIZE: bool = false;
}
