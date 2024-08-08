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
pub struct InsertVecI32Args {
    pub n: Vec<i32>,
}

impl Reducer for InsertVecI32Args {
    const REDUCER_NAME: &'static str = "insert_vec_i32";
}

#[allow(unused)]
pub fn insert_vec_i_32(n: Vec<i32>) {
    InsertVecI32Args { n }.invoke();
}

#[allow(unused)]
pub fn on_insert_vec_i_32(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &Vec<i32>) + Send + 'static,
) -> ReducerCallbackId<InsertVecI32Args> {
    InsertVecI32Args::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertVecI32Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn once_on_insert_vec_i_32(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &Vec<i32>) + Send + 'static,
) -> ReducerCallbackId<InsertVecI32Args> {
    InsertVecI32Args::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertVecI32Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn remove_on_insert_vec_i_32(id: ReducerCallbackId<InsertVecI32Args>) {
    InsertVecI32Args::remove_on_reducer(id);
}
