use std::path::PathBuf;
use std::time::Duration;

const ARTIFACTS_URL: &str =
    "https://raw.githubusercontent.com/namefailed/chaff-artifacts/main/artifacts.json";

const MAX_AGE_SECS: u64 = 86_400; // 24 hours
const FETCH_TIMEOUT: Duration = Duration::from_secs(3);

/// Load order: fresh disk cache → fetched from GitHub → stale disk cache → bundled fallback.
/// Returns None only if the bundled JSON is also broken (shouldn't happen).
pub fn load_or_fetch() -> Option<String> {
    let path = cache_path()?;

    if is_fresh(&path) {
        if let Ok(s) = std::fs::read_to_string(&path) {
            return Some(s);
        }
    }

    if let Ok(body) = fetch() {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(&path, &body);
        return Some(body);
    }

    std::fs::read_to_string(&path).ok()
}

fn fetch() -> Result<String, ()> {
    let body = ureq::AgentBuilder::new()
        .timeout(FETCH_TIMEOUT)
        .build()
        .get(ARTIFACTS_URL)
        .call()
        .map_err(|_| ())?
        .into_string()
        .map_err(|_| ())?;

    // Reject anything that isn't valid JSON
    serde_json::from_str::<serde_json::Value>(&body).map_err(|_| ())?;
    Ok(body)
}

fn cache_path() -> Option<PathBuf> {
    Some(PathBuf::from(std::env::var("APPDATA").ok()?).join("chaff").join("artifacts.json"))
}

fn is_fresh(path: &PathBuf) -> bool {
    path.metadata()
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.elapsed().ok())
        .map(|e| e.as_secs() < MAX_AGE_SECS)
        .unwrap_or(false)
}
