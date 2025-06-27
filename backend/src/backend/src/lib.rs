mod github;
mod models;
// mod llm;
// mod utils;

use crate::models::{GitHubUser, Repository};
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};

#[ic_cdk::update]
async fn get_user_repositories_test(username: String) -> Result<Vec<Repository>, String> {
    if username.is_empty() {
        return Err("GitHub username cannot be empty.".to_string());
    }
    github::fetch_user_repos(&username).await
}

#[ic_cdk::update]
async fn get_user_profile_test(username: String) -> Result<GitHubUser, String> {
    if username.is_empty() {
        return Err("GitHub username cannot be empty.".to_string());
    }
    github::fetch_user_profile(&username).await
}

ic_cdk::export_candid!();