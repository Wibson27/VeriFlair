mod github;
// mod llm; 
mod models;
// mod utils; 

// struct Repository
use crate::models::Repository;

#[ic_cdk::update]
async fn get_user_repositories_test(username: String) -> Result<Vec<Repository>, String> {
    if username.is_empty() {
        return Err("GitHub username cannot be empty.".to_string());
    }
    github::fetch_user_repos(&username).await
}

ic_cdk::export_candid!();