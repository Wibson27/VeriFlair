use crate::models::{GitHubAnalysis, GitHubData, GitHubOAuthRequest, GitHubOAuthResponse, Repository, HttpRequest, HttpResponse};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, TransformArgs,
};
use ic_cdk::api::time;
use serde_json::Value;
use std::collections::HashMap;

// GitHub API Configuration
const GITHUB_API_BASE: &str = "https://api.github.com";
const GITHUB_OAUTH_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";

// OAuth Configuration (to be set by environment/init)
thread_local! {
    static GITHUB_CLIENT_ID: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
    static GITHUB_CLIENT_SECRET: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
}

pub fn set_github_oauth_config(client_id: String, client_secret: String) {
    GITHUB_CLIENT_ID.with(|id| *id.borrow_mut() = Some(client_id));
    GITHUB_CLIENT_SECRET.with(|secret| *secret.borrow_mut() = Some(client_secret));
}

/// Exchange OAuth code for access token
pub async fn exchange_oauth_code(oauth_request: GitHubOAuthRequest) -> Result<GitHubOAuthResponse, String> {
    let client_id = GITHUB_CLIENT_ID.with(|id| id.borrow().clone())
        .ok_or("GitHub client ID not configured")?;

    let client_secret = GITHUB_CLIENT_SECRET.with(|secret| secret.borrow().clone())
        .ok_or("GitHub client secret not configured")?;

    let body = format!(
        "client_id={}&client_secret={}&code={}&state={}",
        client_id, client_secret, oauth_request.code, oauth_request.state
    );

    let request = CanisterHttpRequestArgument {
        url: GITHUB_OAUTH_TOKEN_URL.to_string(),
        method: HttpMethod::POST,
        body: Some(body.into_bytes()),
        max_response_bytes: Some(8192), // INCREASED from 1024 to 8192
        transform: None,
        headers: vec![
            HttpHeader {
                name: "Accept".to_string(),
                value: "application/json".to_string(),
            },
            HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/x-www-form-urlencoded".to_string(),
            },
            HttpHeader {
                name: "User-Agent".to_string(),
                value: "VeriFlair-ICP-Canister".to_string(),
            },
        ],
    };

    match http_request(request, 25_000_000_000).await {
        Ok((response,)) => {
            if response.status == 200u64 {
                let body_str = String::from_utf8(response.body)
                    .map_err(|e| format!("Failed to parse response body: {}", e))?;

                let oauth_response: GitHubOAuthResponse = serde_json::from_str(&body_str)
                    .map_err(|e| format!("Failed to parse OAuth response: {}", e))?;

                Ok(oauth_response)
            } else {
                Err(format!("GitHub OAuth failed with status: {}", response.status))
            }
        }
        Err((r, m)) => Err(format!("HTTP request failed: {:?} - {}", r, m)),
    }
}

/// Fetch GitHub user profile data
pub async fn fetch_github_user(access_token: &str) -> Result<GitHubData, String> {
    let url = format!("{}/user", GITHUB_API_BASE);

    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: Some(16384), // INCREASED from 2048 to 16384
        transform: None,
        headers: vec![
            HttpHeader {
                name: "Authorization".to_string(),
                value: format!("Bearer {}", access_token),
            },
            HttpHeader {
                name: "Accept".to_string(),
                value: "application/vnd.github.v3+json".to_string(),
            },
            HttpHeader {
                name: "User-Agent".to_string(),
                value: "VeriFlair-ICP-Canister".to_string(),
            },
        ],
    };

    match http_request(request, 25_000_000_000).await {
        Ok((response,)) => {
            if response.status == 200u64 {
                let body_str = String::from_utf8(response.body)
                    .map_err(|e| format!("Failed to parse response body: {}", e))?;

                parse_github_user(&body_str)
            } else {
                Err(format!("GitHub API request failed with status: {}", response.status))
            }
        }
        Err((r, m)) => Err(format!("HTTP request failed: {:?} - {}", r, m)),
    }
}

/// Fetch user's repositories
pub async fn fetch_user_repositories(username: &str, access_token: Option<&str>) -> Result<Vec<Repository>, String> {
    let url = format!("{}/users/{}/repos?type=all&sort=updated&per_page=100", GITHUB_API_BASE, username);

    let mut headers = vec![
        HttpHeader {
            name: "Accept".to_string(),
            value: "application/vnd.github.v3+json".to_string(),
        },
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "VeriFlair-ICP-Canister".to_string(),
        },
    ];

    if let Some(token) = access_token {
        headers.push(HttpHeader {
            name: "Authorization".to_string(),
            value: format!("Bearer {}", token),
        });
    }

    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: Some(65536), // INCREASED from 32768 to 65536 for large repo lists
        transform: None,
        headers,
    };

    match http_request(request, 25_000_000_000).await {
        Ok((response,)) => {
            if response.status == 200u64 {
                let body_str = String::from_utf8(response.body)
                    .map_err(|e| format!("Failed to parse response body: {}", e))?;

                parse_repositories(&body_str)
            } else {
                Err(format!("GitHub API request failed with status: {}", response.status))
            }
        }
        Err((r, m)) => Err(format!("HTTP request failed: {:?} - {}", r, m)),
    }
}

