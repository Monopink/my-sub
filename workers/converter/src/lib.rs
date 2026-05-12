use subconverter::api::sub::{sub_process, SubconverterQuery};
use subconverter::Settings;
use subconverter::update_settings_from_content;
use std::collections::HashMap;
use std::sync::OnceLock;
use worker::*;

static PREF_READY: OnceLock<bool> = OnceLock::new();
static DEFAULT_PREF_CONTENT: &str = include_str!("../../../base/config/my-sub-pref.ini");

fn error_response(message: &str, status: u16) -> Result<Response> {
    Ok(Response::from_json(&serde_json::json!({ "error": message }))?.with_status(status))
}

fn build_upstream_request_headers(req: &Request) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    if let Ok(Some(ua)) = req.headers().get("user-agent") {
        let ua = ua.trim();
        if !ua.is_empty() {
            headers.insert("User-Agent".to_string(), ua.to_string());
        }
    }
    headers
}

async fn ensure_pref_loaded(env: &Env) -> Result<(), String> {
    if PREF_READY.get().copied().unwrap_or(false) {
        return Ok(());
    }

    // Prefer optional runtime override; otherwise use repo-embedded default.
    let pref_content = match env.var("SUBCONVERTER_PREF_CONTENT") {
        Ok(value) => {
            let from_env = value.to_string();
            if from_env.trim().is_empty() {
                DEFAULT_PREF_CONTENT.to_string()
            } else {
                from_env
            }
        }
        Err(_) => DEFAULT_PREF_CONTENT.to_string(),
    };
    if pref_content.trim().is_empty() {
        return Err("embedded pref content is empty".to_string());
    }

    update_settings_from_content(&pref_content)
        .await
        .map_err(|err| format!("invalid pref content: {err}"))?;

    // `sub_process` checks `pref_path` and will try to reload from local files
    // when it is empty. In Workers, local pref files do not exist, so we mark
    // settings as initialized from an in-memory source.
    {
        let mut global = Settings::current_mut();
        let settings = std::sync::Arc::make_mut(&mut *global);
        settings.pref_path = "embedded://base/config/my-sub-pref.ini".to_string();
        settings.reload_conf_on_request = false;
    }

    let _ = PREF_READY.set(true);
    Ok(())
}

#[event(fetch)]
pub async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    if req.method() != Method::Get {
        return error_response("method not allowed", 405);
    }

    let url = req.url()?;
    if url.path() != "/sub" {
        return error_response("not found", 404);
    }

    let mut query = match serde_urlencoded::from_str::<SubconverterQuery>(url.query().unwrap_or("")) {
        Ok(q) => q,
        Err(err) => return error_response(&format!("invalid query: {err}"), 400),
    };

    if query.request_headers.is_none() {
        let upstream_headers = build_upstream_request_headers(&req);
        if !upstream_headers.is_empty() {
            query.request_headers = Some(upstream_headers);
        }
    }

    if let Err(err) = ensure_pref_loaded(&env).await {
        return error_response(&format!("converter settings unavailable: {err}"), 500);
    }

    let result = match sub_process(None, query).await {
        Ok(resp) => resp,
        Err(err) => return error_response(&format!("conversion failed: {err}"), 500),
    };

    let mut response = Response::ok(result.content)?;
    response.headers_mut().set("content-type", &result.content_type)?;
    response = response.with_status(result.status_code);

    for (key, value) in result.headers {
        let _ = response.headers_mut().set(&key, &value);
    }

    Ok(response)
}
