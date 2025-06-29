use crate::models::{Badge, BadgeCategory, BadgeTier, BadgeMetadata, BadgeAttribute, GitHubAnalysis};
use ic_cdk::api::time;
use std::collections::HashMap;

/// Generate badges based on GitHub analysis with new Bronze/Silver/Gold tier system
pub fn generate_badges_from_analysis(analysis: &GitHubAnalysis) -> Vec<Badge> {
    let mut badges = Vec::new();
    let current_time = time();

    // Language badges based on usage and proficiency
    badges.extend(generate_language_badges(analysis, current_time));

    // Contribution badges based on activity and consistency
    badges.extend(generate_contribution_badges(analysis, current_time));

    // Achievement badges based on milestones and special accomplishments
    badges.extend(generate_achievement_badges(analysis, current_time));

    // Special badges for unique achievements
    badges.extend(generate_special_badges(analysis, current_time));

    ic_cdk::println!("Generated {} badges for user: {}", badges.len(), analysis.username);
    badges
}

/// Generate language-specific badges
fn generate_language_badges(analysis: &GitHubAnalysis, current_time: u64) -> Vec<Badge> {
    let mut badges = Vec::new();

    for (language, lines_of_code) in &analysis.languages {
        let (tier, criteria_met, score) = determine_language_tier(*lines_of_code, &analysis.repositories, language);

        if let Some(badge_tier) = tier {
            let badge_tier_clone = badge_tier.clone();
            badges.push(Badge {
                id: format!("lang_{}", language.to_lowercase()),
                name: format!("{} Expert", language),
                description: format!("Demonstrated expertise in {} programming", language),
                category: BadgeCategory::Language(language.clone()),
                tier: badge_tier,
                earned_at: current_time,
                criteria_met,
                score_achieved: score,
                metadata: BadgeMetadata {
                    image_url: format!("/badges/languages/{}.svg", language.to_lowercase()),
                    animation_url: Some(format!("/badges/languages/{}_animated.gif", language.to_lowercase())),
                    attributes: vec![
                        BadgeAttribute {
                            trait_type: "Language".to_string(),
                            value: language.clone(),
                            display_type: None,
                        },
                        BadgeAttribute {
                            trait_type: "Lines of Code".to_string(),
                            value: lines_of_code.to_string(),
                            display_type: Some("number".to_string()),
                        },
                        BadgeAttribute {
                            trait_type: "Tier".to_string(),
                            value: badge_tier_clone.get_display_name().to_string(),
                            display_type: None,
                        },
                    ],
                    rarity_score: badge_tier_clone.get_points(),
                },
            });
        }
    }

    badges
}

/// Generate contribution-based badges
fn generate_contribution_badges(analysis: &GitHubAnalysis, current_time: u64) -> Vec<Badge> {
    let mut badges = Vec::new();

    // Open Source Contributor badge
    let open_source_score = calculate_open_source_score(analysis);
    if let Some((tier, criteria)) = determine_contribution_tier(open_source_score, "open_source") {
        badges.push(create_contribution_badge(
            "open_source_contributor",
            "Open Source Contributor",
            "Active contributor to open source projects",
            BadgeCategory::Contribution("OpenSource".to_string()),
            tier,
            criteria,
            open_source_score,
            current_time,
        ));
    }

    // Community Builder badge
    let community_score = analysis.community_engagement_score as u32;
    if let Some((tier, criteria)) = determine_contribution_tier(community_score, "community") {
        badges.push(create_contribution_badge(
            "community_builder",
            "Community Builder",
            "Building and engaging with the developer community",
            BadgeCategory::Contribution("Community".to_string()),
            tier,
            criteria,
            community_score,
            current_time,
        ));
    }

    // Consistent Contributor badge
    let consistency_score = analysis.commit_frequency_score as u32;
    if let Some((tier, criteria)) = determine_contribution_tier(consistency_score, "consistency") {
        badges.push(create_contribution_badge(
            "consistent_contributor",
            "Consistent Contributor",
            "Maintaining consistent contribution patterns",
            BadgeCategory::Contribution("Consistency".to_string()),
            tier,
            criteria,
            consistency_score,
            current_time,
        ));
    }

    badges
}

