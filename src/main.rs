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

mod lib;

use dialoguer::{Input, MultiSelect, Select};

use std::io::{Read, Write};

use crate::lib::SimpleDie;
use crate::lib::{Automatic, Roll, Step};
use crate::lib::{Console, Excel, Export};

fn main() -> Result<(), std::io::Error> {
    human_panic::setup_panic!();

    // TODO export and import dice
    match Select::new()
        .items(&["Einen Würfel entwerfen", "Würfeln", "Bedienungsanleitung"])
        .interact()
        .unwrap()
    {
        0 => create_die(),
        1 => roll_dice(),
        2 => todo!("help screen"),
        _ => unreachable!(),
    };

    print!("Beenden mit ENTER...");
    std::io::stdout().flush()?;
    let buf = &mut [0u8];
    std::io::stdin().read_exact(buf)?;

    Ok(())
}

/// create a die with options supplied via the interactive command line interface
fn create_die() {
    todo!("create dice")
}

/// calculate dice rolls
fn roll_dice() {
    let name: String = Input::new()
        .with_prompt("Gib dieser Stichprobe einen Namen")
        .with_initial_text("Stichprobe")
        .interact_text()
        .unwrap();

    let die: SimpleDie = Input::new()
        .with_prompt("Wähle einen Würfel")
        .with_initial_text("1d6")
        .interact_text()
        .unwrap();

    let mode = Select::new()
        .with_prompt("Was möchtest du mit dem Würfel tun?")
        .items(&["Manuelles Würfeln", "Würfel-Simulation"])
        .interact()
        .unwrap();

    let report = match mode {
        0 => Step.roll(&name, die),
        1 => {
            let total = Input::new()
                .with_prompt("Wie oft soll gewürfelt werden?")
                .interact_text()
                .unwrap();
            Automatic::new(total).roll(&name, die)
        }
        _ => unreachable!(),
    };

    let exports = MultiSelect::new()
        .items(&["Text", "Excel-Tabelle"])
        .defaults(&[true, false])
        .interact()
        .unwrap();

    for export in exports {
        match export {
            0 => Console.export(&report),
            1 => Excel::new(&name).export(&report),
            _ => unreachable!(),
        }
    }
}
