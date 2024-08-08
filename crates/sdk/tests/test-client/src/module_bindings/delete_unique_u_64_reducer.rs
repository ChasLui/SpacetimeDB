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
pub struct DeleteUniqueU64Args {
    pub n: u64,
}

impl Reducer for DeleteUniqueU64Args {
    const REDUCER_NAME: &'static str = "delete_unique_u64";
}

#[allow(unused)]
pub fn delete_unique_u_64(n: u64) {
    DeleteUniqueU64Args { n }.invoke();
}

#[allow(unused)]
pub fn on_delete_unique_u_64(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &u64) + Send + 'static,
) -> ReducerCallbackId<DeleteUniqueU64Args> {
    DeleteUniqueU64Args::on_reducer(move |__identity, __addr, __status, __args| {
        let DeleteUniqueU64Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn once_on_delete_unique_u_64(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &u64) + Send + 'static,
) -> ReducerCallbackId<DeleteUniqueU64Args> {
    DeleteUniqueU64Args::once_on_reducer(move |__identity, __addr, __status, __args| {
        let DeleteUniqueU64Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn remove_on_delete_unique_u_64(id: ReducerCallbackId<DeleteUniqueU64Args>) {
    DeleteUniqueU64Args::remove_on_reducer(id);
}