/// Generate achievement badges
fn generate_achievement_badges(analysis: &GitHubAnalysis, current_time: u64) -> Vec<Badge> {
    let mut badges = Vec::new();

    // Repository Creator badge
    if let Some((tier, criteria, score)) = determine_repository_achievement(analysis.total_repos) {
        badges.push(create_achievement_badge(
            "repository_creator",
            "Repository Creator",
            "Creating and maintaining multiple repositories",
            tier,
            criteria,
            score,
            current_time,
        ));
    }

    // Commit Master badge
    if let Some((tier, criteria, score)) = determine_commit_achievement(analysis.total_commits) {
        badges.push(create_achievement_badge(
            "commit_master",
            "Commit Master",
            "Accumulating significant commit history",
            tier,
            criteria,
            score,
            current_time,
        ));
    }

    // Star Collector badge
    if let Some((tier, criteria, score)) = determine_star_achievement(analysis.total_stars_received) {
        badges.push(create_achievement_badge(
            "star_collector",
            "Star Collector",
            "Creating popular and useful projects",
            tier,
            criteria,
            score,
            current_time,
        ));
    }

    // Coding Streak badge
    if let Some((tier, criteria, score)) = determine_streak_achievement(analysis.contributions_this_year) {
        badges.push(create_achievement_badge(
            "coding_streak",
            "Coding Streak",
            "Maintaining active coding contributions",
            tier,
            criteria,
            score,
            current_time,
        ));
    }

    badges
}

/// Generate special badges for unique achievements
fn generate_special_badges(analysis: &GitHubAnalysis, current_time: u64) -> Vec<Badge> {
    let mut badges = Vec::new();

    // Early Adopter badge (account older than 5 years)
    if analysis.account_age_days > 1825 {
        badges.push(Badge {
            id: "early_adopter".to_string(),
            name: "Early Adopter".to_string(),
            description: "Been on GitHub for over 5 years".to_string(),
            category: BadgeCategory::Special("EarlyAdopter".to_string()),
            tier: BadgeTier::Gold2,
            earned_at: current_time,
            criteria_met: vec!["Account age > 5 years".to_string()],
            score_achieved: analysis.account_age_days / 365,
            metadata: create_special_badge_metadata("early_adopter", analysis.account_age_days / 365),
        });
    }

    // Polyglot badge (uses many languages)
    if analysis.languages.len() >= 5 {
        let tier = match analysis.languages.len() {
            5..=7 => BadgeTier::Bronze3,
            8..=12 => BadgeTier::Silver2,
            _ => BadgeTier::Gold1,
        };

        badges.push(Badge {
            id: "polyglot".to_string(),
            name: "Polyglot".to_string(),
            description: format!("Codes in {} different languages", analysis.languages.len()),
            category: BadgeCategory::Special("Polyglot".to_string()),
            tier,
            earned_at: current_time,
            criteria_met: vec![format!("Uses {} programming languages", analysis.languages.len())],
            score_achieved: analysis.languages.len() as u32,
            metadata: create_special_badge_metadata("polyglot", analysis.languages.len() as u32),
        });
    }

    // Innovation badge (high code quality score)
    if analysis.code_quality_score >= 80.0 {
        badges.push(Badge {
            id: "innovator".to_string(),
            name: "Innovator".to_string(),
            description: "Consistently produces high-quality code".to_string(),
            category: BadgeCategory::Special("Innovation".to_string()),
            tier: BadgeTier::Gold3,
            earned_at: current_time,
            criteria_met: vec!["Code quality score > 80".to_string()],
            score_achieved: analysis.code_quality_score as u32,
            metadata: create_special_badge_metadata("innovator", analysis.code_quality_score as u32),
        });
    }

    badges
}

// Helper functions for tier determination

fn determine_language_tier(lines_of_code: u32, repositories: &[crate::models::Repository], language: &str) -> (Option<BadgeTier>, Vec<String>, u32) {
    let repos_with_language = repositories.iter()
        .filter(|r| r.language.as_deref() == Some(language))
        .count();

    let mut criteria = Vec::new();
    let score = lines_of_code / 100; // Normalize score

    let tier = match (lines_of_code, repos_with_language) {
        (1000..=5000, 1..=2) => {
            criteria.push(format!("Written {}+ lines in {}", lines_of_code, language));
            Some(BadgeTier::Bronze1)
        },
        (5001..=15000, 2..=5) => {
            criteria.push(format!("Written {}+ lines across {} projects", lines_of_code, repos_with_language));
            Some(BadgeTier::Bronze2)
        },
        (15001..=50000, 3..=8) => {
            criteria.push(format!("Extensive {} experience across {} repositories", language, repos_with_language));
            Some(BadgeTier::Bronze3)
        },
        (50001..=100000, 5..=15) => {
            criteria.push(format!("Professional-level {} development", language));
            Some(BadgeTier::Silver1)
        },
        (100001..=250000, 8..=20) => {
            criteria.push(format!("Advanced {} expertise", language));
            Some(BadgeTier::Silver2)
        },
        (250001..=500000, 15..=30) => {
            criteria.push(format!("Expert-level {} mastery", language));
            Some(BadgeTier::Silver3)
        },
        (500001..=1000000, 20..=50) => {
            criteria.push(format!("Elite {} developer", language));
            Some(BadgeTier::Gold1)
        },
        (1000001..=2000000, 30..) => {
            criteria.push(format!("Legendary {} expertise", language));
            Some(BadgeTier::Gold2)
        },
        (2000001.., 50..) => {
            criteria.push(format!("Master {} architect", language));
            Some(BadgeTier::Gold3)
        },
        _ => None,
    };

    (tier, criteria, score)
}

