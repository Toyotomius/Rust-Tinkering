use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug)]
struct Lotto {
    #[serde(alias = "Lotto649")]
    lotto649: Vec<LottoData>,
}
#[derive(Serialize, Deserialize, Debug)]
struct LottoData {
    #[serde(alias = "Date")]
    date: String,

    #[serde(alias = "Numbers")]
    numbers: Vec<i32>,
}

pub fn lotto649(i: &i32) {
    let start = Instant::now();

    let json_loc = "LottoExample.json";

    let write_loc = "649Singles.txt";
    let mut file = match File::create(write_loc) {
        Err(why) => panic!("Couldn't create {}: {}", write_loc, why.description()),
        Ok(file) => file,
    };
    let mut f = File::open(json_loc).expect("File not found");

    let mut lottery_json = String::new();
    f.read_to_string(&mut lottery_json)
        .expect("Something went wrong");

    let lotto: Lotto = serde_json::from_str(&lottery_json).unwrap();

    let mut v = Vec::new();

    for i in 0..lotto.lotto649.len() {
        v.push(&lotto.lotto649[i].numbers)
    }

    let flatten_lotto: Vec<i32> = v.iter().flat_map(|array| array.iter()).cloned().collect();

    let mut frequency: HashMap<i32, u32> = HashMap::new();
    for itm in flatten_lotto {
        *frequency.entry(itm).or_insert(0) += 1;
    }

    let mut sorted_vec: Vec<(&i32, &u32)> = frequency.iter().collect();
    sorted_vec.sort_by(|a, b| b.1.cmp(a.1));

    match write!(file, "{:?}\n", sorted_vec) {
        Err(why) => panic!("Couldn't write to {}: {}", write_loc, why.description()),
        Ok(_) => println!(
            "Successfully wrote to {}, #{:?} in: {} ms",
            write_loc,
            i,
            start.elapsed().as_millis()
        ),
    }
}
