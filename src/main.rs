use config::Config;
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io;
use std::io::copy;
use std::io::Cursor;
use std::num::ParseIntError;
use std::path::Path;

use crate::applications::{ApplicationData, Applications};
use crate::candidates::{Candidate, CandidateData};
use crate::job_stages::JobStageData;
use crate::jobs::JobData;

mod applications;
mod candidates;
mod job_stages;
mod jobs;

//
// TODO
// ordering list
// duplicates
// file strucute - Job Name - Job Stage / all files under that
//

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Config::builder()
        .add_source(config::File::with_name("settings/Settings"))
        .build();

    match &settings {
        Ok(_) => println!("A valid Settings.toml config was found, loading settings..."),
        Err(e) => panic!("Settings.toml could not be found. Please place the Settings.toml file in the same directory as the executable. Error - {}", e),
    }

    let setting = match settings
        .unwrap()
        .try_deserialize::<HashMap<String, String>>()
    {
        Ok(json) => json,
        Err(e) => {
            panic!(
                "Settings.toml file not did not contain a valid API key. Error - {}",
                e
            )
        }
    };

    let api_key = match setting.get("api-key") {
        Some(key) => key,
        None => {
            panic!("api-key could not be found in settings file")
        }
    };

    let api_root = match setting.get("api-root") {
        Some(key) => key,
        None => {
            panic!("api-root could not be found in settings file")
        }
    };

    let output_folder = match setting.get("output-folder") {
        Some(key) => key,
        None => {
            panic!("output-folder could not be found in settings file")
        }
    };

    clear_screen();

    println!("==============================================================================================");
    println!("==============================================================================================");
    println!("================== GREENHOUSE.IO RESUME AND COVER LETTER EXPORT TOOL =========================");
    println!("==============================================================================================");
    println!("==============================================================================================");
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!(
        "Press [ENTER] key to load jobs from Greenhouse.io API ({})",
        api_root
    );
    println!("Press [Ctrl+C] key to exit application at any time");

    let mut x = String::with_capacity(5);
    io::stdin().read_line(&mut x).expect("Error reading input");

    loop {
        clear_screen();

        let job_data = select_job(api_root, api_key);
        clear_screen();

        let job_stage_data = select_job_stage(api_root, api_key, job_data.id);
        clear_screen();

        create_download_folder(output_folder, job_data.name, job_stage_data.name);

        get_applications(
            api_root,
            api_key,
            job_data.id,
            job_stage_data.id,
            output_folder,
        );

        println!("Enter [Y] to continuing searching jobs, [N] to exit.");

        let input = get_input().to_uppercase();
        let run_again = input.trim();

        if run_again == "Y" {
            continue;
        } else {
            break;
        }
    }

    Ok(())
}

fn get_number_input() -> Result<i32, ParseIntError> {
    let mut x = String::with_capacity(5);
    io::stdin().read_line(&mut x).expect("Error reading input");
    x.trim().parse()
}

fn get_input() -> String {
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
        .expect("API call failed");

    status_ok(&response, url);
    response
}

fn call_api_external(url: &str) -> reqwest::blocking::Response {
    let client = Client::new();
    let response = client.get(url).send().unwrap();

    status_ok(&response, url);
    response
}

fn status_ok(res: &reqwest::blocking::Response, url: &str) {
    if !res.status().is_success() {
        panic!("Bad status - {} while calling - {}", res.status(), url);
    }
}

fn select_job(api_root: &str, api_key: &str) -> JobData {
    let response = call_api(api_root, api_key, "/jobs?status=open");

    let jobs: jobs::Jobs = serde_json::from_str(response.text().unwrap().as_str()).unwrap();

    let mut job_map = HashMap::new();
    let mut i: i32 = 1;
    for val in &jobs {
        job_map.insert(i, JobData::new(val.id, val.name.to_string()));
        i += 1;
    }

    println!("{} Jobs Found With Open Status", job_map.keys().len());
    println!();

    let job_id : i64;
    let job_name : String;

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
            Some(job) => {
                return JobData::new(job.id, job.name.to_string())
            }
            None => {
                clear_screen();
                println!("That's not a valid job number");
                continue;
            }
        };
    }
}


fn select_job_stage(api_root: &str, api_key: &str, job_id: i64) -> JobStageData {
    let response = call_api(api_root, api_key, &format!("/jobs/{}/stages", job_id));

    let job_stages: job_stages::JobStage =
        serde_json::from_str(response.text().unwrap().as_str()).unwrap();

    let mut job_stages_map = HashMap::new();
    let mut i: i32 = 1;
    for val in &job_stages {
        job_stages_map.insert(i, JobStageData::new(val.id, val.name.to_string()));
        i += 1;
    }

    println!("{} Job Stages Found", job_stages_map.keys().len());
    println!();

    let job_stage_data : JobStageData;

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
            Some(job_stage) => {
                return JobStageData::new(job_stage.id, job_stage.name.to_string())
            },
            None => {
                clear_screen();
                println!("That's not a valid job stage number");
                continue;
            }
        }
    }
   
}

