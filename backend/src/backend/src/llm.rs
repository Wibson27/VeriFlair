use crate::models::{GitHubAnalysis, Repository};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use candid::CandidType;

// LLM Analysis Results
#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct LLMAnalysis {
    pub code_quality_score: f32,
    pub contribution_consistency: f32,
    pub community_impact: f32,
    pub technical_breadth: f32,
    pub innovation_score: f32,
    pub expertise_areas: Vec<String>,
    pub recommended_badges: Vec<String>,
    pub analysis_summary: String,
    pub strengths: Vec<String>,
    pub improvement_areas: Vec<String>,
}

// Azure OpenAI Configuration
thread_local! {
    static AZURE_API_KEY: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
    static AZURE_ENDPOINT: std::cell::RefCell<String> = std::cell::RefCell::new(String::new());
    static AZURE_DEPLOYMENT: std::cell::RefCell<String> = std::cell::RefCell::new("gpt-35-turbo".to_string());
    static AZURE_API_VERSION: std::cell::RefCell<String> = std::cell::RefCell::new("2025-01-01-preview".to_string());
}

pub fn set_azure_openai_config(
    api_key: String,
    endpoint: String,
    deployment: Option<String>,
    api_version: Option<String>
) {
    AZURE_API_KEY.with(|key| *key.borrow_mut() = Some(api_key));
    AZURE_ENDPOINT.with(|ep| *ep.borrow_mut() = endpoint);

    if let Some(dep) = deployment {
        AZURE_DEPLOYMENT.with(|d| *d.borrow_mut() = dep);
    }

    if let Some(ver) = api_version {
        AZURE_API_VERSION.with(|v| *v.borrow_mut() = ver);
    }
}

// Legacy function for backwards compatibility
pub fn set_llm_config(api_key: String, api_url: Option<String>) {
    if let Some(url) = api_url {
        if url.contains("azure") {
            // Extract Azure details from URL if possible
            AZURE_ENDPOINT.with(|ep| *ep.borrow_mut() = url);
        }
    }
    AZURE_API_KEY.with(|key| *key.borrow_mut() = Some(api_key));
}

/// Analyze code quality using Azure OpenAI
pub async fn analyze_code_quality(analysis: &GitHubAnalysis) -> Result<LLMAnalysis, String> {
    // Check if Azure OpenAI is configured
    if let Some(api_key) = AZURE_API_KEY.with(|key| key.borrow().clone()) {
        let endpoint = AZURE_ENDPOINT.with(|ep| ep.borrow().clone());
        if !endpoint.is_empty() {
            match call_azure_openai_api(analysis, &api_key, &endpoint).await {
                Ok(llm_result) => return Ok(llm_result),
                Err(e) => {
                    ic_cdk::println!("Azure OpenAI API call failed, using fallback analysis: {}", e);
                    // Continue to fallback analysis
                }
            }
        }
    }

    // Fallback: Create analysis based on GitHub metrics
    Ok(create_fallback_analysis(analysis))
}

/// Call Azure OpenAI API for enhanced analysis
async fn call_azure_openai_api(analysis: &GitHubAnalysis, api_key: &str, endpoint: &str) -> Result<LLMAnalysis, String> {
    let prompt = create_analysis_prompt(analysis);

    let request_body = serde_json::json!({
        "messages": [
            {
                "role": "system",
                "content": "You are an expert code reviewer and developer analyst. Analyze the GitHub profile data and provide insights about the developer's skills, contributions, and expertise areas. Respond with valid JSON only."
            },
            {
                "role": "user",
                "content": prompt
            }
        ],
        "max_tokens": 1000,
        "temperature": 0.3,
        "stream": false
    });

    // Construct Azure OpenAI URL
    let deployment = AZURE_DEPLOYMENT.with(|d| d.borrow().clone());
    let api_version = AZURE_API_VERSION.with(|v| v.borrow().clone());

    let url = format!(
        "{}/openai/deployments/{}/chat/completions?api-version={}",
        endpoint, deployment, api_version
    );

    ic_cdk::println!("Calling Azure OpenAI at: {}", url);

    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::POST,
        body: Some(request_body.to_string().into_bytes()),
        max_response_bytes: Some(4096),
        transform: None,
        headers: vec![
            HttpHeader {
                name: "api-key".to_string(), // Azure uses 'api-key' instead of 'Authorization'
                value: api_key.to_string(),
            },
            HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            },
        ],
    };

    match http_request(request, 25_000_000_000).await {
        Ok((response,)) => {
            if response.status == 200u64 {
                let body_str = String::from_utf8(response.body)
                    .map_err(|e| format!("Failed to parse response body: {}", e))?;

                ic_cdk::println!("Azure OpenAI response: {}", body_str);
                parse_azure_openai_response(&body_str)
            } else {
                let error_body = String::from_utf8(response.body).unwrap_or_default();
                Err(format!("Azure OpenAI API request failed with status: {} - {}", response.status, error_body))
            }
        }
        Err((r, m)) => Err(format!("HTTP request failed: {:?} - {}", r, m)),
    }
}

