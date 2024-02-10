use lazy_static::lazy_static;
use regex::Regex;
use serde_json::from_slice;
use starknet_crypto::FieldElement;
use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    str::FromStr,
    sync::RwLock,
    vec,
};

use crate::api::structs::RoundTreeData;

use super::processor::read_airdrops;

/// Use RwLock to allow for mutable access to the data
lazy_static! {
    static ref ROUND_DATA: RwLock<Vec<RoundTreeData>> = RwLock::new(Vec::new());
}

pub fn get_all_data() -> Vec<RoundTreeData> {
    ROUND_DATA
        .read()
        .expect("Failed to acquire read lock")
        .clone()
}

pub fn update_api_data() {
    let mut data = ROUND_DATA.write().expect("Failed to acquire write lock");

    let drops = read_airdrops();

    *data = drops;
}