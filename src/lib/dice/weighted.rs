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
use rand::{distributions::Uniform, prelude::Distribution};
use std::str::FromStr;

#[derive(Debug, Copy)]
pub struct SimpleDie {
    sides: u16,
    times: u8,
    distr: Uniform<u16>,
}

impl SimpleDie {
    pub fn new(sides: u16, times: u8) -> Self {
        let distr = Uniform::new(1, sides + 1);
        Self {
            sides,
            times,
            distr,
        }
    }
}

impl ToString for SimpleDie {
    fn to_string(&self) -> String {
        format!("{}d{}", self.times, self.sides)
    }
}

impl FromStr for SimpleDie {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static ERR: &str = "Invalid string, must match `[1-9]+d[1-9]+`";
        let buffer = String::from(s.trim());
        let seperator = s.find('d').ok_or(ERR)?;
        let times = u8::from_str(&buffer[..seperator]).map_err(|_| ERR)?;
        let sides = u16::from_str(&buffer[seperator + 1..]).map_err(|_| ERR)?;

        Ok(Self::new(sides, times))
    }
}

impl super::Die for SimpleDie {
    fn roll_vec(&self) -> Vec<u32> {
        self.distr
            .sample_iter(rand::thread_rng())
            .take(self.times.into())
            .map(|n| n.into())
            .collect()
    }

    fn possible(&self) -> Vec<u32> {
        let times: u32 = self.times.into();
        let sides: u32 = self.sides.into();
        (times..(times * sides + 1u32)).collect()
    }

    fn probabilities(&self) -> Counter<u32> {
        let times: u32 = self.times.into();
        let sides: u32 = self.sides.into();
        (times..(times * sides + 1u32)).map(|d| (d, 1)).collect()
    }
}

impl Clone for SimpleDie {
    fn clone(&self) -> Self {
        Self::new(self.sides, self.times)
    }
}
