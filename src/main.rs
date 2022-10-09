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
    clear_screen();

    println!("==============================================================================================");
    println!("==============================================================================================");
    println!("================== GREENHOUSE.IO RESUME AND COVER LETTER EXPORT TOOL =========================");
    println!("==============================================================================================");
    println!("==============================================================================================");
    println!();
    println!();
    println!();

    let settings = Config::builder()
        .add_source(config::File::with_name("settings/Settings"))
        .build();

    match &settings {
        Ok(cfg) => println!("A valid Settings.toml config was found, loading settings..."),
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
            println!("api-key could not be found in settings file");
            ""
        }
    };

    let api_root = match setting.get("api-root") {
        Some(key) => key,
        None => {
            println!("api-root could not be found in settings file");
            ""
        }
    };
    
    println!();
    println!();
    println!();
    println!();
    println!();
    println!("Press [ENTER] key to load jobs from Greenhouse.io API ({})", api_root);
    println!("Press [Ctrl+C] key to exit application at any time");

    let mut x = String::with_capacity(5);
    io::stdin().read_line(&mut x).expect("Error reading input");

    loop{
        clear_screen();

        let job_number = select_job(api_root, api_key);
        //println!("You entered: {}", job_number);
        clear_screen();

        let job_stage_number = select_job_stage(api_root, api_key, job_number);
        clear_screen();

        let applications_found = get_applications(api_root, api_key, job_number, job_stage_number);

        if applications_found > 0 {
            let input = get_input().to_uppercase();
            let continue_to_download = input.trim();

            if continue_to_download == "Y"{
                
            }

            clear_screen();
        }
        else
        {
            println!("No applications found in this stage.");
        }

        println!("Enter [Y] to continuing searching jobs, [N] to exit.");

        let input = get_input().to_uppercase();
        let run_again = input.trim();

        if run_again == "Y"{
            continue;
        }
        else
        {
            break;
        }
    }

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

fn get_input<'a>() -> String {
    let mut x = String::with_capacity(5);
    io::stdin().read_line(&mut x).expect("Error reading input");
    x
}

fn call_api(api_root: &str, api_key: &str, url: &str) -> reqwest::blocking::Response {
    let client = Client::new();
    let user_name = api_key.to_string();
    let password: Option<String> = None;
    let combined_url = &format!("{}{}", api_root, url);
    let response = client
        .get(combined_url)
        .basic_auth(user_name, password)
        .send()
        .unwrap(); //TODO: handle error case

    //status_ok(&response);

    response
}

fn select_job(api_root: &str, api_key: &str) -> i64 {
    let response = call_api(api_root, api_key, "/jobs?status=open");

    let jobs: jobs::Jobs = serde_json::from_str(response.text().unwrap().as_str()).unwrap();

    let mut job_map = HashMap::new();
    let mut i: i32 = 1;
    for val in &jobs {
        job_map.insert(i, JobData::new(val.id, &val.name));
        i = i + 1;
    }

    println!("{} Jobs Found With Open Status", job_map.keys().len());
    println!();

    loop {
        println!("Select a job by number to load job stages:");
        println!();

        for (key, value) in &job_map {
            println!("{}: {}", key, value.name);
        }

        let entered_number = match get_number_input() {
            Ok(num) => num,
            Err(_) => {
                clear_screen();
                println!("That's not a valid job number");
                continue;
            }
        };

        match job_map.get(&entered_number) {
            Some(job) => return job.id,
            None => {
                clear_screen();
                println!("That's not a valid job number");
                continue;
            }
        }
    }
}

fn select_job_stage(api_root: &str, api_key: &str, job_id: i64) -> i64 {
    let response = call_api(
        api_root,
        api_key,
        &format!("/jobs/{}/stages", job_id),
    );

    let job_stages: job_stages::JobStage =
        serde_json::from_str(response.text().unwrap().as_str()).unwrap();

    let mut job_stages_map = HashMap::new();
    let mut i: i32 = 1;
    for val in &job_stages {
        job_stages_map.insert(i, JobStageData::new(val.id, &val.name));
        i = i + 1;
    }

    println!("{} Job Stages Found", job_stages_map.keys().len());
    println!();
    loop {
        println!("Select a job stage by number to download resumes and cover letters:");
        println!();

        for (key, value) in &job_stages_map {
            println!("{}: {}", key, value.name);
        }

        let entered_number = match get_number_input() {
            Ok(num) => num,
            Err(_) => {
                clear_screen();
                println!("That's not a valid job stage number");
                continue;
            }
        };

        match job_stages_map.get(&entered_number) {
            Some(job_stage) => return job_stage.id,
            None => {
                clear_screen();
                println!("That's not a valid job stage number");
                continue;
            }
        }
    }
}

fn get_applications(api_root: &str, api_key: &str, job_id: i64, job_stage_id: i64) -> i32 {
    let response = call_api(
        api_root,
        api_key,
        &format!(
            "/applications?job_id={}",
            job_id
        ),
    );

    let applications: Applications = serde_json::from_str(response.text().unwrap().as_str()).unwrap();

    //todo figure out how to use a filter for this
    let mut applications_map = HashMap::new();
    let mut i: i32 = 0;
    for val in &applications {
        let stage_id = val.current_stage.as_ref().unwrap().id;
        if stage_id == job_stage_id {    
            if i == 0 {
                i = 1; //TODO: shameful, fix this
            }
            
            let candidate = get_candidate(api_root, api_key, val.candidate_id);

            //todo: handle current_stage missing better than unwrap
            applications_map.insert(i, ApplicationData::new(val.id, &val.attachments, stage_id, val.candidate_id, candidate));
            i = i + 1;
        }
    }

    clear_screen();

    //pull the attachments of type=resume/cover_letter

    for (key, value) in &applications_map {
       // println!("{} / {:?}", key, value.attachments);
                
        let mut cover_letter_found : bool = false;
        let mut resume_found : bool = false;
        let mut attachment_download_message : &str = "";
        let attachment_iter = value.attachments.iter();

        for a in attachment_iter {
            if a.attachment_type == "cover_letter" {
                cover_letter_found = true;
            }
            if a.attachment_type == "resume" {
                resume_found = true;
            }
        }

        if cover_letter_found && resume_found 
            {
                attachment_download_message = "Cover letter and resume found for";
            }
            else if cover_letter_found {
                attachment_download_message = "Cover letter only found for";
            }
            else if resume_found {
                attachment_download_message = "Resume only found for";
            }

            println!("{} - {}, {}", attachment_download_message, value.candidate.last_name, value.candidate.first_name);
    }

    if i > 0 {
        println!();
        println!("Do you want to download cover letter/resume data for all users above? [Y]/[N]");
    }

    i
}

fn get_candidate(api_root: &str, api_key: &str, candidate_id: i64) -> CandidateData {
    let response = call_api(
        api_root,
        api_key,
        &format!(
            "/candidates/{}",
            candidate_id
        ),
    );

    let candidate: Candidate = serde_json::from_str(response.text().unwrap().as_str()).unwrap();
    let candidate_data = CandidateData::new(candidate.id, candidate.first_name, candidate.last_name);
    
    let first_name = &candidate_data.first_name;
    let last_name = &candidate_data.last_name;
    println!("Loading candidate data for - {}, {}...", last_name, first_name);
    
    candidate_data
}

fn clear_screen() {
    clearscreen::clear().expect("Failed to clear screen");
}