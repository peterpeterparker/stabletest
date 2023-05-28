use crate::types::candid::{Controller, Entity, StableState};
use crate::types::stable::{MyPrincipal, StableKey};
use candid::{decode_one, encode_one, Principal};
use ic_cdk::print;
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

impl Storable for Controller {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        decode_one(&bytes).unwrap()
    }
}

impl BoundedStorable for Controller {
    const MAX_SIZE: u32 = 10 * 1024 * 1024; // 10 MB
    const IS_FIXED_SIZE: bool = false;
}

/// Source: https://forum.dfinity.org/t/increased-canister-smart-contract-memory/6148/160?u=peterparker
impl Storable for MyPrincipal {
    fn to_bytes(&self) -> Cow<[u8]> {
        print("MyPrincipal to_bytes.");
        Cow::Borrowed(self.0.as_slice())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        print("MyPrincipal from_bytes.");
        Self(Principal::from_slice(bytes.as_ref()))
    }
}

impl BoundedStorable for MyPrincipal {
    const MAX_SIZE: u32 = 29;
    const IS_FIXED_SIZE: bool = false;
}

impl From<&Principal> for MyPrincipal {
    fn from(principal: &Principal) -> Self {
        MyPrincipal(*principal)
    }
}

///

impl Storable for Entity {
    fn to_bytes(&self) -> Cow<[u8]> {
        print("Entity to_bytes.");
        Cow::Owned(encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        print("Entity from_bytes.");
        decode_one(&bytes).unwrap()
    }
}

impl BoundedStorable for Entity {
    const MAX_SIZE: u32 = 10 * 1024 * 1024; // 10 MB
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for StableKey {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        decode_one(&bytes).unwrap()
    }
}

impl BoundedStorable for StableKey {
    const MAX_SIZE: u32 = 10 * 1024 * 1024; // 10 MB
    const IS_FIXED_SIZE: bool = false;
}
