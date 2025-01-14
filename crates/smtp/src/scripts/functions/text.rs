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

use sieve::{runtime::Variable, Context};

use super::ApplyString;

pub fn fn_trim<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    v[0].transform(|s| Some(s.trim()))
}

pub fn fn_len<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    match &v[0] {
        Variable::String(s) => s.len(),
        Variable::StringRef(s) => s.len(),
        Variable::Array(a) => a.len(),
        Variable::ArrayRef(a) => a.len(),
        v => v.to_string().len(),
    }
    .into()
}

pub fn fn_is_ascii<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    match &v[0] {
        Variable::String(s) => s.chars().all(|c| c.is_ascii()),
        Variable::StringRef(s) => s.chars().all(|c| c.is_ascii()),
        Variable::Integer(_) | Variable::Float(_) => true,
        Variable::Array(a) => a.iter().all(|v| match v {
            Variable::String(s) => s.chars().all(|c| c.is_ascii()),
            Variable::StringRef(s) => s.chars().all(|c| c.is_ascii()),
            _ => true,
        }),
        Variable::ArrayRef(a) => a.iter().all(|v| match v {
            Variable::String(s) => s.chars().all(|c| c.is_ascii()),
            Variable::StringRef(s) => s.chars().all(|c| c.is_ascii()),
            _ => true,
        }),
    }
    .into()
}

pub fn fn_to_lowercase<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    v[0].to_cow().to_lowercase().to_string().into()
}

pub fn fn_to_uppercase<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    v[0].to_cow().to_uppercase().to_string().into()
}

pub fn fn_is_uppercase<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    v[0].to_cow()
        .as_ref()
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| c.is_uppercase())
        .into()
}

pub fn fn_is_lowercase<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    v[0].to_cow()
        .as_ref()
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| c.is_lowercase())
        .into()
}

pub fn fn_tokenize_words<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    match &v[0] {
        Variable::StringRef(s) => s
            .split_whitespace()
            .filter(|word| word.chars().all(|c| c.is_alphanumeric()))
            .map(Variable::from)
            .collect::<Vec<_>>(),
        Variable::String(s) => s
            .split_whitespace()
            .filter(|word| word.chars().all(|c| c.is_alphanumeric()))
            .map(|word| Variable::from(word.to_string()))
            .collect::<Vec<_>>(),
        v => v
            .to_string()
            .split_whitespace()
            .filter(|word| word.chars().all(|c| c.is_alphanumeric()))
            .map(|word| Variable::from(word.to_string()))
            .collect::<Vec<_>>(),
    }
    .into()
}

pub fn fn_max_line_len<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    match &v[0] {
        Variable::String(s) => s.lines().map(|l| l.len()).max().unwrap_or(0),
        Variable::StringRef(s) => s.lines().map(|l| l.len()).max().unwrap_or(0),
        Variable::Integer(_) | Variable::Float(_) => 0,
        Variable::Array(a) => a.iter().map(|v| v.to_cow().len()).max().unwrap_or(0),
        Variable::ArrayRef(a) => a.iter().map(|v| v.to_cow().len()).max().unwrap_or(0),
    }
    .into()
}

pub fn fn_count_spaces<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    v[0].to_cow()
        .as_ref()
        .chars()
        .filter(|c| c.is_whitespace())
        .count()
        .into()
}

pub fn fn_count_uppercase<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    v[0].to_cow()
        .as_ref()
        .chars()
        .filter(|c| c.is_alphabetic() && c.is_uppercase())
        .count()
        .into()
}

pub fn fn_count_lowercase<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    v[0].to_cow()
        .as_ref()
        .chars()
        .filter(|c| c.is_alphabetic() && c.is_lowercase())
        .count()
        .into()
}

pub fn fn_count_chars<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    v[0].to_cow().as_ref().chars().count().into()
}

pub fn fn_eq_ignore_case<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    v[0].to_cow()
        .eq_ignore_ascii_case(v[1].to_cow().as_ref())
        .into()
}

pub fn fn_contains<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    match &v[0] {
        Variable::String(s) => s.contains(v[1].to_cow().as_ref()),
        Variable::StringRef(s) => s.contains(v[1].to_cow().as_ref()),
        Variable::Array(arr) => arr.contains(&v[1]),
        Variable::ArrayRef(arr) => arr.contains(&v[1]),
        val => val.to_string().contains(v[1].to_cow().as_ref()),
    }
    .into()
}

