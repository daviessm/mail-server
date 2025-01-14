/*
 * Copyright (c) 2023 Stalwart Labs Ltd.
 *
 * This file is part of Stalwart Mail Server.
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

use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use sieve::{runtime::Variable, Context};

pub fn fn_count<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    match &v[0] {
        Variable::Array(a) => a.len(),
        Variable::ArrayRef(a) => a.len(),
        _ => 1,
    }
    .into()
}

pub fn fn_sort<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    let is_asc = v[1].to_bool();
    let mut arr = v.into_iter().next().unwrap().into_array();
    if is_asc {
        arr.sort_unstable_by(|a, b| b.cmp(a));
    } else {
        arr.sort_unstable();
    }
    arr.into()
}

pub fn fn_dedup<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    let arr = v.into_iter().next().unwrap().into_array();
    let mut result = Vec::with_capacity(arr.len());

    for item in arr {
        if !result.contains(&item) {
            result.push(item);
        }
    }

    result.into()
}

pub fn fn_cosine_similarity<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    let mut word_freq: HashMap<Cow<str>, [u32; 2]> = HashMap::new();

    for (idx, var) in v.into_iter().enumerate() {
        match var {
            Variable::Array(l) => {
                for item in l {
                    word_freq.entry(item.into_cow()).or_insert([0, 0])[idx] += 1;
                }
            }
            Variable::ArrayRef(l) => {
                for item in l {
                    word_freq.entry(item.to_cow()).or_insert([0, 0])[idx] += 1;
                }
            }
            _ => {
                for char in var.to_cow().chars() {
                    word_freq.entry(char.to_string().into()).or_insert([0, 0])[idx] += 1;
                }
            }
        }
    }

    let mut dot_product = 0;
    let mut magnitude_a = 0;
    let mut magnitude_b = 0;

    for (_word, count) in word_freq.iter() {
        dot_product += count[0] * count[1];
        magnitude_a += count[0] * count[0];
        magnitude_b += count[1] * count[1];
    }

    if magnitude_a != 0 && magnitude_b != 0 {
        dot_product as f64 / (magnitude_a as f64).sqrt() / (magnitude_b as f64).sqrt()
    } else {
        0.0
    }
    .into()
}

pub fn fn_jaccard_similarity<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    let mut word_freq = [HashSet::new(), HashSet::new()];

    for (idx, var) in v.into_iter().enumerate() {
        match var {
            Variable::Array(l) => {
                for item in l {
                    word_freq[idx].insert(item.into_cow());
                }
            }
            Variable::ArrayRef(l) => {
                for item in l {
                    word_freq[idx].insert(item.to_cow());
                }
            }
            _ => {
                for char in var.to_cow().chars() {
                    word_freq[idx].insert(char.to_string().into());
                }
            }
        }
    }

    let intersection_size = word_freq[0].intersection(&word_freq[1]).count();
    let union_size = word_freq[0].union(&word_freq[1]).count();

    if union_size != 0 {
        intersection_size as f64 / union_size as f64
    } else {
        0.0
    }
    .into()
}