fn get_applications(
    api_root: &str,
    api_key: &str,
    job_id: i64,
    job_stage_id: i64,
    output_folder: &str,
) {
    let response = call_api(
        api_root,
        api_key,
        &format!("/applications?job_id={}", job_id),
    );

    let applications: Applications =
        serde_json::from_str(response.text().unwrap().as_str()).unwrap();

    let mut any_applications_found = false;
    //todo figure out how to use a filter for this
    let mut applications_map = HashMap::new();
    let mut i: i32 = 1;
    for val in &applications {
        if val.status == "active" {
        let stage_id = val.current_stage.as_ref().unwrap().id;
        if stage_id == job_stage_id {
            any_applications_found = true;
            let candidate = get_candidate(api_root, api_key, val.candidate_id);

            //todo: handle current_stage missing better than unwrap
            applications_map.insert(
                i,
                ApplicationData::new(
                    val.id,
                    &val.attachments,
                    stage_id,
                    val.candidate_id,
                    candidate,
                ),
            );
            i += 1;
        }
    }
    }

    clear_screen();

    for value in applications_map.values() {
        let mut cover_letter_found: bool = false;
        let mut resume_found: bool = false;
        let mut attachment_download_message: &str = "";
        let attachment_iter = value.attachments.iter();

        for a in attachment_iter {
            if a.attachment_type == "cover_letter" {
                cover_letter_found = true;
            }
            if a.attachment_type == "resume" {
                resume_found = true;
            }
        }

        if cover_letter_found && resume_found {
            attachment_download_message = "Cover letter and resume found for";
        } else if cover_letter_found {
            attachment_download_message = "Cover letter only found for";
        } else if resume_found {
            attachment_download_message = "Resume only found for";
        }

        println!(
            "{} - {}, {}",
            attachment_download_message, value.candidate.last_name, value.candidate.first_name
        );
    }

    if any_applications_found {
        println!();
        println!("Do you want to download cover letter/resume data for all users above? [Y]/[N]");

        let input = get_input().to_uppercase();
        let continue_to_download = input.trim();

        if continue_to_download == "Y" {
            println!("Downloading...");

            for value in applications_map.values() {

                for attachment in value.attachments {
                    download_attachments(
                        &attachment.url,
                        &value.candidate.last_name,
                        &attachment.attachment_type,
                        &output_folder
                    );
                }
            }
        }
    } else {
        println!("No applications found in this stage.");
        println!();
    }
}

fn get_candidate(api_root: &str, api_key: &str, candidate_id: i64) -> CandidateData {
    let response = call_api(api_root, api_key, &format!("/candidates/{}", candidate_id));

    let candidate: Candidate = serde_json::from_str(response.text().unwrap().as_str()).unwrap();
    let candidate_data =
        CandidateData::new(candidate.id, candidate.first_name, candidate.last_name);

    let first_name = &candidate_data.first_name;
    let last_name = &candidate_data.last_name;
    println!(
        "Loading candidate data for - {}, {}...",
        last_name, first_name
    );

    candidate_data
}

fn clear_screen() {
    clearscreen::clear().expect("Failed to clear screen");
}

fn create_download_folder(output_folder : &str, job_name : String, job_stage_name : String) {
    let dir = format!("{}{}-{}", output_folder, job_name, job_stage_name);
    fs::create_dir_all(&dir).expect("Could not create directory");
}

fn download_attachments(url: &str, candidate_last_name: &str, attachment_type: &str, dir: &str) {
    if attachment_type == "resume" || attachment_type == "cover_letter" {
        let response = call_api_external(url);

        let dest = {
            let filename = response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp.pdf");

            let extension = get_extension_from_filename(filename);
            let fname = format!(
                "{} {}.{}",
                candidate_last_name,
                format_attachment_type(attachment_type),
                extension.expect("Could not format extension")
            );
            let fname = format!("{}/{}", dir, fname);

            println!("Writing file - {:?}", fname);
            File::create(fname)
        };

        let mut content = Cursor::new(response.bytes().expect("Could not parse response bytes"));
        copy(
            &mut content,
            &mut dest.expect("Destination could not be found"),
        )
        .expect("Could not copy data to file");
    }
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

fn format_attachment_type(attachment_type: &str) -> &str {
    if attachment_type == "cover_letter" {
        "cover letter"
    } else {
        attachment_type
    }
}