pub fn fn_contains_ignore_case<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    let needle = v[1].to_cow();
    match &v[0] {
        Variable::String(s) => s.to_lowercase().contains(&needle.to_lowercase()),
        Variable::StringRef(s) => s.to_lowercase().contains(&needle.to_lowercase()),
        Variable::Array(arr) => arr.iter().any(|v| match v {
            Variable::String(s) => s.eq_ignore_ascii_case(needle.as_ref()),
            Variable::StringRef(s) => s.eq_ignore_ascii_case(needle.as_ref()),
            _ => false,
        }),
        Variable::ArrayRef(arr) => arr.iter().any(|v| match v {
            Variable::String(s) => s.eq_ignore_ascii_case(needle.as_ref()),
            Variable::StringRef(s) => s.eq_ignore_ascii_case(needle.as_ref()),
            _ => false,
        }),
        val => val.to_string().contains(needle.as_ref()),
    }
    .into()
}

pub fn fn_starts_with<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    v[0].to_cow().starts_with(v[1].to_cow().as_ref()).into()
}

pub fn fn_ends_with<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    v[0].to_cow().ends_with(v[1].to_cow().as_ref()).into()
}

pub fn fn_lines<'x>(_: &'x Context<'x>, mut v: Vec<Variable<'x>>) -> Variable<'x> {
    match v.remove(0) {
        Variable::StringRef(s) => s.lines().map(Variable::from).collect::<Vec<_>>().into(),
        Variable::String(s) => s
            .lines()
            .map(|s| Variable::String(s.to_string()))
            .collect::<Vec<_>>()
            .into(),
        val => val,
    }
}

pub fn fn_substring<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    v[0].to_cow()
        .chars()
        .skip(v[1].to_usize())
        .take(v[2].to_usize())
        .collect::<String>()
        .into()
}

pub fn fn_split<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    match &v[0] {
        Variable::StringRef(s) => s
            .split(v[1].to_cow().as_ref())
            .map(Variable::from)
            .collect::<Vec<_>>()
            .into(),
        Variable::String(s) => s
            .split(v[1].to_cow().as_ref())
            .map(|s| Variable::String(s.to_string()))
            .collect::<Vec<_>>()
            .into(),
        val => val
            .to_string()
            .split(v[1].to_cow().as_ref())
            .map(|s| Variable::String(s.to_string()))
            .collect::<Vec<_>>()
            .into(),
    }
}

/**
 * `levenshtein-rs` - levenshtein
 *
 * MIT licensed.
 *
 * Copyright (c) 2016 Titus Wormer <tituswormer@gmail.com>
 */
pub fn fn_levenshtein_distance<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    let a = v[0].to_cow();
    let b = v[1].to_cow();

    let mut result = 0;

    /* Shortcut optimizations / degenerate cases. */
    if a == b {
        return result.into();
    }

    let length_a = a.chars().count();
    let length_b = b.chars().count();

    if length_a == 0 {
        return length_b.into();
    } else if length_b == 0 {
        return length_a.into();
    }

    /* Initialize the vector.
     *
     * This is why it’s fast, normally a matrix is used,
     * here we use a single vector. */
    let mut cache: Vec<usize> = (1..).take(length_a).collect();
    let mut distance_a;
    let mut distance_b;

    /* Loop. */
    for (index_b, code_b) in b.chars().enumerate() {
        result = index_b;
        distance_a = index_b;

        for (index_a, code_a) in a.chars().enumerate() {
            distance_b = if code_a == code_b {
                distance_a
            } else {
                distance_a + 1
            };

            distance_a = cache[index_a];

            result = if distance_a > result {
                if distance_b > result {
                    result + 1
                } else {
                    distance_b
                }
            } else if distance_b > distance_a {
                distance_a + 1
            } else {
                distance_b
            };

            cache[index_a] = result;
        }
    }

    result.into()
}

pub fn fn_detect_language<'x>(_: &'x Context<'x>, v: Vec<Variable<'x>>) -> Variable<'x> {
    whatlang::detect_lang(v[0].to_cow().as_ref())
        .map(|l| l.code())
        .unwrap_or("unknown")
        .into()
}
