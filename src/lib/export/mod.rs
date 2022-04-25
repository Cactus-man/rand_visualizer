// Copyright 2022 Paul Bühne
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod histogram;
mod spreadsheet;
mod text;

use counter::Counter;

use crate::lib::dice::Die;

// IDEA impl IntoIter for Report, iterate over ReportData
// struct ReportData {
//     dots: u32,
//     occurences: usize,
//     rel: (Fraction, f64),
//     expected: (Fraction, f64),
// }

pub struct Report<'s> {
    name: &'s str,
    total: usize,
    counts: Vec<(u32, usize)>,
}

impl<'s> Report<'s> {
    pub fn new<D>(name: &'s str, die: D, mut counts: Counter<u32>, total: Option<usize>) -> Self
    where
        D: Die,
    {
        let total = total.unwrap_or_else(|| counts.iter().map(|(_, &n)| n).sum());
        counts.extend(die.possible().iter().map(|&n| (n.into(), 0)));
        let mut counts: Vec<(u32, usize)> = counts.into_iter().collect();
        counts.sort_by_cached_key(|&(d, _)| d);

        Self {
            name,
            total,
            counts,
        }
    }

    pub fn with_percentages(&self) -> Vec<(u32, usize, f64)> {
        self.counts
            .iter()
            .map(|&(d, n)| (d, n, n as f64 / self.total as f64 * 100.))
            .collect()
    }
}

pub trait Export {
    fn export<'s>(self, report: &'s Report<'s>);
}

pub use spreadsheet::Excel;
pub use text::Console;
