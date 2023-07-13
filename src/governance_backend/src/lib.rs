use crate::law::Law;
use crate::map::MAP;

mod map;
mod law;


#[ic_cdk_macros::query]
fn get_law(key: u128) -> Option<Law>
{
    MAP.with(|p| p.borrow().get(&key))
}

#[ic_cdk_macros::update]
fn set_law(key: u128, value: Law) -> Option<Law>
{
    MAP.with(|p| p.borrow_mut().insert(key, value))
}