use anyhow::Ok;
use csv::Reader;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    pub name: String,
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Player = result?;
        //println!("{:?}", record);
        ret.push(record);
    }
    let json_str = serde_json::to_string_pretty(&ret)?;
    std::fs::write(output, json_str)?;
    Ok(())
}
