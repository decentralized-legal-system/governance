use std::borrow::Cow;
use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{BoundedStorable, Storable};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Law
{
    pub id: u128,
    pub version: u128,
    pub date_issued: String,
}

impl Storable for Law
{
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}


impl BoundedStorable for Law
{
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}