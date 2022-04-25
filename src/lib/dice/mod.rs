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

// TODO mod digits;
mod simple;
// TODO mod weighted;

use counter::Counter;
pub use simple::SimpleDie;

use std::str::FromStr;

pub trait Die: FromStr + ToString + Clone + Sync {
    fn roll_vec(&self) -> Vec<u32>;

    fn roll_sum(&self) -> u32 {
        self.roll_vec().into_iter().sum()
    }

    fn roll_reduced<R>(&self, reducer: R) -> Option<u32>
    where
        R: FnMut(u32, u32) -> u32,
    {
        self.roll_vec().into_iter().reduce(reducer)
    }

    fn possible(&self) -> Vec<u32>;

    fn probabilities(&self) -> Counter<u32>;
}
