use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Lotto {
	Lotto649: Vec<LottoData>
}
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct LottoData {
	Date: String,
	Numbers: Vec<i32>
}


fn main() {
	let json_loc = "E:\\VS Solutions\\Rust Solutions\\LotteryRust\\src\\Lotto649.json";
	
	let mut f = File::open(json_loc).expect("File not found");

	let mut lottery_json = String::new();
	f.read_to_string(&mut lottery_json).expect("Something went wrong");

	let lotto: Lotto = serde_json::from_str(&lottery_json).unwrap();

	let mut v = Vec::new();

	for i in 0..lotto.Lotto649.len() {
		v.push(&lotto.Lotto649[i].Numbers)
	}

	let flatten_lotto: Vec<i32> = v.iter().flat_map(|array| array.iter()).cloned().collect();

	let mut frequency: HashMap<i32, u32> = HashMap::new();
	for itm in flatten_lotto{
		*frequency.entry(itm).or_insert(0) += 1;
	}

	println!("{:?}", frequency);

}
