// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused_imports)]
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, i256, ser::Serialize, u256},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address, ScheduleAt,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UniqueString {
    pub s: String,
    pub data: i32,
}

impl TableType for UniqueString {
    const TABLE_NAME: &'static str = "UniqueString";
    type ReducerEvent = super::ReducerEvent;
}

impl UniqueString {
    #[allow(unused)]
    pub fn filter_by_s(s: String) -> TableIter<Self> {
        Self::filter(|row| row.s == s)
    }
    #[allow(unused)]
    pub fn find_by_s(s: String) -> Option<Self> {
        Self::find(|row| row.s == s)
    }
    #[allow(unused)]
    pub fn filter_by_data(data: i32) -> TableIter<Self> {
        Self::filter(|row| row.data == data)
    }
}
