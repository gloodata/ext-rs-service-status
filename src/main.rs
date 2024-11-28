use axum::{extract::Json, http::StatusCode, routing::post, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct StatusRoot {
    page: StatusPageInfo,
    components: Vec<StatusEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
struct StatusPageInfo {
    id: String,
    name: String,
    time_zone: String,
    updated_at: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct StatusEntry {
    created_at: String,
    updated_at: String,
    start_date: Option<String>,
    description: Option<String>,
    name: String,
    status: String,
    position: i64,
    id: String,
    page_id: String,
    group: bool,
    group_id: Option<String>,
    showcase: bool,
    only_show_if_degraded: bool,
}

enum Service {
    OpenAI,
    Cloudflare,
    Discord,
    //Intercom,
    Dropbox,
    DigitalOcean,
    HubSpot,
    Github,
    Bitbucket,
    Sendgrid,
    Snowflake,
    Twilio,
    Npm,
    Akamai,
    Twitch,
    SquareSpace,
    NewRelic,
    Reddit,
    Coinbase,
    //Twitter,
}

const SERVICE_NAME_OPENAI: &str = "openai";
const SERVICE_NAME_CLOUDFLARE: &str = "cloudflare";
const SERVICE_NAME_DISCORD: &str = "discord";
const SERVICE_NAME_DROPBOX: &str = "dropbox";
const SERVICE_NAME_DIGITALOCEAN: &str = "digitalocean";
const SERVICE_NAME_HUBSPOT: &str = "hubspot";
const SERVICE_NAME_GITHUB: &str = "github";
const SERVICE_NAME_BITBUCKET: &str = "bitbucket";
const SERVICE_NAME_SENDGRID: &str = "sendgrid";
const SERVICE_NAME_SNOWFLAKE: &str = "snowflake";
const SERVICE_NAME_TWILIO: &str = "twilio";
const SERVICE_NAME_NPM: &str = "npm";
const SERVICE_NAME_AKAMAI: &str = "akamai";
const SERVICE_NAME_TWITCH: &str = "twitch";
const SERVICE_NAME_SQUARESPACE: &str = "squarespace";
const SERVICE_NAME_NEWRELIC: &str = "newrelic";
const SERVICE_NAME_REDDIT: &str = "reddit";
const SERVICE_NAME_COINBASE: &str = "coinbase";

const DEFAULT_SERVICE_NAME: &str = SERVICE_NAME_GITHUB;

impl Service {
    fn to_entries() -> Vec<(String, String)> {
        vec![
            (SERVICE_NAME_OPENAI.into(), "OpenAI".into()),
            (SERVICE_NAME_CLOUDFLARE.into(), "Cloudflare".into()),
            (SERVICE_NAME_DISCORD.into(), "Discord".into()),
            (SERVICE_NAME_DROPBOX.into(), "Dropbox".into()),
            (SERVICE_NAME_DIGITALOCEAN.into(), "Digital Ocean".into()),
            (SERVICE_NAME_HUBSPOT.into(), "Hubspot".into()),
            (SERVICE_NAME_GITHUB.into(), "Github".into()),
            (SERVICE_NAME_BITBUCKET.into(), "Bitbucket".into()),
            (SERVICE_NAME_SENDGRID.into(), "Sendgrid".into()),
            (SERVICE_NAME_SNOWFLAKE.into(), "Snowflake".into()),
            (SERVICE_NAME_TWILIO.into(), "Twilio".into()),
            (SERVICE_NAME_NPM.into(), "Npm".into()),
            (SERVICE_NAME_AKAMAI.into(), "Akamai".into()),
            (SERVICE_NAME_TWITCH.into(), "Twitch".into()),
            (SERVICE_NAME_SQUARESPACE.into(), "Squarespace".into()),
            (SERVICE_NAME_NEWRELIC.into(), "New Relic".into()),
            (SERVICE_NAME_REDDIT.into(), "Reddit".into()),
            (SERVICE_NAME_COINBASE.into(), "Coinbase".into()),
        ]
    }

    fn to_keys() -> Vec<String> {
        vec![
            SERVICE_NAME_OPENAI.into(),
            SERVICE_NAME_CLOUDFLARE.into(),
            SERVICE_NAME_DISCORD.into(),
            SERVICE_NAME_DROPBOX.into(),
            SERVICE_NAME_DIGITALOCEAN.into(),
            SERVICE_NAME_HUBSPOT.into(),
            SERVICE_NAME_GITHUB.into(),
            SERVICE_NAME_BITBUCKET.into(),
            SERVICE_NAME_SENDGRID.into(),
            SERVICE_NAME_SNOWFLAKE.into(),
            SERVICE_NAME_TWILIO.into(),
            SERVICE_NAME_NPM.into(),
            SERVICE_NAME_AKAMAI.into(),
            SERVICE_NAME_TWITCH.into(),
            SERVICE_NAME_SQUARESPACE.into(),
            SERVICE_NAME_NEWRELIC.into(),
            SERVICE_NAME_REDDIT.into(),
            SERVICE_NAME_COINBASE.into(),
        ]
    }

    fn from_name(s: &str) -> Option<Service> {
        match s {
            SERVICE_NAME_OPENAI => Some(Service::OpenAI),
            SERVICE_NAME_CLOUDFLARE => Some(Service::Cloudflare),
            SERVICE_NAME_DISCORD => Some(Service::Discord),
            SERVICE_NAME_DROPBOX => Some(Service::Dropbox),
            SERVICE_NAME_DIGITALOCEAN => Some(Service::DigitalOcean),
            SERVICE_NAME_HUBSPOT => Some(Service::HubSpot),
            SERVICE_NAME_GITHUB => Some(Service::Github),
            SERVICE_NAME_BITBUCKET => Some(Service::Bitbucket),
            SERVICE_NAME_SENDGRID => Some(Service::Sendgrid),
            SERVICE_NAME_SNOWFLAKE => Some(Service::Snowflake),
            SERVICE_NAME_TWILIO => Some(Service::Twilio),
            SERVICE_NAME_NPM => Some(Service::Npm),
            SERVICE_NAME_AKAMAI => Some(Service::Akamai),
            SERVICE_NAME_TWITCH => Some(Service::Twitch),
            SERVICE_NAME_SQUARESPACE => Some(Service::SquareSpace),
            SERVICE_NAME_NEWRELIC => Some(Service::NewRelic),
            SERVICE_NAME_REDDIT => Some(Service::Reddit),
            SERVICE_NAME_COINBASE => Some(Service::Coinbase),
            //"intercom" => Some(Service::Intercom),
            //"twitter" => Some(Service::Twitter),
            _ => None,
        }
    }

    fn to_base_url(&self) -> String {
        match self {
            Service::OpenAI => "status.openai.com".into(),
            Service::Cloudflare => "www.cloudflarestatus.com".into(),
            Service::Discord => "discordstatus.com".into(),
            //Service::Intercom => "www.intercomstatus.com".into(),
            Service::Dropbox => "status.dropbox.com".into(),
            Service::DigitalOcean => "status.digitalocean.com".into(),
            Service::HubSpot => "status.hubspot.com".into(),
            Service::Github => "www.githubstatus.com".into(),
            Service::Bitbucket => "bitbucket.status.atlassian.com".into(),
            Service::Sendgrid => "status.sendgrid.com".into(),
            Service::Snowflake => "status.snowflake.com".into(),
            Service::Twilio => "status.twilio.com".into(),
            Service::Npm => "status.npmjs.org".into(),
            Service::Akamai => "www.akamaistatus.com".into(),
            Service::Twitch => "status.twitch.com".into(),
            Service::SquareSpace => "status.squarespace.com".into(),
            Service::NewRelic => "status.newrelic.com".into(),
            Service::Reddit => "www.redditstatus.com".into(),
            Service::Coinbase => "status.coinbase.com".into(),
            //Service::Twitter => "api.twitterstat.us".into(),
        }
    }

    fn to_url(&self) -> String {
        let base_url = self.to_base_url();
        format!("https://{base_url}/api/v2/components.json")
    }
}

async fn get_status_for_url(url: &str) -> Result<serde_json::Value, reqwest::Error> {
    reqwest::Client::new().get(url).send().await?.json().await
}

#[derive(Debug)]
pub enum GetStatusError {
    UnknownService { name: String },
    RequestError { error: reqwest::Error },
    ParseError { error: serde_json::Error },
}

async fn get_status_for_service_name(name: &str) -> Result<StatusRoot, GetStatusError> {
    match Service::from_name(name) {
        Some(service) => {
            let url = service.to_url();
            match get_status_for_url(&url).await {
                Ok(value) => match serde_json::from_value::<StatusRoot>(value) {
                    Ok(status) => Ok(status),
                    Err(err) => Err(GetStatusError::ParseError { error: err }),
                },

                Err(err) => Err(GetStatusError::RequestError { error: err }),
            }
        }

        None => Err(GetStatusError::UnknownService { name: name.into() }),
    }
}

pub async fn print_status(name: &str) {
    match get_status_for_service_name(name).await {
        Ok(status) => {
            println!("{status:?}");
            //println!("ok: {name:?}");
        }
        Err(err) => {
            eprintln!("{name}: {err:?}");
        }
    }
}

const H_GET_ENUM_SERVICES: &str = "getEnumServices";
const H_SERVICE_STATUS_FOR_ENUM_SERVICE_ID: &str = "serviceStatusForEnumServiceId";
const ENUM_SERVICE_ID: &str = "ServiceId";
const FN_GET_SERVICE_STATUS: &str = "getServiceStatus";

#[tokio::main]
async fn main() {
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8890".to_string());
    let bind_address = format!("{}:{}", host, port);
    let app = Router::new().route("/", post(handler));
    let listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();
    println!("Listening at {}", bind_address);
    axum::serve(listener, app).await.unwrap();
}

fn get_info() -> serde_json::Value {
    json!({
        "ns": "ext-rs-service-status",
        "title": "Service Status",
        "tagValues": {
            ENUM_SERVICE_ID: {"icon": "server", "loadEntriesHandlerId": H_GET_ENUM_SERVICES}
        },
        "tools": {
            FN_GET_SERVICE_STATUS: {
                "title": "Get Service Status",
                "description":
                    "Check service API status to know if a service is up or down",
                "schema": {
                    "fields": {
                        "service": {
                            "type": "string",
                            "enum": Service::to_keys(),
                            "description": "the service to display the API status, if unknown default to github"
                        }
                    },
                },
                "ui": {
                  "prefix": "Service Status",
                  "args": {
                      "service": {
                          "prefix": "For",
                           "dtypeName": ENUM_SERVICE_ID
                       }
                   },
                },
                "contextActions": [
                  {
                    "for": { "name": ENUM_SERVICE_ID },
                    "handler": H_SERVICE_STATUS_FOR_ENUM_SERVICE_ID,
                  },
                ],
                "examples": [
                  "Show status for snowflake",
                  "Cloudflare service status",
                  "Is twilio up?",
                ],
            }
        }
    })
}

fn res_error(code: &str, reason: &str) -> serde_json::Value {
    json!({"ok": false, "code": code, "reason": reason})
}

fn fail(code: &str, reason: &str) -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::BAD_REQUEST, Json(res_error(code, reason)))
}

fn ok(result: serde_json::Value) -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::OK, Json(result))
}

