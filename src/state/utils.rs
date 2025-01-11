use cosmwasm_std::{StdError, StdResult, Storage, Timestamp};
use serde::{Serialize};

use secret_toolkit::storage::{Keyset, Keymap};
use secret_toolkit::serialization::{Serde};
use secret_toolkit::storage::iter_options::{WithIter};
use serde::de::DeserializeOwned;
use sqids::Sqids;

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

pub fn get_generated_sqid(id_u64: u64, block_time: Timestamp) -> StdResult<String> {
    let sqids = Sqids::default();
    Ok(sqids.encode(&[id_u64, block_time.nanos()]).unwrap())
}
