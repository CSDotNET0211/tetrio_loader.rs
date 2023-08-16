use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::enums::*;
use crate::enums::event_type::EventType;
use crate::enums::bag_type::BagType;
use crate::enums::spin_bonuses_type::SpinBonusesType;
use crate::enums::replay_type::ReplayType;
use crate::enums::garbage_blocking_type::GarbageBlockingType;
use crate::enums::kickset_type::KicksetType;
use crate::enums::garbage_type::GarbageType;
use crate::enums::passthrough_type::PassthroughType;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
	#[serde(rename = "_id")]
	pub id: String,
	pub endcontext: Vec<Endcontext>,
	pub gametype: String,
	pub ismulti: Option<bool>,
	pub mt: Option<i64>,
	pub shortid: String,
	pub ts: String,
	pub verified: Option<bool>,
	pub data: Vec<Daum>,
	pub back: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endcontext {
	pub user: Option<User>,
	pub handling: Option<Handling>,
	pub active: Option<bool>,
	pub success: Option<bool>,
	pub inputs: Option<i64>,
	pub piecesplaced: Option<i64>,
	pub naturalorder: Option<i64>,
	pub score: Option<i64>,
	pub wins: Option<i64>,
	pub points: Option<Points>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
	#[serde(rename = "_id")]
	pub id: String,
	pub username: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Points {
	pub primary: Option<i64>,
	pub secondary: Option<f64>,
	pub tertiary: Option<f64>,
	pub extra: Option<Extra>,
	pub secondary_avg_tracking: Option<Vec<f64>>,
	pub tertiary_avg_tracking: Option<Vec<f64>>,
	pub extra_avg_tracking: Option<ExtraAvgTracking>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extra {
	pub vs: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtraAvgTracking {
	#[serde(rename = "aggregatestats___vsscore")]
	pub aggregatestats_vsscore: Option<Vec<f64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
	pub board: Option<Vec<Board>>,
	pub replays: Option<Vec<Replay>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Board {
	pub user: Option<User>,
	pub active: Option<bool>,
	pub success: Option<bool>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Replay {
	pub frames: Option<i64>,
	pub events: Option<Vec<Event>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
	pub frame: i64,
	#[serde(rename = "type")]
	pub type_field: EventType,
	pub data: Option<Data>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
	pub successful: Option<bool>,
	pub gameoverreason: Option<Value>,
	pub options: Option<Options>,
	pub stats: Option<Stats>,
	#[serde(default)]
	pub targets: Vec<Value>,
	pub fire: Option<i64>,
	pub game: Option<Game>,
	pub killer: Option<Killer>,
	pub aggregatestats: Option<Aggregatestats>,
	pub id: Option<Value>,
	pub frame: Option<i64>,
	#[serde(rename = "type")]
	pub type_field: Option<String>,
	pub data: Option<Value>,
	pub key: Option<String>,
	pub subframe: Option<f64>,
	pub reason: Option<String>,
	pub export: Option<Export>,
	pub hoisted: Option<bool>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
	pub version: Option<i64>,
	#[serde(rename = "seed_random")]
	pub seed_random: Option<bool>,
	pub seed: Option<i64>,
	pub g: Option<f64>,
	pub stock: Option<i64>,
	pub countdown: Option<bool>,
	#[serde(rename = "countdown_count")]
	pub countdown_count: Option<i64>,
	#[serde(rename = "countdown_interval")]
	pub countdown_interval: Option<i64>,
	pub precountdown: Option<i64>,
	pub prestart: Option<i64>,
	pub mission: Option<String>,
	#[serde(rename = "mission_type")]
	pub mission_type: Option<String>,
	pub zoominto: Option<String>,
	#[serde(rename = "slot_counter1")]
	pub slot_counter1: Option<String>,
	#[serde(rename = "slot_counter2")]
	pub slot_counter2: Option<String>,
	#[serde(rename = "slot_counter3")]
	pub slot_counter3: Option<String>,
	#[serde(rename = "slot_counter4")]
	pub slot_counter4: Option<Value>,
	#[serde(rename = "slot_counter5")]
	pub slot_counter5: Option<String>,
	#[serde(rename = "slot_bar1")]
	pub slot_bar1: Option<String>,
	#[serde(rename = "display_fire")]
	pub display_fire: Option<bool>,
	#[serde(rename = "display_username")]
	pub display_username: Option<bool>,
	pub hasgarbage: Option<bool>,
	pub neverstopbgm: Option<bool>,
	#[serde(rename = "display_next")]
	pub display_next: Option<bool>,
	#[serde(rename = "display_hold")]
	pub display_hold: Option<bool>,
	pub gmargin: Option<i64>,
	pub gincrease: Option<f64>,
	pub garbagemultiplier: Option<i64>,
	pub garbagemargin: Option<i64>,
	pub garbageincrease: Option<f64>,
	pub garbagecap: Option<i64>,
	pub garbagecapincrease: Option<i64>,
	pub garbagecapmax: Option<i64>,
	pub garbageblocking: Option<GarbageBlockingType>,
	pub presets: Option<String>,
	pub bagtype: Option<BagType>,
	pub spinbonuses: Option<SpinBonusesType>,
	pub combotable: Option<String>,
	pub kickset: Option<KicksetType>,
	pub nextcount: Option<i64>,
	#[serde(rename = "allow_harddrop")]
	pub allow_harddrop: Option<bool>,
	#[serde(rename = "display_shadow")]
	pub display_shadow: Option<bool>,
	pub locktime: Option<i64>,
	pub garbagespeed: Option<i64>,
	#[serde(rename = "forfeit_time")]
	pub forfeit_time: Option<i64>,
	pub are: Option<i64>,
	#[serde(rename = "lineclear_are")]
	pub lineclear_are: Option<i64>,
	pub infinitemovement: Option<bool>,
	pub lockresets: Option<i64>,
	pub allow180: Option<bool>,
	pub objective: Option<Objective>,
	#[serde(rename = "room_handling")]
	pub room_handling: Option<bool>,
	#[serde(rename = "room_handling_arr")]
	pub room_handling_arr: Option<i64>,
	#[serde(rename = "room_handling_das")]
	pub room_handling_das: Option<i64>,
	#[serde(rename = "room_handling_sdf")]
	pub room_handling_sdf: Option<i64>,
	#[serde(rename = "manual_allowed")]
	pub manual_allowed: Option<bool>,
	pub b2bchaining: Option<bool>,
	pub clutch: Option<bool>,
	pub nolockout: Option<bool>,
	pub passthrough: Option<PassthroughType>,
	#[serde(rename = "can_undo")]
	pub can_undo: Option<bool>,
	#[serde(rename = "can_retry")]
	pub can_retry: Option<bool>,
	pub retryisclear: Option<bool>,
	pub noextrawidth: Option<bool>,
	pub stride: Option<bool>,
	pub boardwidth: Option<i64>,
	pub boardheight: Option<i64>,
	pub latencypreference: Option<String>,
	pub handling: Option<Handling>,
	pub fulloffset: Option<i64>,
	pub fullinterval: Option<i64>,
	pub username: Option<String>,
	pub boardbuffer: Option<i64>,
	pub physical: Option<bool>,
	pub minoskin: Option<Minoskin>,
	pub ghostskin: Option<String>,
	pub boardskin: Option<String>,

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Objective {
	#[serde(rename = "type")]
	pub type_field: Option<String>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Minoskin {
	pub z: String,
	pub l: String,
	pub o: String,
	pub s: String,
	pub i: String,
	pub j: String,
	pub t: String,
	pub other: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
	pub seed: Option<i64>,
	pub lines: Option<i64>,
	#[serde(rename = "level_lines")]
	pub level_lines: Option<i64>,
	#[serde(rename = "level_lines_needed")]
	pub level_lines_needed: Option<i64>,
	pub inputs: Option<i64>,
	pub holds: Option<i64>,
	pub time: Option<Time>,
	pub score: Option<i64>,
	pub zenlevel: Option<i64>,
	pub zenprogress: Option<i64>,
	pub level: Option<i64>,
	pub combo: Option<i64>,
	pub currentcombopower: Option<i64>,
	pub topcombo: Option<i64>,
	pub btb: Option<i64>,
	pub topbtb: Option<i64>,
	pub currentbtbchainpower: Option<i64>,
	pub tspins: Option<i64>,
	pub piecesplaced: Option<i64>,
	pub clears: Option<Clears>,
	pub garbage: Option<Garbage>,
	pub kills: Option<i64>,
	pub finesse: Option<Finesse>,

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Time {
	pub start: Option<i64>,
	pub zero: Option<bool>,
	pub locked: Option<bool>,
	pub prev: Option<i64>,
	pub frameoffset: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clears {
	pub singles: Option<i64>,
	pub doubles: Option<i64>,
	pub triples: Option<i64>,
	pub quads: Option<i64>,
	pub realtspins: Option<i64>,
	pub minitspins: Option<i64>,
	pub minitspinsingles: Option<i64>,
	pub tspinsingles: Option<i64>,
	pub minitspindoubles: Option<i64>,
	pub tspindoubles: Option<i64>,
	pub tspintriples: Option<i64>,
	pub tspinquads: Option<i64>,
	pub allclear: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Garbage {
	pub sent: Option<i64>,
	pub received: Option<i64>,
	pub attack: Option<i64>,
	pub cleared: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Finesse {
	pub combo: Option<i64>,
	pub faults: Option<i64>,
	pub perfectpieces: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
	pub board: Option<Vec<Vec<Value>>>,
	pub bag: Option<Vec<String>>,
	pub hold: Option<Hold>,
	pub g: Option<f64>,
	pub handling: Option<Handling>,
	pub playing: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hold {
	pub piece: Option<Value>,
	pub locked: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Killer {
	pub name: Option<Value>,
	#[serde(rename = "type")]
	pub type_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aggregatestats {
	pub apm: Option<f64>,
	pub pps: Option<f64>,
	pub vsscore: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Export {
	pub successful: Option<bool>,
	pub gameoverreason: Option<String>,
	pub options: Option<Options>,
	pub stats: Option<Stats>,
	pub targets: Option<Vec<String>>,
	pub fire: Option<i64>,
	pub game: Option<Game>,
	pub killer: Option<Killer>,
	pub aggregatestats: Option<Aggregatestats>,

}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Handling {
	pub arr: Option<f64>,
	pub das: Option<f64>,
	pub dcd: Option<f64>,
	pub sdf: Option<f64>,
	pub safelock: Option<bool>,
	pub cancel: Option<bool>,
}
