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
pub struct NoOpSucceedsArgs {}

impl Reducer for NoOpSucceedsArgs {
    const REDUCER_NAME: &'static str = "no_op_succeeds";
}

#[allow(unused)]
pub fn no_op_succeeds() {
    NoOpSucceedsArgs {}.invoke();
}

#[allow(unused)]
pub fn on_no_op_succeeds(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status) + Send + 'static,
) -> ReducerCallbackId<NoOpSucceedsArgs> {
    NoOpSucceedsArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let NoOpSucceedsArgs {} = __args;
        __callback(__identity, __addr, __status);
    })
}

#[allow(unused)]
pub fn once_on_no_op_succeeds(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status) + Send + 'static,
) -> ReducerCallbackId<NoOpSucceedsArgs> {
    NoOpSucceedsArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let NoOpSucceedsArgs {} = __args;
        __callback(__identity, __addr, __status);
    })
}

#[allow(unused)]
pub fn remove_on_no_op_succeeds(id: ReducerCallbackId<NoOpSucceedsArgs>) {
    NoOpSucceedsArgs::remove_on_reducer(id);
}
