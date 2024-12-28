use client::HttpClient;
use reqwest::header::{self, HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::cli::Parameters;
use crate::{Error, Result};
mod client;

const CHECK_IP_URL: &str = "https://httpbin.org/ip";
const CF_BASE_URL: &str = "https://api.cloudflare.com/client/v4/zones";
const CF_MAIL_TOKEN_NAME: &str = "X-Auth-Email";

#[derive(Deserialize)]
struct Ip {
    origin: String,
}

#[derive(Deserialize)]
struct CloudflareResultList<T> {
    result: Vec<T>,
}

#[derive(Deserialize)]
struct CloudflareResultSingle<T> {
    result: T,
}

#[derive(Deserialize, Debug)]
struct DnsZone {
    id: String,
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct DnsRecord {
    #[serde(skip_serializing)]
    id: String,
    content: String,
    comment: Option<String>,
    name: String,
    #[serde(rename = "type")]
    kind: Option<String>,
}

fn get_default_headers(params: &Parameters) -> Result<HeaderMap> {
    let mut headers = HeaderMap::new();

    let mut auth_value = HeaderValue::from_str(&format!("Bearer {}", params.token))?;
    auth_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_value);

    let email_name = HeaderName::from_str(CF_MAIL_TOKEN_NAME)?;
    let mut email_value = HeaderValue::from_str(params.email.as_str())?;
    email_value.set_sensitive(true);
    headers.insert(email_name, email_value);

    Ok(headers)
}

async fn get_current_ip(client: &HttpClient) -> Result<String> {
    println!("Start requesting current ip...");
    client
        .do_get::<Ip>(CHECK_IP_URL)
        .await
        .map(|res| res.body.origin)
        .inspect(|ip| println!("Current ip is : {ip}"))
}

async fn get_zone(client: &HttpClient, domain: &str) -> Result<DnsZone> {
    println!("Start requesting zone...");
    client
        .do_get::<CloudflareResultList<DnsZone>>(CF_BASE_URL)
        .await
        .map(|res| res.body.result)?
        .into_iter()
        .find(|z| z.name == domain)
        .ok_or_else(|| Error::DnsZoneNotFound)
        .inspect(|zone| println!("Zone found : {zone:?}"))
}

async fn get_record(client: &HttpClient, domain: &str, zone_id: &str) -> Result<DnsRecord> {
    println!("Start requesting record...");
    let url = format!("{CF_BASE_URL}/{zone_id}/dns_records");
    client
        .do_get::<CloudflareResultList<DnsRecord>>(&url)
        .await
        .map(|res| res.body.result)?
        .into_iter()
        .find(|record| record.name == domain)
        .ok_or_else(|| Error::DnsRecordNotFound)
        .inspect(|record| println!("Record found : {record:?}"))
}

async fn put_record(client: &HttpClient, record: &DnsRecord, zone_id: &str) -> Result<DnsRecord> {
    let url = format!("{CF_BASE_URL}/{zone_id}/dns_records/{}", record.id);
    let body = serde_json::to_string(record)?;

    println!("Start updating record with body {body:?}...");
    client
        .do_put::<CloudflareResultSingle<DnsRecord>>(&url, body)
        .await
        .map(|res| res.body.result)
        .inspect(|record| println!("Record updated : {record:?}"))
}

pub async fn update_dns_record(params: Parameters) -> Result<()> {
    let client = HttpClient::default();
    let current_ip = get_current_ip(&client).await?;

    // Now we need cloudflare headers
    let headers = get_default_headers(&params)?;
    let client = HttpClient::with_headers(headers)?;

    let zone = get_zone(&client, &params.domain).await?;
    let mut record = get_record(&client, &params.domain, &zone.id).await?;

    // Skip update
    if current_ip == record.content {
        println!(
            "Current ip {current_ip} matches record's ip {}, skipping update.",
            &record.content
        );

    // Do update
    } else {
        println!(
            "The current ip {current_ip} does not match the record's ip {}",
            record.content
        );
        record.content = current_ip;
        record.comment = Some("Powered by Rust !".to_string());
        put_record(&client, &record, &zone.id).await?;
    }
    Ok(())
}