fn determine_contribution_tier(score: u32, category: &str) -> Option<(BadgeTier, Vec<String>)> {
    let (tier, description) = match score {
        10..=25 => (BadgeTier::Bronze1, "Getting started"),
        26..=50 => (BadgeTier::Bronze2, "Regular contributor"),
        51..=75 => (BadgeTier::Bronze3, "Active participant"),
        76..=100 => (BadgeTier::Silver1, "Dedicated contributor"),
        101..=150 => (BadgeTier::Silver2, "Community leader"),
        151..=200 => (BadgeTier::Silver3, "Influential member"),
        201..=300 => (BadgeTier::Gold1, "Community champion"),
        301..=500 => (BadgeTier::Gold2, "Elite contributor"),
        501.. => (BadgeTier::Gold3, "Legendary figure"),
        _ => return None,
    };

    let criteria = vec![format!("{} in {} category with score {}", description, category, score)];
    Some((tier, criteria))
}

fn determine_repository_achievement(repo_count: u32) -> Option<(BadgeTier, Vec<String>, u32)> {
    let (tier, description) = match repo_count {
        5..=10 => (BadgeTier::Bronze1, "Created first repositories"),
        11..=25 => (BadgeTier::Bronze2, "Regular repository creator"),
        26..=50 => (BadgeTier::Bronze3, "Prolific project starter"),
        51..=100 => (BadgeTier::Silver1, "Dedicated builder"),
        101..=200 => (BadgeTier::Silver2, "Project factory"),
        201..=300 => (BadgeTier::Silver3, "Repository master"),
        301..=500 => (BadgeTier::Gold1, "Elite creator"),
        501..=1000 => (BadgeTier::Gold2, "Legendary builder"),
        1001.. => (BadgeTier::Gold3, "Repository titan"),
        _ => return None,
    };

    let criteria = vec![format!("{} - {} repositories created", description, repo_count)];
    Some((tier, criteria, repo_count))
}

fn determine_commit_achievement(commit_count: u32) -> Option<(BadgeTier, Vec<String>, u32)> {
    let (tier, description) = match commit_count {
        100..=500 => (BadgeTier::Bronze1, "Getting into the rhythm"),
        501..=1000 => (BadgeTier::Bronze2, "Regular committer"),
        1001..=2500 => (BadgeTier::Bronze3, "Consistent contributor"),
        2501..=5000 => (BadgeTier::Silver1, "Dedicated coder"),
        5001..=10000 => (BadgeTier::Silver2, "Prolific committer"),
        10001..=25000 => (BadgeTier::Silver3, "Commit machine"),
        25001..=50000 => (BadgeTier::Gold1, "Elite contributor"),
        50001..=100000 => (BadgeTier::Gold2, "Legendary committer"),
        100001.. => (BadgeTier::Gold3, "Commit titan"),
        _ => return None,
    };

    let criteria = vec![format!("{} - {} commits made", description, commit_count)];
    Some((tier, criteria, commit_count))
}

fn determine_star_achievement(star_count: u32) -> Option<(BadgeTier, Vec<String>, u32)> {
    let (tier, description) = match star_count {
        10..=50 => (BadgeTier::Bronze1, "Gaining recognition"),
        51..=150 => (BadgeTier::Bronze2, "Community appreciated"),
        151..=300 => (BadgeTier::Bronze3, "Popular creator"),
        301..=750 => (BadgeTier::Silver1, "Widely recognized"),
        751..=1500 => (BadgeTier::Silver2, "Highly regarded"),
        1501..=3000 => (BadgeTier::Silver3, "Community favorite"),
        3001..=7500 => (BadgeTier::Gold1, "Influential creator"),
        7501..=15000 => (BadgeTier::Gold2, "Legendary developer"),
        15001.. => (BadgeTier::Gold3, "Star magnet"),
        _ => return None,
    };

    let criteria = vec![format!("{} - {} stars received", description, star_count)];
    Some((tier, criteria, star_count))
}

