#![allow(dead_code, unused_variables, deprecated, unused_imports)] //TODO: cleanup
use config::Config;
use reqwest::{blocking::Client, Error};
use serde_json;
use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::num::ParseIntError;
use std::vec;
use crate::candidates::{CandidateData, Candidate};
use crate::applications::{ApplicationData, AttachmentDownload, Applications};
use crate::jobs::JobData;
use crate::job_stages::JobStageData;

mod applications;
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

    let job_stage_number = select_job_stage(api_key, job_number);
    clear_screen();

    get_applications(api_key, job_number, job_stage_number);

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

    //status_ok(&response);

    response
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

    println!("{} Jobs Found", job_map.keys().len());
    println!();

    loop {
        println!("Select a job by number to load job stages:");

        for (key, value) in &job_map {
            println!("{}: {} {}", key, value.name, value.id);
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

fn select_job_stage(api_key: &str, job_id: i64) -> i64 {
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

    println!("{} Job Stages Found", job_stages_map.keys().len());
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

fn get_applications(api_key: &str, job_id: i64, job_stage_id: i64) {
    let response = call_api(
        api_key,
        &format!(
            "https://harvest.greenhouse.io/v1/applications?job_id={}",
            job_id
        ),
    );

    let applications: Applications = serde_json::from_str(response.text().unwrap().as_str()).unwrap();
    let applications_iter = applications.iter();

    //todo figure out how to use a filter for this
    let mut applications_map = HashMap::new();
    let mut i: i32 = 1;
    for val in applications_iter {
        let stage_id = val.current_stage.as_ref().unwrap().id;
        if stage_id == job_stage_id {
            
            let candidate = get_candidate(api_key, val.candidate_id);

            //todo: handle current_stage missing better than unwrap
            applications_map.insert(i, ApplicationData::new(val.id, &val.attachments, stage_id, val.candidate_id, candidate));
            i = i + 1;
        }
    }

    //pull the attachments of type=resume/cover_letter
    //let attachments_to_download = HashMap::new();

    for (key, value) in &applications_map {
       // println!("{} / {:?}", key, value.attachments);
        
        let app_attachment_iter = value.attachments.iter();
        for a in app_attachment_iter {
            if a.attachment_type == "cover_letter" || a.attachment_type == "resume" {
                println!("{}", a.url);
            }
        }
    }

    // let app_map_iter = applications_map.iter();
    // for val in app_map_iter {
    //     println!("{:?}",val);
    //     println!();


    //     // let app_attachment_iter = val.attachments.iter();
    //     // for attachment in app_attachment_iter {
    //     //     if attachment.attachment_type == "cover_letter" || attachment.attachment_type == "resume" {
    //     //         attachments_to_download.insert(val.candidate.last_name, attachment.url);
    //     //     }
    //     // }
    // }


   //println!("{:?}", attachments_to_download);



    println!(
        "{} Applications Found For Stage {}",
        applications_map.keys().len(),
        job_stage_id
    );
    println!();
}


fn get_candidate(api_key: &str, candidate_id: i64) -> CandidateData {
    let response = call_api(
        api_key,
        &format!(
            "https://harvest.greenhouse.io/v1/candidates/{}",
            candidate_id
        ),
    );

    let candidate: Candidate = serde_json::from_str(response.text().unwrap().as_str()).unwrap();
    let candidate_data = CandidateData::new(candidate.id, candidate.first_name, candidate.last_name);
    
    let first_name = &candidate_data.first_name;
    let last_name = &candidate_data.last_name;
    println!("Loading candidate data for - {}, {}", first_name, last_name);

    candidate_data
}

fn clear_screen() {
    clearscreen::clear().expect("Failed to clear screen");
}