/// Create analysis prompt for LLM
fn create_analysis_prompt(analysis: &GitHubAnalysis) -> String {
    format!(
        "Analyze this GitHub developer profile:

Username: {}
Total Repositories: {}
Total Commits: {}
Stars Received: {}
Forks Received: {}
Account Age: {} days
Languages Used: {}
Commit Frequency Score: {}
Code Quality Score: {}
Community Engagement Score: {}

Top 5 Repositories:
{}

Please provide:
1. Code quality assessment (0-100)
2. Contribution consistency (0-100)
3. Community impact (0-100)
4. Technical breadth (0-100)
5. Innovation score (0-100)
6. Top 3 expertise areas
7. Recommended badge categories
8. Brief analysis summary
9. Key strengths
10. Areas for improvement

Format as JSON with these exact field names: code_quality_score, contribution_consistency, community_impact, technical_breadth, innovation_score, expertise_areas, recommended_badges, analysis_summary, strengths, improvement_areas",
        analysis.username,
        analysis.total_repos,
        analysis.total_commits,
        analysis.total_stars_received,
        analysis.total_forks_received,
        analysis.account_age_days,
        analysis.languages.len(),
        analysis.commit_frequency_score,
        analysis.code_quality_score,
        analysis.community_engagement_score,
        analysis.repositories
            .iter()
            .take(5)
            .map(|r| format!("- {} ({}★, {} forks)", r.name, r.stars, r.forks))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

/// Parse Azure OpenAI API response
fn parse_azure_openai_response(response: &str) -> Result<LLMAnalysis, String> {
    let response_json: Value = serde_json::from_str(response)
        .map_err(|e| format!("Failed to parse Azure OpenAI response JSON: {}", e))?;

    let content = response_json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("No content in Azure OpenAI response")?;

    // Try to parse as JSON first
    if let Ok(analysis_json) = serde_json::from_str::<Value>(content) {
        Ok(LLMAnalysis {
            code_quality_score: analysis_json["code_quality_score"].as_f64().unwrap_or(0.0) as f32,
            contribution_consistency: analysis_json["contribution_consistency"].as_f64().unwrap_or(0.0) as f32,
            community_impact: analysis_json["community_impact"].as_f64().unwrap_or(0.0) as f32,
            technical_breadth: analysis_json["technical_breadth"].as_f64().unwrap_or(0.0) as f32,
            innovation_score: analysis_json["innovation_score"].as_f64().unwrap_or(0.0) as f32,
            expertise_areas: analysis_json["expertise_areas"]
                .as_array()
                .unwrap_or(&Vec::new())
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect(),
            recommended_badges: analysis_json["recommended_badges"]
                .as_array()
                .unwrap_or(&Vec::new())
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect(),
            analysis_summary: analysis_json["analysis_summary"]
                .as_str()
                .unwrap_or("Azure OpenAI analysis completed")
                .to_string(),
            strengths: analysis_json["strengths"]
                .as_array()
                .unwrap_or(&Vec::new())
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect(),
            improvement_areas: analysis_json["improvement_areas"]
                .as_array()
                .unwrap_or(&Vec::new())
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect(),
        })
    } else {
        // If not valid JSON, create a simple analysis from the text
        Ok(LLMAnalysis {
            code_quality_score: 75.0,
            contribution_consistency: 70.0,
            community_impact: 65.0,
            technical_breadth: 80.0,
            innovation_score: 70.0,
            expertise_areas: vec!["General Development".to_string()],
            recommended_badges: vec!["Developer".to_string()],
            analysis_summary: content.chars().take(200).collect::<String>(),
            strengths: vec!["Active contributor".to_string()],
            improvement_areas: vec!["Expand skill diversity".to_string()],
        })
    }
}

/// Legacy parse function for backwards compatibility
fn parse_llm_response(response: &str) -> Result<LLMAnalysis, String> {
    parse_azure_openai_response(response)
}

/// Create fallback analysis when LLM is not available
fn create_fallback_analysis(analysis: &GitHubAnalysis) -> LLMAnalysis {
    let mut expertise_areas = Vec::new();
    let mut recommended_badges = Vec::new();
    let mut strengths = Vec::new();
    let mut improvement_areas = Vec::new();

    // Determine expertise areas based on languages and activity
    if !analysis.languages.is_empty() {
        let top_language = analysis.languages
            .iter()
            .max_by_key(|(_, &usage)| usage)
            .map(|(lang, _)| lang.clone())
            .unwrap_or_default();

        if !top_language.is_empty() {
            expertise_areas.push(format!("{} Development", top_language));
        }
    }

    if analysis.total_repos > 50 {
        expertise_areas.push("Project Architecture".to_string());
    }

    if analysis.community_engagement_score > 70.0 {
        expertise_areas.push("Open Source Leadership".to_string());
    }

    // Recommend badges based on metrics
    if analysis.total_commits > 1000 {
        recommended_badges.push("Commit Master".to_string());
    }

    if analysis.total_stars_received > 100 {
        recommended_badges.push("Star Collector".to_string());
    }

    if analysis.languages.len() >= 5 {
        recommended_badges.push("Polyglot".to_string());
    }

    // Determine strengths
    if analysis.commit_frequency_score > 80.0 {
        strengths.push("Consistent contribution pattern".to_string());
    }

    if analysis.code_quality_score > 75.0 {
        strengths.push("High-quality code practices".to_string());
    }

    if analysis.total_repos > 20 {
        strengths.push("Prolific project creator".to_string());
    }

    // Suggest improvements
    if analysis.commit_frequency_score < 50.0 {
        improvement_areas.push("Increase contribution consistency".to_string());
    }

    if analysis.community_engagement_score < 30.0 {
        improvement_areas.push("Engage more with the community".to_string());
    }

    if analysis.languages.len() < 3 {
        improvement_areas.push("Explore additional programming languages".to_string());
    }

    let analysis_summary = format!(
        "Developer with {} repositories and {} commits. Shows {} in {} languages with {} community engagement.",
        analysis.total_repos,
        analysis.total_commits,
        if analysis.code_quality_score > 70.0 { "strong" } else { "moderate" },
        analysis.languages.len(),
        if analysis.community_engagement_score > 50.0 { "good" } else { "limited" }
    );

    LLMAnalysis {
        code_quality_score: analysis.code_quality_score,
        contribution_consistency: analysis.commit_frequency_score,
        community_impact: analysis.community_engagement_score,
        technical_breadth: (analysis.languages.len() as f32 * 10.0).min(100.0),
        innovation_score: (analysis.total_stars_received as f32 / 10.0).min(100.0),
        expertise_areas,
        recommended_badges,
        analysis_summary,
        strengths,
        improvement_areas,
    }
}

/// Enhanced repository analysis
pub async fn analyze_repository_quality(repos: &[Repository]) -> Result<f32, String> {
    let mut quality_score = 0.0;
    let total_repos = repos.len() as f32;

    if total_repos == 0.0 {
        return Ok(0.0);
    }

    for repo in repos {
        let mut repo_score = 0.0;

        // Documentation score
        if repo.description.is_some() && !repo.description.as_ref().unwrap().is_empty() {
            repo_score += 20.0;
        }

        // Popularity score
        repo_score += (repo.stars as f32 * 2.0).min(30.0);
        repo_score += (repo.forks as f32 * 3.0).min(20.0);

        // Activity score (based on size as proxy)
        if repo.size > 100 {
            repo_score += 15.0;
        }

        // Originality bonus
        if !repo.is_fork {
            repo_score += 15.0;
        }

        quality_score += repo_score.min(100.0);
    }

    Ok(quality_score / total_repos)
}

/// Analyze contribution patterns
pub async fn analyze_contribution_patterns(analysis: &GitHubAnalysis) -> Result<f32, String> {
    let mut pattern_score = 0.0;

    // Consistency score based on frequency
    pattern_score += analysis.commit_frequency_score * 0.4;

    // Volume score
    let commit_volume_score = if analysis.total_commits > 5000 {
        100.0
    } else if analysis.total_commits > 1000 {
        80.0
    } else if analysis.total_commits > 500 {
        60.0
    } else if analysis.total_commits > 100 {
        40.0
    } else {
        20.0
    };
    pattern_score += commit_volume_score * 0.3;

    // Diversity score (languages and repositories)
    let diversity_score = (analysis.languages.len() as f32 * 5.0).min(50.0);
    pattern_score += diversity_score * 0.3;

    Ok(pattern_score.min(100.0))
}