fn determine_streak_achievement(contributions: u32) -> Option<(BadgeTier, Vec<String>, u32)> {
    let (tier, description) = match contributions {
        50..=100 => (BadgeTier::Bronze1, "Building momentum"),
        101..=200 => (BadgeTier::Bronze2, "Steady progress"),
        201..=365 => (BadgeTier::Bronze3, "Daily contributor"),
        366..=500 => (BadgeTier::Silver1, "Dedicated maintainer"),
        501..=750 => (BadgeTier::Silver2, "Consistent champion"),
        751..=1000 => (BadgeTier::Silver3, "Unstoppable force"),
        1001..=1500 => (BadgeTier::Gold1, "Elite contributor"),
        1501..=2000 => (BadgeTier::Gold2, "Legendary consistency"),
        2001.. => (BadgeTier::Gold3, "Ultimate dedication"),
        _ => return None,
    };

    let criteria = vec![format!("{} - {} contributions this year", description, contributions)];
    Some((tier, criteria, contributions))
}

// Helper functions for badge creation

fn create_contribution_badge(
    id: &str,
    name: &str,
    description: &str,
    category: BadgeCategory,
    tier: BadgeTier,
    criteria: Vec<String>,
    score: u32,
    earned_at: u64,
) -> Badge {
    let tier_clone = tier.clone();
    Badge {
        id: id.to_string(),
        name: name.to_string(),
        description: description.to_string(),
        category,
        tier,
        earned_at,
        criteria_met: criteria,
        score_achieved: score,
        metadata: BadgeMetadata {
            image_url: format!("/badges/contributions/{}.svg", id),
            animation_url: Some(format!("/badges/contributions/{}_animated.gif", id)),
            attributes: vec![
                BadgeAttribute {
                    trait_type: "Category".to_string(),
                    value: "Contribution".to_string(),
                    display_type: None,
                },
                BadgeAttribute {
                    trait_type: "Score".to_string(),
                    value: score.to_string(),
                    display_type: Some("number".to_string()),
                },
            ],
            rarity_score: tier_clone.get_points(),
        },
    }
}

fn create_achievement_badge(
    id: &str,
    name: &str,
    description: &str,
    tier: BadgeTier,
    criteria: Vec<String>,
    score: u32,
    earned_at: u64,
) -> Badge {
    let tier_clone = tier.clone();
    Badge {
        id: id.to_string(),
        name: name.to_string(),
        description: description.to_string(),
        category: BadgeCategory::Achievement(name.to_string()),
        tier,
        earned_at,
        criteria_met: criteria,
        score_achieved: score,
        metadata: BadgeMetadata {
            image_url: format!("/badges/achievements/{}.svg", id),
            animation_url: Some(format!("/badges/achievements/{}_animated.gif", id)),
            attributes: vec![
                BadgeAttribute {
                    trait_type: "Achievement Type".to_string(),
                    value: name.to_string(),
                    display_type: None,
                },
                BadgeAttribute {
                    trait_type: "Score".to_string(),
                    value: score.to_string(),
                    display_type: Some("number".to_string()),
                },
            ],
            rarity_score: tier_clone.get_points(),
        },
    }
}

fn create_special_badge_metadata(badge_type: &str, score: u32) -> BadgeMetadata {
    BadgeMetadata {
        image_url: format!("/badges/special/{}.svg", badge_type),
        animation_url: Some(format!("/badges/special/{}_animated.gif", badge_type)),
        attributes: vec![
            BadgeAttribute {
                trait_type: "Special Type".to_string(),
                value: badge_type.to_string(),
                display_type: None,
            },
            BadgeAttribute {
                trait_type: "Achievement Score".to_string(),
                value: score.to_string(),
                display_type: Some("number".to_string()),
            },
        ],
        rarity_score: score,
    }
}

fn calculate_open_source_score(analysis: &GitHubAnalysis) -> u32 {
    let public_repos = analysis.repositories.iter()
        .filter(|r| !r.is_private && !r.is_fork)
        .count() as u32;

    let fork_factor = analysis.total_forks_received * 2;
    let star_factor = analysis.total_stars_received;

    public_repos * 5 + fork_factor + star_factor
}

pub fn calculate_reputation_score(badges: &[Badge]) -> u64 {
    badges.iter()
        .map(|badge| badge.tier.get_points() as u64)
        .sum()
}