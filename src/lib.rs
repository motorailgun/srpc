use serde::{Deserialize, Serialize};
use rmp_serde;
// use std::collections;

pub fn serialize<T: Clone + Serialize>(message: &T) -> Vec<u8> {
    rmp_serde::to_vec(message).unwrap()
}

pub fn deserialize<'a, T: Deserialize<'a>>(message: &'a Vec<u8>) -> T {
    rmp_serde::from_slice::<T>(message).unwrap()
}
