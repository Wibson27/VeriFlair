use crate::models::{GitHubAnalysis, GitHubData};
use crate::utils::get_developer_level;

pub async fn analyze_with_llm(github_data: &GitHubAnalysis) -> Result<String, String> {
    // For now, return a mock analysis
    // In production, this would call an LLM API

    ic_cdk::println!("Analyzing GitHub data with LLM for user: {}", github_data.username);

    let developer_level = get_developer_level(github_data.total_commits, github_data.account_age_days);
    let primary_language = get_primary_language(github_data);

    let analysis = format!(
        "🚀 Developer Analysis for {}: \
        \n\n📊 Profile Overview:\
        \n• Experience Level: {} \
        \n• GitHub Age: {} days ({:.1} years) \
        \n• Primary Languages: {} \
        \n• Total Repositories: {} \
        \n• Total Commits: {} \
        \n\n📈 Activity Metrics:\
        \n• Contributions This Year: {} \
        \n• Community Presence: {} followers, {} following \
        \n• Language Diversity: {} programming languages \
        \n\n🎯 Insights & Recommendations:\
        \n• {} shows {} activity levels with consistent contributions \
        \n• Strong focus on {} development \
        \n• {} community engagement through {} followers \
        \n• Recommended next steps: {} \
        \n\n🏆 Standout Qualities:\
        \n{}",
        github_data.username,
        developer_level,
        github_data.account_age_days,
        github_data.account_age_days as f64 / 365.25,
        github_data.languages.join(", "),
        github_data.total_repos,
        github_data.total_commits,
        github_data.contributions_this_year,
        github_data.followers,
        github_data.following,
        github_data.languages.len(),
        github_data.username,
        get_activity_level(github_data.contributions_this_year),
        primary_language,
        get_engagement_level(github_data.followers),
        github_data.followers,
        get_recommendations(github_data),
        get_standout_qualities(github_data)
    );

    Ok(analysis)
}

pub async fn analyze_github_data(github_data: &GitHubData) -> Result<String, String> {
    // Alternative function for GitHubData type
    ic_cdk::println!("Analyzing GitHub user data with LLM");

    let analysis = format!(
        "📊 GitHub Data Analysis: \
        \n• Total Repositories: {} \
        \n• Total Commits: {} \
        \n• Stars Received: {} \
        \n• Followers: {} \
        \n• Primary Languages: {} \
        \n• Activity Level: {} \
        \n• Repository Diversity: {}",
        github_data.repos,
        github_data.commits,
        github_data.stars,
        github_data.followers,
        github_data.languages.join(", "),
        if github_data.commits > 1000 { "High" } else if github_data.commits > 500 { "Moderate" } else { "Growing" },
        if github_data.repos > 20 { "Excellent" } else if github_data.repos > 10 { "Good" } else { "Building" }
    );

    Ok(analysis)
}

pub async fn generate_profile_summary(github_data: &GitHubAnalysis) -> Result<String, String> {
    let developer_level = get_developer_level(github_data.total_commits, github_data.account_age_days);
    let primary_language = github_data.languages.first()
        .map(|s| s.as_str())
        .unwrap_or("various languages");

    let summary = format!(
        "{} is a {} with {} repositories and {} total commits. \
        They have been active on GitHub for {:.1} years and primarily work with {}. \
        With {} followers and contributions in {} languages, they demonstrate {} \
        in the developer community.",
        github_data.username,
        developer_level,
        github_data.total_repos,
        github_data.total_commits,
        github_data.account_age_days as f64 / 365.25,
        primary_language,
        github_data.followers,
        github_data.languages.len(),
        if github_data.followers > 100 { "strong influence" } else { "growing presence" }
    );

    Ok(summary)
}

fn get_activity_level(contributions: u32) -> &'static str {
    match contributions {
        c if c > 500 => "exceptional",
        c if c > 300 => "high",
        c if c > 200 => "good",
        c if c > 100 => "moderate",
        _ => "growing",
    }
}

fn get_primary_language(data: &GitHubAnalysis) -> String {
    data.languages.first()
        .map(|s| s.clone())
        .unwrap_or_else(|| "multi-language".to_string())
}

fn get_engagement_level(followers: u32) -> &'static str {
    match followers {
        f if f > 500 => "Exceptional",
        f if f > 200 => "Strong",
        f if f > 100 => "Good",
        f if f > 50 => "Moderate",
        _ => "Building",
    }
}

fn get_recommendations(data: &GitHubAnalysis) -> String {
    let mut recommendations = Vec::new();

    if data.contributions_this_year < 100 {
        recommendations.push("Increase daily contribution frequency");
    }

    if data.followers < 50 {
        recommendations.push("Engage more with the open source community");
    }

    if data.total_repos < 10 {
        recommendations.push("Create more diverse projects to showcase skills");
    }

    if data.languages.len() < 3 {
        recommendations.push("Explore additional programming languages");
    }

    if recommendations.is_empty() {
        "Continue maintaining excellent development practices".to_string()
    } else {
        recommendations.join(", ")
    }
}

fn get_standout_qualities(data: &GitHubAnalysis) -> String {
    let mut qualities = Vec::new();

    if data.total_commits > 5000 {
        qualities.push("🌟 Exceptional commit history with 5000+ contributions");
    } else if data.total_commits > 1000 {
        qualities.push("⭐ Strong development track record with 1000+ commits");
    }

    if data.followers > 500 {
        qualities.push("👥 Influential community member with 500+ followers");
    } else if data.followers > 100 {
        qualities.push("🤝 Active community participant with 100+ followers");
    }

    if data.languages.len() > 5 {
        qualities.push("🔧 Polyglot programmer with 5+ languages");
    }

    if data.total_repos > 50 {
        qualities.push("📚 Prolific creator with 50+ repositories");
    }

    if data.account_age_days > 2555 { // 7+ years
        qualities.push("🏛️ Veteran developer with 7+ years on GitHub");
    }

    if qualities.is_empty() {
        "🚀 Promising developer with consistent growth trajectory".to_string()
    } else {
        qualities.join("\n• ")
    }
}