/// Fetch user's contribution statistics
pub async fn fetch_user_stats(username: &str, access_token: Option<&str>) -> Result<(u32, u32, u32), String> {
    // This would typically require GraphQL API or scraping, for now we'll estimate
    // In a real implementation, you'd use GitHub's GraphQL API for contribution data

    let repos = fetch_user_repositories(username, access_token).await?;

    let total_repos = repos.len() as u32;
    let total_stars: u32 = repos.iter().map(|r| r.stars).sum();
    let total_forks: u32 = repos.iter().map(|r| r.forks).sum();

    // Estimate commits based on repository activity and age
    let estimated_commits = estimate_commit_count(&repos);

    Ok((total_repos, estimated_commits, total_stars))
}

/// Comprehensive GitHub analysis
pub async fn perform_comprehensive_analysis(username: &str, access_token: Option<&str>) -> Result<GitHubAnalysis, String> {
    ic_cdk::println!("Starting comprehensive GitHub analysis for: {}", username);

    // Fetch user repositories
    let repositories = fetch_user_repositories(username, access_token).await?;

    // Calculate statistics
    let (total_repos, estimated_commits, total_stars) = fetch_user_stats(username, access_token).await?;
    let total_forks: u32 = repositories.iter().map(|r| r.forks).sum();

    // Analyze languages
    let languages = analyze_languages(&repositories);

    // Calculate scores
    let commit_frequency_score = calculate_commit_frequency_score(&repositories);
    let code_quality_score = calculate_code_quality_score(&repositories);
    let community_engagement_score = calculate_community_engagement_score(&repositories);

    // Estimate account age (you'd get this from user profile in real implementation)
    let account_age_days = estimate_account_age(&repositories);

    // Estimate contributions this year
    let contributions_this_year = estimate_yearly_contributions(&repositories);

    let analysis = GitHubAnalysis {
        username: username.to_string(),
        total_repos,
        total_commits: estimated_commits,
        total_stars_received: total_stars,
        total_forks_received: total_forks,
        languages,
        repositories,
        contributions_this_year,
        account_age_days,
        followers: 0, // Would need user profile data
        following: 0, // Would need user profile data
        analyzed_at: time(),
        commit_frequency_score,
        code_quality_score,
        community_engagement_score,
    };

    ic_cdk::println!("GitHub analysis completed for: {}", username);
    Ok(analysis)
}

/// Validate GitHub username exists
pub async fn validate_github_username(username: &str) -> Result<bool, String> {
    if username.is_empty() || username.len() > 39 {
        return Ok(false);
    }

    // Basic character validation
    if !username.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return Ok(false);
    }

    let url = format!("{}/users/{}", GITHUB_API_BASE, username);

    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: Some(4096), // INCREASED from 1024 to 4096
        transform: None,
        headers: vec![
            HttpHeader {
                name: "User-Agent".to_string(),
                value: "VeriFlair-ICP-Canister".to_string(),
            },
        ],
    };

    match http_request(request, 25_000_000_000).await {
        Ok((response,)) => Ok(response.status == 200u64),
        Err(_) => Ok(false),
    }
}

// Helper Functions

fn parse_github_user(json_str: &str) -> Result<GitHubData, String> {
    let user: Value = serde_json::from_str(json_str)
        .map_err(|e| format!("Failed to parse user JSON: {}", e))?;

    Ok(GitHubData {
        login: user["login"].as_str().unwrap_or("").to_string(),
        name: user["name"].as_str().map(|s| s.to_string()),
        avatar_url: user["avatar_url"].as_str().unwrap_or("").to_string(),
        bio: user["bio"].as_str().map(|s| s.to_string()),
        public_repos: user["public_repos"].as_u64().unwrap_or(0) as u32,
        followers: user["followers"].as_u64().unwrap_or(0) as u32,
        following: user["following"].as_u64().unwrap_or(0) as u32,
        created_at: user["created_at"].as_str().unwrap_or("").to_string(),
        updated_at: user["updated_at"].as_str().unwrap_or("").to_string(),
    })
}

fn parse_repositories(json_str: &str) -> Result<Vec<Repository>, String> {
    let repos: Value = serde_json::from_str(json_str)
        .map_err(|e| format!("Failed to parse repositories JSON: {}", e))?;

    let repos_array = repos.as_array()
        .ok_or("Expected array of repositories")?;

    let mut repositories = Vec::new();

    for repo in repos_array {
        repositories.push(Repository {
            name: repo["name"].as_str().unwrap_or("").to_string(),
            full_name: repo["full_name"].as_str().unwrap_or("").to_string(),
            description: repo["description"].as_str().map(|s| s.to_string()),
            language: repo["language"].as_str().map(|s| s.to_string()),
            stars: repo["stargazers_count"].as_u64().unwrap_or(0) as u32,
            forks: repo["forks_count"].as_u64().unwrap_or(0) as u32,
            size: repo["size"].as_u64().unwrap_or(0) as u32,
            is_fork: repo["fork"].as_bool().unwrap_or(false),
            is_private: repo["private"].as_bool().unwrap_or(false),
            created_at: repo["created_at"].as_str().unwrap_or("").to_string(),
            updated_at: repo["updated_at"].as_str().unwrap_or("").to_string(),
            pushed_at: repo["pushed_at"].as_str().unwrap_or("").to_string(),
            commits_count: None, // Would need separate API call
        });
    }

    Ok(repositories)
}