pub async fn handler(
    axum::extract::Json(data0): axum::extract::Json<serde_json::Value>,
) -> (StatusCode, Json<serde_json::Value>) {
    let data: serde_json::Value = data0;
    // you don't need to read this, it's not going to change
    if let Some(obj) = data.as_object() {
        if let Some(action) = obj.get("action").and_then(|v| v.as_str()) {
            if action == "request" {
                if let Some(op_name) = obj.get("opName").and_then(|v| v.as_str()) {
                    if let Some(req_info) = obj.get("info") {
                        ok(handle_request(op_name, req_info).await)
                    } else {
                        fail("BadOpInfo", "Operation Info Not Found")
                    }
                } else {
                    fail("BadOpName", "Operation Name Not Defined")
                }
            } else if action == "info" {
                (StatusCode::OK, Json(get_info()))
            } else {
                fail("BadAction", "Bad Action")
            }
        } else {
            fail("ActionNotFound", "Bad Request Body Format (No Action)")
        }
    } else {
        fail("BadBodyFormat", "Bad Request Body Format")
    }
}

#[derive(Serialize, Deserialize)]
struct GetServiceStatusInfo {
    service: Option<String>,
}

fn status_root_to_headless_table(status: StatusRoot) -> serde_json::Value {
    let rows: Vec<Vec<serde_json::Value>> = status
        .components
        .into_iter()
        .map(|entry| {
            vec![
                json!(entry.name),
                json!(entry.status),
                json!(["datetime", {"iso": entry.updated_at}]),
            ]
        })
        .collect();

    json!({
        "info": {
            "type": "table",
            "cols": [["name", "Name"], ["status", "Status"], ["date", "Date"]]
        },
        "data": {
            "cols": ["name", "status", "date"],
            "rows": rows
        }
    })
}

