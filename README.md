# greenhouse.io-exporter
Export Greenhouse.io resumes and cover letters

My first attempt at a Rust app. This will download attachments of type "cover_letter" or "resume" from Greenhouse.io's API based on selections made by the user.

Jobs are listed first and once a job is selected, the stages of that job posting will be listed. Select a job stage and the application lists out all candidates in "active" status for that job. The user can then download all resumes and cover letters for those applicants.

All settings are stored in /settings/Settings.toml. This is where the API key must be placed.
