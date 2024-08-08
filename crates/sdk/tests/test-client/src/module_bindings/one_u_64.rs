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
pub struct OneU64 {
    pub n: u64,
}

impl TableType for OneU64 {
    const TABLE_NAME: &'static str = "OneU64";
    type ReducerEvent = super::ReducerEvent;
}

impl OneU64 {
    #[allow(unused)]
    pub fn filter_by_n(n: u64) -> TableIter<Self> {
        Self::filter(|row| row.n == n)
    }
}
