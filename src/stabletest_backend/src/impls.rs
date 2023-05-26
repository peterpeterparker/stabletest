use crate::types::candid::State;
use candid::{decode_one, encode_one};
use ic_stable_structures::Storable;
use std::borrow::Cow;

impl Storable for State {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        decode_one(&bytes).unwrap()
    }
}
