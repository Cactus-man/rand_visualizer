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

use super::Die;

// IDEA allow bases with #[feature]

/// Counts the number of digits in decimal representation as the rolled value
struct DigitCountDie<D>
where
    D: Die,
{
    die: D,
}

/// Uses the first digit of the rolled number
struct FirstDigitDie<D>
where
    D: Die,
{
    die: D,
}

/// Uses the first digit of the rolled number
struct LastDigitDie<D>
where
    D: Die,
{
    die: D,
}
