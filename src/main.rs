use std::io::Read;
use std::{env, io};

use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use url::Url;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
struct Source {
    host: String,
    username: Option<String>,
    password: Option<String>,
    token: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
struct Params {
    title: Option<String>,
    message: Option<String>,
    topic: String,
    tags: Option<Vec<String>>,
    priority: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
struct Input {
    source: Source,
    params: Params,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
struct Notification {
    topic: String,
    message: Option<String>,
    title: Option<String>,
    tags: Option<Vec<String>>,
    priority: Option<u8>,
    // actions: Option<_>, # not yet implemented
    click: Option<String>,
    attach: Option<String>,
    markdown: Option<bool>,
    icon: Option<String>,
    filename: Option<String>,
    delay: Option<String>,
    email: Option<String>,
    call: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct ConcourseMetadata {
    build_id: Option<String>,
    build_name: Option<String>,
    build_team_id: Option<String>,
    build_team_name: Option<String>,
    build_job_id: Option<String>,
    build_job_name: Option<String>,
    build_pipeline_id: Option<String>,
    build_pipeline_name: Option<String>,
    atc_external_url: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
struct ConcourseVersion {
    time: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
struct ConcourseReturn {
    version: ConcourseVersion,
    metadata: Vec<Value>,
}

fn check_resource() -> Result<()> {
    println!("[]");
    Ok(())
}

fn in_resource() -> Result<()> {
    let version = r#"
        {
            "version": {
                "time": "static",
                "stage": "in"
            }
        }"#;
    println!("{}", version);
    Ok(())
}

fn out_resource() -> Result<()> {
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;

    let input_data: Input = serde_json::from_slice(&buffer)?;
    let url: Url = Url::parse(&input_data.source.host)?;

    let notification = Notification {
        topic: input_data.params.topic,
        message: input_data.params.message,
        title: input_data.params.title,
        tags: input_data.params.tags,
        priority: input_data
            .params
            .priority
            .map(|s| s.parse::<u8>().map_err(anyhow::Error::from))
            .transpose()?,
        click: None,
        attach: None,
        markdown: None,
        icon: None,
        filename: None,
        delay: None,
        email: None,
        call: None,
    };

    let mut response_buffer = String::new();
    let http_client = reqwest::blocking::Client::new();
    let mut request = http_client.post(url).json(&notification);

    if let Some(token) = input_data.source.token {
        request = request.bearer_auth(token);
    } else if let (Some(user), pass) = (&input_data.source.username, input_data.source.password) {
        request = request.basic_auth(user, pass);
    }

    request.send()?.read_to_string(&mut response_buffer)?;

    // println!("Got message: {}", response_buffer);
    let mut metadata = Vec::new();
    let env_names = [
        "BUILD_ID",
        "BUILD_NAME",
        "BUILD_TEAM_ID",
        "BUILD_TEAM_NAME",
        "BUILD_JOB_ID",
        "BUILD_JOB_NAME",
        "BUILD_PIPELINE_ID",
        "BUILD_PIPELINE_NAME",
        "ATC_EXTERNAL_URL",
    ];

    for env_name in env_names {
        if let Ok(value) = env::var(env_name) {
            metadata.push(json!({ "name": env_name, "value": value }));
        }
    }

    let version = ConcourseVersion {
        time: Utc::now().timestamp().to_string(),
    };

    let concourse_return = ConcourseReturn {
        version: version,
        metadata: metadata,
    };

    let concourse_return_output = serde_json::to_string(&concourse_return)?;
    println!("{}", concourse_return_output);

    Ok(())
}

fn main() -> Result<()> {
    if let Some((executable, _args)) = env::args().collect::<Vec<String>>().split_first() {
        match executable.as_str() {
            "/opt/resource/check" => check_resource()?,
            "/opt/resource/in" => in_resource()?,
            "/opt/resource/out" => out_resource()?,
            _ => panic!("you did something majorly wrong..."),
        }
    };
    Ok(())
}
