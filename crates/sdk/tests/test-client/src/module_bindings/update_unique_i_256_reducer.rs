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
pub struct UpdateUniqueI256Args {
    pub n: i256,
    pub data: i32,
}

impl Reducer for UpdateUniqueI256Args {
    const REDUCER_NAME: &'static str = "update_unique_i256";
}

#[allow(unused)]
pub fn update_unique_i_256(n: i256, data: i32) {
    UpdateUniqueI256Args { n, data }.invoke();
}

#[allow(unused)]
pub fn on_update_unique_i_256(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &i256, &i32) + Send + 'static,
) -> ReducerCallbackId<UpdateUniqueI256Args> {
    UpdateUniqueI256Args::on_reducer(move |__identity, __addr, __status, __args| {
        let UpdateUniqueI256Args { n, data } = __args;
        __callback(__identity, __addr, __status, n, data);
    })
}

#[allow(unused)]
pub fn once_on_update_unique_i_256(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &i256, &i32) + Send + 'static,
) -> ReducerCallbackId<UpdateUniqueI256Args> {
    UpdateUniqueI256Args::once_on_reducer(move |__identity, __addr, __status, __args| {
        let UpdateUniqueI256Args { n, data } = __args;
        __callback(__identity, __addr, __status, n, data);
    })
}

#[allow(unused)]
pub fn remove_on_update_unique_i_256(id: ReducerCallbackId<UpdateUniqueI256Args>) {
    UpdateUniqueI256Args::remove_on_reducer(id);
}
