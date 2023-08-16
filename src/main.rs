use std::fs;
use crate::json_data::Root;

mod json_data;
mod replay_data;
mod data_ttr;
mod data_ttrm;

pub mod enums {
	pub mod event_type;
	pub mod bag_type;
	pub mod garbage_blocking_type;
	pub mod garbage_type;
	pub mod kickset_type;
	pub mod passthrough_type;
	pub mod replay_type;
	pub mod spin_bonuses_type;
}

pub mod json_class {
	pub mod event {
		pub mod Event;
		pub mod EventEnd;
	}
}

fn main() {
	println!("Hello, world!");

	let path = r#"C:\Users\CSDotNET\Downloads\eEieLg6WR.ttrm"#;
	let content = fs::read_to_string(&path);

	println!("aiueo");

	match content {
		Ok(content) => {
			let data = serde_json::from_str::<Root>(&content);
			match data {
				Ok(data) => {}
				Err(err) => {
					return;
				}
			}
		}
		Err(err) => {
			return;
		}
	}
}
