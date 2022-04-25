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

use counter::Counter;
use dialoguer::Confirm;
use easy_parallel::Parallel;
use num_cpus;
use std::ops::{Add, AddAssign};

use super::export::Report;
use super::Die;

pub trait Roll {
    fn roll<D>(self, name: &str, die: D) -> Report
    where
        D: Die;
}

pub struct Step;

impl Roll for Step {
    fn roll<D>(self, name: &str, die: D) -> Report
    where
        D: Die,
    {
        let mut total = Counter::new();
        loop {
            let rolls = die.roll_vec();
            let mut sum = 0;

            for roll in rolls.iter() {
                sum.add_assign(roll);
                total[roll] += 1;
            }

            let rolls: Vec<_> = rolls.into_iter().map(|x| x.to_string()).collect();
            println!("{} = {}", rolls.join(" + "), sum);
            if !Confirm::new()
                .with_prompt("Nochmal würfeln?")
                .default(true)
                .interact()
                .unwrap()
            {
                break;
            }
        }

        Report::new(name, die, total, None)
    }
}

pub struct Automatic(usize);

impl Automatic {
    pub fn new(total: usize) -> Self {
        Self(total)
    }
}

impl Roll for Automatic {
    fn roll<D>(self, name: &str, die: D) -> Report
    where
        D: Die,
    {
        let threads = (self.0 >> 10 | 1).max(num_cpus::get());
        let rolls = (self.0 / threads, self.0 % threads);

        let total = Parallel::new()
            .each(0..threads, |i| {
                let rolls = rolls.0 + if rolls.1 > i { 1 } else { 0 };

                let mut counter = Counter::new();
                counter.update((0..rolls).map(|_| die.roll_sum()));
                return counter;
            })
            .run()
            .into_iter()
            .reduce(Add::add);

        Report::new(name, die, total.unwrap_or_else(Counter::new), Some(self.0))
    }
}
