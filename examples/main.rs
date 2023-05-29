use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use serde;
use timed_fields::add_timed_fields;

#[add_timed_fields]
#[derive(Debug, Default, Deserialize, Serialize)]
struct Record {
    value: String,
}

#[add_timed_fields(no_updated_at)]
#[derive(Debug, Default, Deserialize, Serialize)]
struct NoUpdate {
    value: String,
}

fn main() {
    let rec = Record::default();
    println!("{:?}", rec);

    let rec = NoUpdate::default();
    println!("{:?}", rec);
}