pub async fn handle_request(op_name: &str, req_info: &serde_json::Value) -> serde_json::Value {
    match op_name {
        FN_GET_SERVICE_STATUS => {
            match serde_json::from_value::<GetServiceStatusInfo>(req_info.clone()) {
                Ok(info) => {
                    let name = info
                        .service
                        .unwrap_or_else(|| DEFAULT_SERVICE_NAME.to_string());
                    match get_status_for_service_name(&name).await {
                        Ok(status) => status_root_to_headless_table(status),
                        Err(err) => {
                            eprintln!("{name}: {err:?}");
                            res_error("BadResponse", "Bad Response")
                        }
                    }
                }
                Err(err) => {
                    eprintln!("{err:?}");
                    res_error("BadReqInfoFormat", "Bad Request Info Format")
                }
            }
        }
        H_SERVICE_STATUS_FOR_ENUM_SERVICE_ID => {
            let service_id = req_info.pointer("/info/service");
            if let Some(service_id) = service_id {
                json!({"name": FN_GET_SERVICE_STATUS, "args": {"service": service_id}})
            } else {
                json!({"name": FN_GET_SERVICE_STATUS, "args": {"service": SERVICE_NAME_GITHUB}})
            }
        }
        H_GET_ENUM_SERVICES => {
            json!({
                "info": null,
                "entries": Service::to_entries()
            })
        }
        _ => res_error("UnknownOpName", "Unknown Operation Name"),
    }
}
