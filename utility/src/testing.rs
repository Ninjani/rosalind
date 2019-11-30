use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use failure::Error;
use glob::glob;
use path_abs;
use reqwest;
use scraper;

pub const ROSALIND_FLOAT_ERROR_F32: f32 = 0.001;
pub const ROSALIND_FLOAT_ERROR_F64: f64 = 0.001;

const ROSALIND_URL: &'static str = "http://rosalind.info/problems";

pub fn get_input_output_file(question_name: &str) -> Result<(String, String), Error> {
    let test_data_dir = get_test_data_dir();
    let input_file = path_abs::PathFile::new(format!("{}/{}.txt", test_data_dir, question_name))?;
    let output_file =
        path_abs::PathFile::new(format!("{}/{}_output.txt", test_data_dir, question_name))?;
    Ok((input_file.to_string(), output_file.to_string()))
}

pub fn get_input_file(question_name: &str) -> Result<String, Error> {
    let test_data_dir = get_test_data_dir();
    let input_file = path_abs::PathFile::new(format!("{}/{}.txt", test_data_dir, question_name))?;
    Ok(input_file.to_string())
}

pub fn get_sample_data_for_question(question_name: &str, folder_name: &str) -> Result<(), Error> {
    let body = reqwest::get(&format!("{}/{}", ROSALIND_URL, question_name))?.text()?;
    let html = scraper::Html::parse_document(&body);
    let selector = scraper::Selector::parse("div.codehilite").unwrap();
    let input_file =
        path_abs::PathFile::create(format!("{}/rosalind_{}.txt", folder_name, question_name))?;
    let output_file = path_abs::PathFile::create(format!(
        "{}/rosalind_{}_output.txt",
        folder_name, question_name
    ))?;
    let mut fragments = html.select(&selector);
    input_file.write_str(
        &fragments
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join(""),
    )?;
    output_file.write_str(
        &fragments
            .next()
            .unwrap()
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

fn get_top_dir() -> PathBuf {
    let bin = env::current_exe().expect("exe path");
    let mut target_dir = PathBuf::from(bin.parent().expect("bin parent"));
    while target_dir.file_name() != Some(OsStr::new("target")) {
        target_dir.pop();
    }
    target_dir.parent().expect("target parent").to_owned()
}

fn get_test_data_dir() -> String {
    format!("{}/test_data/small", get_top_dir().display())
}

pub fn get_all_sample_data() -> Result<(), Error> {
    let test_data_dir = get_test_data_dir();
    let question_names = get_question_names_from_folder(".")?;
    for question_name in question_names {
        let path =
            path_abs::PathAbs::new(&format!("{}/rosalind_{}.txt", test_data_dir, question_name))?;
        if !path.is_file() {
            println!("Retrieving data for {}", question_name);
            get_sample_data_for_question(&question_name, &test_data_dir)?;
            thread::sleep(Duration::from_secs(2));
        }
    }
    Ok(())
}
