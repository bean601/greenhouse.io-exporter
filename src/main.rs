#![allow(dead_code, unused_variables, deprecated, unused_imports)] //TODO: cleanup
use config::Config;
use reqwest::{blocking::Client, Error};
use serde_json;
use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::num::ParseIntError;
use std::vec;

mod candidates;
mod job_stages;
mod jobs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Config::builder()
        .add_source(config::File::with_name("settings/Settings"))
        .build();

    match &settings {
        Ok(cfg) => println!("A config was found"),
        Err(e) => println!("Settings.toml could not be found. Error - {}", e),
    }

    let setting = match settings.unwrap().deserialize::<HashMap<String, String>>() {
        Ok(json) => json,
        Err(e) => {
            println!(
                "Settings.toml file not did not contain a valid API key. Error - {}",
                e
            );
            HashMap::new()
        }
    };

    let api_key = match setting.get("api-key") {
        Some(key) => key,
        None => {
            println!("API could not be found in settings file");
            ""
        }
    };

    println!("Press any key to load jobs from Greenhouse.io");

    let mut x = String::with_capacity(5);
    io::stdin().read_line(&mut x).expect("Error reading input");

    clear_screen();

    let job_number = select_job(api_key);
    //println!("You entered: {}", job_number);
    clear_screen();

    let job_status_number = select_job_stage(api_key, job_number);
    clear_screen();

    Ok(())
}

fn status_ok(res: &reqwest::blocking::Response) {
    if res.status().is_success() {
        println!("Successfully called API with status - {}", res.status());
    } else {
        println!("Bad status - {}", res.status());
    }
}

fn get_number_input() -> Result<i32, ParseIntError> {
    let mut x = String::with_capacity(5);
    io::stdin().read_line(&mut x).expect("Error reading input");
    x.trim().parse()
}

fn call_api(api_key: &str, url: &str) -> reqwest::blocking::Response {
    let client = Client::new();
    let user_name = api_key.to_string();
    let password: Option<String> = None;
    let response = client
        .get(url)
        .basic_auth(user_name, password)
        .send()
        .unwrap(); //TODO: handle error case

    status_ok(&response);

    response
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct JobData<'a> {
    pub id: i64,
    pub name: &'a str,
}

impl JobData<'_> {
    fn new(id: i64, name: &str) -> JobData {
        JobData { id: id, name: name }
    }
}

fn select_job(api_key: &str) -> i64 {
    let response = call_api(api_key, "https://harvest.greenhouse.io/v1/jobs?status=open");

    let jobs: jobs::Jobs = serde_json::from_str(response.text().unwrap().as_str()).unwrap();
    let job_iter = jobs.iter();

    let mut job_map = HashMap::new();
    let mut i: i32 = 1;
    for val in job_iter {
        job_map.insert(i, JobData::new(val.id, &val.name));
        i = i + 1;
    }

    println!("{} Jobs Found from API", job_map.keys().len());
    println!();

    loop {
        println!("Select a job by number to load job stages:");

        for (key, value) in &job_map {
            println!("{}: {}", key, value.name);
        }

        let entered_number = match get_number_input() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a valid job number");
                clear_screen();
                continue;
            }
        };

        match job_map.get(&entered_number) {
            Some(job) => return job.id,
            None => {
                println!("That's not a valid job number");
                clear_screen();
                continue;
            }
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct JobStageData<'a> {
    pub id: i64,
    pub name: &'a str,
}

impl JobStageData<'_> {
    fn new(id: i64, name: &str) -> JobStageData {
        JobStageData { id: id, name: name }
    }
}

fn select_job_stage(api_key: &str, job_id: i64) -> i64 {
    println!("{}", job_id);
    let response = call_api(
        api_key,
        &format!("https://harvest.greenhouse.io/v1/jobs/{}/stages", job_id),
    );

    let job_stages: job_stages::JobStage =
        serde_json::from_str(response.text().unwrap().as_str()).unwrap();
    let job_stages_iter = job_stages.iter();

    let mut job_stages_map = HashMap::new();
    let mut i: i32 = 1;
    for val in job_stages_iter {
        job_stages_map.insert(i, JobStageData::new(val.id, &val.name));
        i = i + 1;
    }

    println!("{} Job Stages Found from API", job_stages_map.keys().len());
    println!();
    loop {
        println!("Select a job stage by number to download resumes and cover letters:");

        for (key, value) in &job_stages_map {
            println!("{}: {}", key, value.name);
        }

        let entered_number = match get_number_input() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a valid job stage number");
                clear_screen();
                continue;
            }
        };

        match job_stages_map.get(&entered_number) {
            Some(job_stage) => return job_stage.id,
            None => {
                println!("That's not a valid job stage number");
                clear_screen();
                continue;
            }
        }
    }
}

fn clear_screen(){
    clearscreen::clear().expect("Failed to clear screen");
}