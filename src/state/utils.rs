use cosmwasm_std::{Env, StdError, StdResult, Storage};
use serde::{Serialize};

use secret_toolkit::storage::{Keyset, Keymap};
use secret_toolkit::serialization::{Serde};
use secret_toolkit::storage::iter_options::{WithIter};
use serde::de::DeserializeOwned;
use ulid::Ulid;

/// `paging` method only present for ascend order
pub fn keyset_reverse_paging<'a, K, Ser>(keyset: &Keyset<'a, K, Ser, WithIter>, storage: &dyn Storage, start_page: u32, size: u32) -> StdResult<Vec<K>>
where
    K: Serialize + DeserializeOwned,
    Ser: Serde,
{
    let start_pos = start_page * size;

    let max_size = keyset.get_len(storage)?;

    if max_size == 0 {
        return Ok(vec![]);
    }

    if start_pos > max_size {
        return Err(StdError::not_found("out of bounds"));
    }

    keyset.iter(storage)?.rev()
        .skip(start_pos as usize)
        .take(size as usize)
        .collect()
}

/// `paging` method only present for ascend order
pub fn keymap_reverse_paging<'a, K, T, Ser>(keymap: &Keymap<'a, K, T, Ser, WithIter>, storage: &dyn Storage, start_page: u32, size: u32) -> StdResult<Vec<(K, T)>>
where
    K: Serialize + DeserializeOwned,
    T: Serialize + DeserializeOwned,
    Ser: Serde,
{
    let start_pos = start_page * size;

    let max_size = keymap.get_len(storage)?;

    if max_size == 0 {
        return Ok(vec![]);
    }

    if start_pos > max_size {
        return Err(StdError::not_found("out of bounds"));
    }

    keymap.iter(storage)?.rev()
        .skip(start_pos as usize)
        .take(size as usize)
        .collect()
}

pub fn get_generated_ulid(id_u64: u64, env: &Env) -> StdResult<String> {
    let random_u128 = u128::from_le_bytes(env.block.random.clone().unwrap().as_slice()[..16].try_into().unwrap());
    let ulid = Ulid::from_parts(env.block.time.nanos() / 1_000_000, random_u128 + id_u64 as u128);
    Ok(ulid.to_string())
}
