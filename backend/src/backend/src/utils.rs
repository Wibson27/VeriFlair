use crate::models::{Badge, BadgeRarity, GitHubAnalysis};
use ic_cdk::api::time;

pub fn generate_badges_from_analysis(analysis: &GitHubAnalysis) -> Vec<Badge> {
    let mut badges = Vec::new();
    let current_time = time();

    // Early Adopter badge - account older than 5 years
    if analysis.account_age_days > 1825 {
        badges.push(Badge {
            id: "early_adopter".to_string(),
            name: "Early Adopter".to_string(),
            description: "GitHub account older than 5 years".to_string(),
            earned_at: current_time,
            rarity: BadgeRarity::Rare,
            criteria: "Account age > 5 years".to_string(),
        });
    }

    // Prolific Coder badge - more than 1000 commits
    if analysis.total_commits > 1000 {
        badges.push(Badge {
            id: "prolific_coder".to_string(),
            name: "Prolific Coder".to_string(),
            description: "Made over 1000 commits".to_string(),
            earned_at: current_time,
            rarity: BadgeRarity::Epic,
            criteria: "Total commits > 1000".to_string(),
        });
    }

    // Repository Creator badge - more than 10 repos
    if analysis.total_repos > 10 {
        badges.push(Badge {
            id: "repo_creator".to_string(),
            name: "Repository Creator".to_string(),
            description: "Created more than 10 repositories".to_string(),
            earned_at: current_time,
            rarity: BadgeRarity::Uncommon,
            criteria: "Total repos > 10".to_string(),
        });
    }

    // Social Developer badge - more than 100 followers
    if analysis.followers > 100 {
        badges.push(Badge {
            id: "social_dev".to_string(),
            name: "Social Developer".to_string(),
            description: "Has over 100 followers".to_string(),
            earned_at: current_time,
            rarity: BadgeRarity::Rare,
            criteria: "Followers > 100".to_string(),
        });
    }

    // Polyglot badge - knows more than 5 languages
    if analysis.languages.len() > 5 {
        badges.push(Badge {
            id: "polyglot".to_string(),
            name: "Polyglot".to_string(),
            description: "Codes in more than 5 languages".to_string(),
            earned_at: current_time,
            rarity: BadgeRarity::Epic,
            criteria: "Languages > 5".to_string(),
        });
    }

    // Active Contributor badge - more than 200 contributions this year
    if analysis.contributions_this_year > 200 {
        badges.push(Badge {
            id: "active_contributor".to_string(),
            name: "Active Contributor".to_string(),
            description: "Made over 200 contributions this year".to_string(),
            earned_at: current_time,
            rarity: BadgeRarity::Uncommon,
            criteria: "Contributions this year > 200".to_string(),
        });
    }

    // Networking Expert badge - more than 500 followers
    if analysis.followers > 500 {
        badges.push(Badge {
            id: "networking_expert".to_string(),
            name: "Networking Expert".to_string(),
            description: "Has over 500 followers".to_string(),
            earned_at: current_time,
            rarity: BadgeRarity::Legendary,
            criteria: "Followers > 500".to_string(),
        });
    }

    // Repository Powerhouse badge - more than 50 repos
    if analysis.total_repos > 50 {
        badges.push(Badge {
            id: "repo_powerhouse".to_string(),
            name: "Repository Powerhouse".to_string(),
            description: "Created more than 50 repositories".to_string(),
            earned_at: current_time,
            rarity: BadgeRarity::Epic,
            criteria: "Total repos > 50".to_string(),
        });
    }

    // Commit Master badge - more than 5000 commits
    if analysis.total_commits > 5000 {
        badges.push(Badge {
            id: "commit_master".to_string(),
            name: "Commit Master".to_string(),
            description: "Made over 5000 commits".to_string(),
            earned_at: current_time,
            rarity: BadgeRarity::Legendary,
            criteria: "Total commits > 5000".to_string(),
        });
    }

    badges
}

pub fn calculate_reputation_score(badges: &Vec<Badge>) -> u64 {
    let mut score = 0u64;

    for badge in badges {
        let badge_points = match badge.rarity {
            BadgeRarity::Common => 10,
            BadgeRarity::Uncommon => 25,
            BadgeRarity::Rare => 50,
            BadgeRarity::Epic => 100,
            BadgeRarity::Legendary => 250,
        };
        score += badge_points;
    }

    score
}

pub fn format_timestamp(timestamp: u64) -> String {
    // Convert nanoseconds to seconds
    let seconds = timestamp / 1_000_000_000;
    format!("Timestamp: {}", seconds)
}

pub fn get_developer_level(total_commits: u32, account_age_days: u32) -> String {
    match (total_commits, account_age_days) {
        (commits, age) if commits > 5000 && age > 2555 => "Elite Developer".to_string(),
        (commits, age) if commits > 1000 && age > 1825 => "Senior Developer".to_string(),
        (commits, age) if commits > 500 && age > 730 => "Mid-level Developer".to_string(),
        (commits, age) if commits > 100 && age > 365 => "Junior Developer".to_string(),
        _ => "Beginner Developer".to_string(),
    }
}

pub fn calculate_activity_score(contributions_this_year: u32) -> u32 {
    match contributions_this_year {
        contrib if contrib > 1000 => 100, // Max activity score
        contrib if contrib > 500 => 80,
        contrib if contrib > 200 => 60,
        contrib if contrib > 100 => 40,
        contrib if contrib > 50 => 20,
        _ => 10, // Minimum activity score
    }
}