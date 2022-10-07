#![allow(dead_code, unused_variables, deprecated, unused_imports)] //TODO: cleanup
use config::Config;
use reqwest::blocking::Client;
use reqwest::Error;
use serde_json;
use std::collections::HashMap;
use std::io;
use std::io::Read;
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
    //   let x = x.trim().parse().expect("Error parsing number");

    clearscreen::clear().expect("failed to clear screen");

    let client = Client::new();

    let user_name = api_key.to_string();
    let password: Option<String> = None;

    let response = client
        .get("https://harvest.greenhouse.io/v1/jobs?status=open")
        .basic_auth(user_name, password)
        .send()?;

    status_ok(&response);

    //  let json = response.json::<HashMap<String, String>>()?;
    //  eprintln!("{:?}", json);

    // &response.unwrap().copy_to(&mut std::io::stdout())?;

    //  let json: jobs::Jobs = response.json()?;
    //  let text = response.text();
    //  eprintln!("{:?}", text);

    let jobs: jobs::Jobs = serde_json::from_str(response.text().unwrap().as_str()).unwrap();
    let job_iter = jobs.iter();

    let mut job_map = HashMap::new();
    let mut i: i32 = 1;
    for val in job_iter {
        job_map.insert(i, &val.name);
        i = i + 1;
    }

    println!("{} Jobs Found from API", job_map.keys().len());
    println!("Select a job by number to load job stages");

    for (key, value) in &job_map {
        println!("{}: {}", key, value);
    }

    let mut x = String::with_capacity(5);
    io::stdin().read_line(&mut x).expect("Error reading input");
    let x: i32 = x.trim().parse().expect("Error parsing number");

    println!("You entered: {}", x);

    Ok(())
}

fn status_ok(res: &reqwest::blocking::Response) {
    if res.status().is_success() {
        println!("Successfully called API with status - {}", res.status());
    } else {
        println!("Bad status - {}", res.status());
    }
}