fn analyze_languages(repositories: &[Repository]) -> HashMap<String, u32> {
    let mut languages = HashMap::new();

    for repo in repositories {
        if let Some(lang) = &repo.language {
            if !lang.is_empty() {
                *languages.entry(lang.clone()).or_insert(0) += repo.size;
            }
        }
    }

    languages
}

fn calculate_commit_frequency_score(repositories: &[Repository]) -> f32 {
    // Estimate based on repository activity and size
    let total_repos = repositories.len() as f32;
    let active_repos = repositories.iter()
        .filter(|r| !r.is_fork && r.size > 0)
        .count() as f32;

    if total_repos == 0.0 {
        return 0.0;
    }

    let activity_ratio = active_repos / total_repos;
    let size_factor = repositories.iter()
        .map(|r| r.size as f32)
        .sum::<f32>() / total_repos;

    (activity_ratio * 50.0 + size_factor.log10().max(0.0) * 10.0).min(100.0)
}

fn calculate_code_quality_score(repositories: &[Repository]) -> f32 {
    let total_repos = repositories.len() as f32;
    if total_repos == 0.0 {
        return 0.0;
    }

    let documented_repos = repositories.iter()
        .filter(|r| r.description.is_some() && !r.description.as_ref().unwrap().is_empty())
        .count() as f32;

    let original_repos = repositories.iter()
        .filter(|r| !r.is_fork)
        .count() as f32;

    let star_factor = repositories.iter()
        .map(|r| r.stars as f32)
        .sum::<f32>() / total_repos;

    let documentation_score = (documented_repos / total_repos) * 30.0;
    let originality_score = (original_repos / total_repos) * 40.0;
    let popularity_score = (star_factor.log10().max(0.0) * 5.0).min(30.0);

    documentation_score + originality_score + popularity_score
}

fn calculate_community_engagement_score(repositories: &[Repository]) -> f32 {
    let total_stars: u32 = repositories.iter().map(|r| r.stars).sum();
    let total_forks: u32 = repositories.iter().map(|r| r.forks).sum();

    let star_score = (total_stars as f32 * 2.0).min(60.0);
    let fork_score = (total_forks as f32 * 3.0).min(40.0);

    star_score + fork_score
}

fn estimate_commit_count(repositories: &[Repository]) -> u32 {
    // Rough estimation based on repository size, age, and activity
    repositories.iter()
        .map(|r| {
            if r.is_fork {
                return 0; // Don't count fork commits
            }

            // Estimate based on size (rough proxy for activity)
            let size_factor = (r.size as f32 / 100.0).min(50.0) as u32;
            let base_commits = if r.size > 0 { 10 } else { 0 };

            base_commits + size_factor
        })
        .sum()
}

fn estimate_account_age(repositories: &[Repository]) -> u32 {
    if repositories.is_empty() {
        return 0;
    }

    // Find oldest repository as proxy for account age
    let oldest_repo = repositories.iter()
        .filter_map(|r| parse_github_date(&r.created_at))
        .min()
        .unwrap_or_else(|| time());

    let now = time();
    let age_ns = now.saturating_sub(oldest_repo);
    let age_days = age_ns / (24 * 60 * 60 * 1_000_000_000);

    age_days as u32
}

fn estimate_yearly_contributions(repositories: &[Repository]) -> u32 {
    let current_year_repos = repositories.iter()
        .filter(|r| is_from_current_year(&r.updated_at))
        .count() as u32;

    // Rough estimation: active repos * average commits per repo
    current_year_repos * 20
}

fn parse_github_date(date_str: &str) -> Option<u64> {
    // Simple parsing for ISO 8601 dates from GitHub
    // In production, you'd use a proper date parsing library
    if date_str.len() >= 4 {
        if let Ok(year) = date_str[0..4].parse::<u32>() {
            // Very rough conversion to nanoseconds since epoch
            let years_since_1970 = year.saturating_sub(1970);
            let approx_ns = (years_since_1970 as u64) * 365 * 24 * 60 * 60 * 1_000_000_000;
            return Some(approx_ns);
        }
    }
    None
}

fn is_from_current_year(date_str: &str) -> bool {
    if date_str.len() >= 4 {
        if let Ok(year) = date_str[0..4].parse::<u32>() {
            // Rough current year check (you'd use proper date handling in production)
            return year >= 2024;
        }
    }
    false
}