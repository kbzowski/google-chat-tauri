use std::env;
use std::time::Duration;

const FALLBACK_FIREFOX_VERSION: &str = "137.0";
const MOZILLA_API: &str = "https://product-details.mozilla.org/1.0/firefox_versions.json";

fn fetch_firefox_version() -> Option<String> {
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(5))
        .build();
    let resp = agent.get(MOZILLA_API).call().ok()?;
    let json: serde_json::Value = resp.into_json().ok()?;
    json.get("LATEST_FIREFOX_VERSION")?
        .as_str()
        .map(String::from)
}

fn main() {
    let firefox_version = match env::var("FIREFOX_VERSION") {
        Ok(v) => v,
        Err(_) => match fetch_firefox_version() {
            Some(v) => v,
            None => {
                println!(
                    "cargo:warning=Could not fetch Firefox version from {}, using fallback {}",
                    MOZILLA_API, FALLBACK_FIREFOX_VERSION
                );
                FALLBACK_FIREFOX_VERSION.to_string()
            }
        },
    };

    let user_agent = format!(
        "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:{ver}) Gecko/20100101 Firefox/{ver}",
        ver = firefox_version
    );
    let build_date = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

    println!("cargo:rustc-env=FIREFOX_VERSION={}", firefox_version);
    println!("cargo:rustc-env=FIREFOX_USER_AGENT={}", user_agent);
    println!("cargo:rustc-env=BUILD_DATE={}", build_date);
    println!("cargo:rerun-if-env-changed=FIREFOX_VERSION");
    println!("cargo:rerun-if-changed=injection.js");

    tauri_build::build();
}
