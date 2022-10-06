#![allow(dead_code)]
use config::Config;
use serde_json;
use std::collections::HashMap;

mod candidates;
mod jobs;

fn main() {

    let json = r#"[
        {
          "id": 6404,
          "name": "Archaeologist",
          "requisition_id": "abc123",
          "notes": "<p>Resistance to electro-magnetic radiation a plus!</p>",
          "confidential": false,
          "status": "closed",
          "created_at": "2013-12-10T14:42:58Z",
          "opened_at": "2013-12-11T14:42:58Z",
          "closed_at": "2013-12-12T14:42:58Z",
          "updated_at": "2013-12-12T14:42:58Z",
          "is_template": false,
          "copied_from_id": 2345,
          "departments": [
            {
              "id": 25907,
              "name": "Second-Level department",
              "parent_id": 25908,
              "child_ids": [14510],
              "external_id": "12345"
            }
          ],
          "offices": [
            {
              "id": 47012,
              "name": "New York",
              "location": {
                "name": "New York, United States"
              },
              "primary_contact_user_id": 150893,
              "parent_id": 50849,
              "child_ids": [50852, 50891],
              "external_id": "15679"
            }
          ],
          "custom_fields": {
            "employment_type": "Full-Time",
            "maximum_budget": "$81.5k",
            "salary_range": {
              "min_value": 70000,
              "max_value": 90000,
              "unit": "USD"
            }
          },
          "keyed_custom_fields": {
            "employment_type": {
              "name": "Time type",
              "type": "single_select",
              "value": "Full-Time"
            },
            "budget": {
              "name": "Maximum Budget",
              "type": "short_text",
              "value": "Full-Time"
            },
            "salary_range": {
              "name": "Salary Range",
              "type": "currency_range",
              "value": {
                "min_value": 70000,
                "max_value": 90000,
                "unit": "USD"
              }
            }
          },
          "hiring_team": {
            "hiring_managers": [
              {
                "id": 84275,
                "first_name": "Kaylee",
                "last_name": "Prime",
                "name": "Kaylee Prime",
                "employee_id": "13636"
              },
              {
                "id": 169779,
                "first_name": "Hank",
                "last_name": "Hollandaise",
                "name": "Hank Hollandaise",
                "employee_id": "34537"
              }
            ],
            "recruiters": [
              {
                "id": 81111,
                "first_name": "Samuel",
                "last_name": "Skateboard",
                "name": "Samuel Skateboard",
                "employee_id": "34531",
                "responsible": false
              },
              {
                "id": 153448,
                "first_name": "Stegosaurus",
                "last_name": "Heels",
                "name": "Stegosaurus Heels",
                "employee_id": "45748",
                "responsible": true
              }
            ],
            "coordinators": [
              {
                "id": 122635,
                "first_name": "Teddy",
                "last_name": "Pizzazz",
                "name": "Teddy Pizzazz",
                "employee_id": "47327",
                "responsible": true
              },
              {
                "id": 177046,
                "first_name": "Mirandella",
                "last_name": "Lager",
                "name": "Mirandella Lager",
                "employee_id": "43626",
                "responsible": false
              }
            ],
            "sourcers": [
              {
                "id": 122635,
                "first_name": "Teddy",
                "last_name": "Pizzazz",
                "name": "Teddy Pizzazz",
                "employee_id": "47327"
              }
            ]
          },
          "openings": [
            {
              "id": 123,
              "opening_id": "3-1",
              "status": "open",
              "opened_at": "2015-11-20T23:14:14.736Z",
              "closed_at": "2017-11-20T23:14:14.736Z",
              "application_id": 45678,
              "close_reason": {
                "id": 678,
                "name": "Hired - Backfill"
              }
            },
            {
              "id": 124,
              "opening_id": "3-2",
              "status": "open",
              "opened_at": "2015-11-20T23:14:14.739Z",
              "closed_at": null,
              "application_id": null,
              "close_reason": null
            },
            {
              "id": 125,
              "opening_id": null,
              "status": "open",
              "opened_at": "2016-02-03T20:00:00.000Z",
              "closed_at": null,
              "application_id": null
            },
            {
              "id": 126,
              "opening_id": "2-4",
              "status": "closed",
              "opened_at": "2016-02-03T16:38:46.985Z",
              "closed_at": "2016-02-03T16:39:09.811Z",
              "application_id": 1232,
              "close_reason": {
                "id": 689,
                "name": "Hired"
              }
            }
          ]
        }
      ]"#;

    let model: jobs::Jobs = serde_json::from_str(&json).unwrap();

    let model = &model[0];

    println!("{}", &model.name);
    println!("{}", &model.id);
    println!("{}", &model.requisition_id);

    let settings = Config::builder()
        .add_source(config::File::with_name("settings/Settings"))
        .build();

    match &settings {
            Ok(cfg) => println!("A config was found: {:#?}", cfg),
            Err(e) => println!("Settings.toml could not be found. Error - {}", e),
        }

    let setting = match settings.unwrap().deserialize::<HashMap<String, String>>()
        {
            Ok(json) => {json},
            Err(e) =>{ 
                println!("Settings.toml file not did not contain a valid API key. Error - {}", e);
                HashMap::new()
            },
        };

    println!("{:?}", setting);
}
