use std::{
    collections::{BTreeMap, BTreeSet},
    io,
};

use staff_list::*;

fn main() {
    // ソートが面倒なのでHashMapもVecも使わなかった。重複なしの前提で。すまん。
    let mut staff_list: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    loop {
        eprint!("> ");

        let mut command_line = String::new();

        let bytes = io::stdin()
            .read_line(&mut command_line)
            .expect("Failed to read line");

        if bytes == 0 {
            break;
        }

        let mut tokens = command_line.trim().split_whitespace();

        let command = match tokens.next() {
            None => {
                usage();
                continue;
            }
            Some(token) => token,
        };

        match command {
            "usage" | "help" => usage(),
            "add" => add(&mut staff_list, &mut tokens),
            "list" => list(&staff_list, &mut tokens),
            _ => usage(),
        }
    }
}
