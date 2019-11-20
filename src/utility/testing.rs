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
const TEST_DATA_DIR: &'static str = "test_data/small";

pub fn get_input_output_file(question_name: &str) -> Result<(String, String), Error> {
    let input_file = path_abs::PathFile::new(format!("{}/{}.txt", TEST_DATA_DIR, question_name))?;
    let output_file =
        path_abs::PathFile::new(format!("{}/{}_output.txt", TEST_DATA_DIR, question_name))?;
    Ok((input_file.to_string(), output_file.to_string()))
}

pub fn get_input_file(question_name: &str) -> Result<String, Error> {
    let input_file = path_abs::PathFile::new(format!("{}/{}.txt", TEST_DATA_DIR, question_name))?;
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
    for filename in glob(&format!("{}/*.rs", folder_name))? {
        if let Ok(filename) = filename {
            let stem = filename.as_path().file_stem();
            if let Some(stem) = stem {
                let stem = stem.to_str();
                if let Some(stem) = stem {
                    if &stem[0..1] == "r" && stem.split("_").count() == 2 {
                        let question_name = stem.split('_').nth(1);
                        if let Some(question_name) = question_name {
                            question_names.push(question_name.to_owned());
                        }
                    }
                }
            }
        }
    }
    Ok(question_names)
}

pub fn get_all_sample_data(folder_name: &str) -> Result<(), Error> {
    let src_folder_names = vec![
        "src/algorithmic_heights",
        "src/stronghold",
        "src/textbook_track",
    ];
    let mut question_names = Vec::new();
    for dir in src_folder_names {
        question_names.extend_from_slice(&get_question_names_from_folder(dir)?);
    }
    for question_name in question_names {
        let path =
            path_abs::PathAbs::new(&format!("{}/rosalind_{}.txt", folder_name, question_name))?;
        if !path.is_file() {
            println!("Retrieving data for {}", question_name);
            get_sample_data_for_question(&question_name, folder_name)?;
            thread::sleep(Duration::from_secs(2));
        }
    }
    Ok(())
}
