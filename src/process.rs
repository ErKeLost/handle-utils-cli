use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "PascalCase")]
pub struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    // let records = reader.deserialize().map(|record| record.unwrap()).collect::<Vec<Player>>();
    for result in reader.deserialize() {
        let record: Player = result?;
        ret.push(record);
    }

    let json = serde_json::to_string_pretty(&ret)?;

    fs::write(output, json)?;

    Ok(())
}
