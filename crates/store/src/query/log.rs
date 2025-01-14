/*
 * Copyright (c) 2023 Stalwart Labs Ltd.
 *
 * This file is part of the Stalwart Mail Server.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of
 * the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 * in the LICENSE file at the top-level directory of this distribution.
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * You can be released from the requirements of the AGPLv3 license by
 * purchasing a commercial license. Please contact licensing@stalw.art
 * for more details.
*/

use utils::codec::leb128::Leb128Iterator;

use crate::{write::key::DeserializeBigEndian, Error, LogKey, Store};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Change {
    Insert(u64),
    Update(u64),
    ChildUpdate(u64),
    Delete(u64),
}

#[derive(Debug)]
pub struct Changes {
    pub changes: Vec<Change>,
    pub from_change_id: u64,
    pub to_change_id: u64,
}

#[derive(Debug)]
pub enum Query {
    All,
    Since(u64),
    SinceInclusive(u64),
    RangeInclusive(u64, u64),
}

impl Default for Changes {
    fn default() -> Self {
        Self {
            changes: Vec::with_capacity(10),
            from_change_id: 0,
            to_change_id: 0,
        }
    }
}

impl Store {
    pub async fn changes(
        &self,
        account_id: u32,
        collection: impl Into<u8>,
        query: Query,
    ) -> crate::Result<Changes> {
        let collection = collection.into();
        let (is_inclusive, from_change_id, to_change_id) = match query {
            Query::All => (true, 0, u64::MAX),
            Query::Since(change_id) => (false, change_id, u64::MAX),
            Query::SinceInclusive(change_id) => (true, change_id, u64::MAX),
            Query::RangeInclusive(from_change_id, to_change_id) => {
                (true, from_change_id, to_change_id)
            }
        };
        let from_key = LogKey {
            account_id,
            collection,
            change_id: from_change_id,
        };
        let to_key = LogKey {
            account_id,
            collection,
            change_id: to_change_id,
        };

        let mut changelog = self
            .iterate(
                Changes::default(),
                from_key,
                to_key,
                false,
                true,
                move |changelog, key, value| {
                    let change_id =
                        key.deserialize_be_u64(key.len() - std::mem::size_of::<u64>())?;
                    if is_inclusive || change_id != from_change_id {
                        if changelog.changes.is_empty() {
                            changelog.from_change_id = change_id;
                        }
                        changelog.to_change_id = change_id;
                        changelog.deserialize(value).ok_or_else(|| {
                            Error::InternalError(format!(
                                "Failed to deserialize changelog for [{}/{:?}]: [{:?}]",
                                account_id, collection, query
                            ))
                        })?;
                    }
                    Ok(true)
                },
            )
            .await?;

        if changelog.changes.is_empty() {
            changelog.from_change_id = from_change_id;
            changelog.to_change_id = if to_change_id != u64::MAX {
                to_change_id
            } else {
                from_change_id
            };
        }

        Ok(changelog)
    }
}

impl Changes {
    pub fn deserialize(&mut self, bytes: &[u8]) -> Option<()> {
        let mut bytes_it = bytes.iter();
        let total_inserts: usize = bytes_it.next_leb128()?;
        let total_updates: usize = bytes_it.next_leb128()?;
        let total_child_updates: usize = bytes_it.next_leb128()?;
        let total_deletes: usize = bytes_it.next_leb128()?;

        if total_inserts > 0 {
            for _ in 0..total_inserts {
                self.changes.push(Change::Insert(bytes_it.next_leb128()?));
            }
        }

        if total_updates > 0 || total_child_updates > 0 {
            'update_outer: for change_pos in 0..(total_updates + total_child_updates) {
                let id = bytes_it.next_leb128()?;
                let mut is_child_update = change_pos >= total_updates;

                for (idx, change) in self.changes.iter().enumerate() {
                    match change {
                        Change::Insert(insert_id) if *insert_id == id => {
                            // Item updated after inserted, no need to count this change.
                            continue 'update_outer;
                        }
                        Change::Update(update_id) if *update_id == id => {
                            // Move update to the front
                            is_child_update = false;
                            self.changes.remove(idx);
                            break;
                        }
                        Change::ChildUpdate(update_id) if *update_id == id => {
                            // Move update to the front
                            self.changes.remove(idx);
                            break;
                        }
                        _ => (),
                    }
                }

                self.changes.push(if !is_child_update {
                    Change::Update(id)
                } else {
                    Change::ChildUpdate(id)
                });
            }
        }

        if total_deletes > 0 {
            'delete_outer: for _ in 0..total_deletes {
                let id = bytes_it.next_leb128()?;

                'delete_inner: for (idx, change) in self.changes.iter().enumerate() {
                    match change {
                        Change::Insert(insert_id) if *insert_id == id => {
                            self.changes.remove(idx);
                            continue 'delete_outer;
                        }
                        Change::Update(update_id) | Change::ChildUpdate(update_id)
                            if *update_id == id =>
                        {
                            self.changes.remove(idx);
                            break 'delete_inner;
                        }
                        _ => (),
                    }
                }

                self.changes.push(Change::Delete(id));
            }
        }

        Some(())
    }
}

impl Change {
    pub fn id(&self) -> u64 {
        match self {
            Change::Insert(id) => *id,
            Change::Update(id) => *id,
            Change::ChildUpdate(id) => *id,
            Change::Delete(id) => *id,
        }
    }

    pub fn unwrap_id(self) -> u64 {
        match self {
            Change::Insert(id) => id,
            Change::Update(id) => id,
            Change::ChildUpdate(id) => id,
            Change::Delete(id) => id,
        }
    }
}
