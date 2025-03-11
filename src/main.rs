#![feature(iter_advance_by)]
mod structs;

use hickory_resolver::proto::rr::rdata::svcb::SvcParamKey;
use reqwest::Client;
use serde::{Deserialize, de::DeserializeOwned};
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
    if let Some(o) = rt.block_on(check_dns(&url)) {
        url = o;
    }
    let quota = rt.block_on(forgejo(auth, url));
    println!("{:?}", quota)
}

async fn forgejo(auth: String, url: Url) -> Result<Vec<Repo>, reqwest::Error> {
    let repos: Vec<Repo> = reqwest(
        &auth,
        url.clone().join("user/repos?page=1&limit=-1").unwrap(),
    )
    .await?;
    Ok(repos)
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

async fn check_dns(url: &Url) -> Option<Url> {
    println!("{}", url);
    let dom = url.domain()?;
    let resolv = hickory_resolver::TokioAsyncResolver::tokio_from_system_conf().ok()?;
    let res = resolv
        .lookup(dom, hickory_resolver::proto::rr::RecordType::HTTPS)
        .await
        .ok()?;
    let rec = res
        .record_iter()
        .filter(|x| x.name().to_string().eq(&format!("{}.", dom)))
        .next()?;
    let rdata = rec.clone().into_data()?.into_https().ok()?;
    for (key, param) in rdata.0.svc_params() {
        if key == &SvcParamKey::Port {
            let mut url = url.clone();
            url.set_port(param.clone().into_port().ok()).ok()?;
            return Some(url.clone());
        }
    }
    Some(url.clone())
}
