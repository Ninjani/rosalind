use std::env;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use crate::errors::RosalindParseError;
use anyhow::Error;
use glob::glob;
use reqwest;
use scraper;

pub const ROSALIND_FLOAT_ERROR_F32: f32 = 0.001;
pub const ROSALIND_FLOAT_ERROR_F64: f64 = 0.001;

const ROSALIND_URL: &str = "https://rosalind.info/problems";

pub fn get_input_output_file(question_name: &str) -> Result<(PathBuf, PathBuf), Error> {
    let sample_data_dir = get_sample_data_dir();
    let input_file = sample_data_dir.join(format!("{}.txt", question_name));
    let output_file = sample_data_dir.join(format!("{}_output.txt", question_name));
    Ok((input_file, output_file))
}

pub fn get_input_file(question_name: &str) -> Result<PathBuf, Error> {
    let input_file = get_sample_data_dir().join(format!("{}.txt", question_name));
    Ok(input_file)
}

pub async fn get_sample_data_for_question(question_name: &str, folder: &Path) -> Result<(), Error> {
    let body = reqwest::get(&format!("{}/{}", ROSALIND_URL, question_name))
        .await?
        .text()
        .await?;
    let html = scraper::Html::parse_document(&body);
    let selector = scraper::Selector::parse("div.codehilite").unwrap();
    let input_file = folder.join(format!("rosalind_{}.txt", question_name));
    let output_file = folder.join(format!("rosalind_{}_output.txt", question_name));
    let mut fragments = html.select(&selector);
    std::fs::write(
        input_file,
        &fragments
            .next()
            .ok_or_else(|| RosalindParseError::SampleDataError(question_name.to_owned()))?
            .text()
            .collect::<Vec<_>>()
            .join(""),
    )?;
    std::fs::write(
        output_file,
        &fragments
            .next()
            .ok_or_else(|| RosalindParseError::SampleDataError(question_name.to_owned()))?
            .text()
            .collect::<Vec<_>>()
            .join(""),
    )?;
    Ok(())
}

fn get_question_names_from_folder(folder_name: &str) -> Result<Vec<String>, Error> {
    let mut question_names = Vec::new();
    for folder in glob(&format!("{}/*", folder_name))? {
        let folder = folder?;
        let stem = folder.as_path().file_stem().unwrap().to_str().unwrap();
        if stem.chars().nth(1).unwrap() == '_' {
            let question_name = stem.split('_').nth(1);
            if let Some(question_name) = question_name {
                question_names.push(question_name.to_owned());
            }
        }
    }
    Ok(question_names)
}

fn get_sample_data_dir() -> PathBuf {
    [env!("CARGO_WORKSPACE_DIR"), "sample_data"]
        .iter()
        .collect()
}

pub async fn get_all_sample_data() -> Result<(), Error> {
    let sample_data_dir = get_sample_data_dir();
    if !sample_data_dir.exists() {
        std::fs::create_dir(&sample_data_dir)?;
    }
    let question_names = get_question_names_from_folder(".")?;
    for question_name in question_names {
        let path = sample_data_dir.join(&format!("rosalind_{}.txt", question_name));
        if !path.is_file() {
            println!("Retrieving data for {}", question_name);
            get_sample_data_for_question(&question_name, &sample_data_dir).await?;
            thread::sleep(Duration::from_secs(2));
        }
    }
    Ok(())
}
