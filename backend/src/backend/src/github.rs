use crate::models::{GitHubUser, Repository};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse,
    TransformArgs, TransformContext,
};
use ic_cdk::query;
use serde_json;

#[query]
fn transform(raw: TransformArgs) -> HttpResponse {
    let mut res = HttpResponse {
        status: raw.response.status.clone(),
        ..Default::default()
    };
    if res.status == candid::Nat::from(200u16) {
        res.body = raw.response.body;
    } else {
        ic_cdk::api::print(format!("Received an error from github: {:?}", raw));
    }
    res
}

pub async fn fetch_user_repos(username: &str) -> Result<Vec<Repository>, String> {
    let host = "api.github.com";
    let url = format!("/users/{}/repos", username);
    let request_headers = vec![HttpHeader { name: "User-Agent".to_string(), value: "veriflair-icp-hackathon".to_string() }];
    let request = CanisterHttpRequestArgument {
        url: format!("https://{host}{url}"),
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: Some(2_000_000),
        transform: Some(TransformContext::from_name("transform".to_string(), vec![])),
        headers: request_headers,
    };
    match http_request(request, 30_000_000_000).await {
        Ok((res,)) => {
            if res.status == candid::Nat::from(200u16) {
                serde_json::from_slice::<Vec<Repository>>(&res.body).map_err(|e| format!("Failed to parse GitHub response: {}", e))
            } else {
                Err(format!("GitHub API returned an error: status {}, body {}", res.status, String::from_utf8(res.body).unwrap_or_default()))
            }
        }
        Err((code, msg)) => Err(format!("Failed to call GitHub API: ({:?}) {}", code, msg)),
    }
}

pub async fn fetch_user_profile(username: &str) -> Result<GitHubUser, String> {
    let host = "api.github.com";
    let url = format!("/users/{}", username);
    let request_headers = vec![HttpHeader { name: "User-Agent".to_string(), value: "veriflair-icp-hackathon".to_string() }];
    let request = CanisterHttpRequestArgument {
        url: format!("https://{host}{url}"),
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: Some(1_000_000),
        transform: Some(TransformContext::from_name("transform".to_string(), vec![])),
        headers: request_headers,
    };
    match http_request(request, 30_000_000_000).await {
        Ok((res,)) => {
            if res.status == candid::Nat::from(200u16) {
                serde_json::from_slice::<GitHubUser>(&res.body).map_err(|e| format!("Failed to parse GitHub user profile response: {}", e))
            } else {
                Err(format!("GitHub API returned an error for user profile: status {}, body {}", res.status, String::from_utf8(res.body).unwrap_or_default()))
            }
        }
        Err((code, msg)) => Err(format!("Failed to call GitHub API for user profile: ({:?}) {}", code, msg)),
    }
}