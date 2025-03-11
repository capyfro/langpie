#![feature(iter_advance_by)]
mod structs;

use std::{collections::HashMap, ops::Add};

use colored::Colorize;
use piechart::{Color, Data};
use rand::{random, seq::IndexedRandom};
use reqwest::Client;
use serde::de::DeserializeOwned;
use structs::Repo;
use url::Url;

fn main() {
    let auth = format!(
        "token {}",
        std::env::var("FORGEJO_AUTH_LANG_PIE")
            .expect("Forgejo authentication must be provided in $FORGEJO_AUTH_LANG_PIE")
    );
    let mut url = Url::parse(
        &std::env::var("FORGEJO_URL").expect("Forgejo URL must be provided in $FORGEJO_URL"),
    )
    .expect("Could not parse FORGEJO_URL");
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    if let Ok(o) = rt.block_on(portify::resolve_svcb(&url)) {
        url = o;
    }
    let res = rt.block_on(forgejo(auth, url)).unwrap();
    make_chart(res);
}

fn make_chart(data: HashMap<String, u64>) {
    let mut chart_data = vec![];
    let possible_chars = vec!['•', '▪', '▴'];
    for (k, v) in data {
        let mut rng = rand::rng();
        chart_data.push(Data {
            label: k,
            value: v as f32,
            color: Some(Color::Fixed(random()).into()),
            fill: possible_chars.choose(&mut rng).unwrap().clone(),
        })
    }
    piechart::Chart::new()
        .radius(9)
        .aspect_ratio(3)
        .legend(true)
        .draw(&chart_data);
}

async fn forgejo(auth: String, url: Url) -> Result<HashMap<String, u64>, reqwest::Error> {
    println!("Receiving repositories...");
    let repos: Vec<Repo> = reqwest(
        &auth,
        url.clone().join("user/repos?page=1&limit=-1").unwrap(),
    )
    .await?;
    println!("{}", "Repositories received!".bright_green());
    let mut lang_map: HashMap<String, u64> = HashMap::new();
    println!("Receiving repository languages...");
    for repo in repos {
        let languages: serde_json::Value = reqwest(
            &auth,
            url.clone()
                .join(&format!("repos/{}/languages", repo.full_name))
                .unwrap(),
        )
        .await?;
        match languages.as_object() {
            Some(o) => {
                for (lang, freq) in o.iter() {
                    let lang = lang.as_str().to_string();
                    let freq = freq.as_u64().unwrap_or(0);
                    if freq == 0 {
                        println!(
                            "{}{}{}{}",
                            "Could not parse ".red(),
                            freq,
                            " for ".red(),
                            lang
                        )
                    }
                    if lang_map.contains_key(&lang) {
                        if let Some(e) = lang_map.get_mut(&lang) {
                            *e = e.add(freq);
                        }
                    } else {
                        lang_map.insert(lang, freq);
                    }
                }
            }
            None => continue,
        }
    }
    println!("{}", "Ok! Building chart...".bright_green());
    Ok(lang_map)
}

async fn reqwest<U>(auth: &str, url: Url) -> Result<U, reqwest::Error>
where
    U: DeserializeOwned,
{
    let client = Client::new();
    let req = client
        .get(url)
        .header("Authorization", auth)
        .header("Accept", "application/json")
        .send()
        .await?;
    Ok(req.json().await?)
}
