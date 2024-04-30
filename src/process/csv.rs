use anyhow::Ok;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::options::OutputFormat;
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub struct Player {
    pub name: String,
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub kit: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    // for result in reader.deserialize() {
    //     let record: Player = result?;
    //     ret.push(record);
    // }
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    std::fs::write(output, content)?;
    Ok(())
}
