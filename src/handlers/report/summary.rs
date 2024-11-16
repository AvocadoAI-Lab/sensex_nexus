use crate::handlers::wql::GroupResponse;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, TimeZone, Timelike, FixedOffset};

#[derive(Debug, Serialize, Deserialize)]
pub struct Summary {
    pub total_alerts: usize,
    pub alerts_by_level: HashMap<i64, usize>,
    pub alerts_by_rule: HashMap<String, usize>,
    pub alerts_by_category: HashMap<String, usize>,
    pub alerts_by_mitre: HashMap<String, usize>,
    pub top_rules: Vec<(String, usize)>,
    pub agents_overview: Vec<AgentSummary>,
    pub time_analysis: TimeAnalysis,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentSummary {
    pub name: String,
    pub total_alerts: usize,
    pub highest_level: i64,
    pub alert_distribution: HashMap<i64, usize>,
    pub top_rules: Vec<(String, usize)>,
    pub categories: Vec<String>,
    pub last_alert: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeAnalysis {
    pub alerts_by_hour: HashMap<u32, usize>,
    pub alerts_by_day: HashMap<String, usize>,
    pub first_alert: Option<DateTime<Utc>>,
    pub last_alert: Option<DateTime<Utc>>,
}

pub fn generate_summary(group_response: &GroupResponse) -> Result<Summary, String> {
    let mut summary = Summary {
        total_alerts: 0,
        alerts_by_level: HashMap::new(),
        alerts_by_rule: HashMap::new(),
        alerts_by_category: HashMap::new(),
        alerts_by_mitre: HashMap::new(),
        top_rules: Vec::new(),
        agents_overview: Vec::new(),
        time_analysis: TimeAnalysis {
            alerts_by_hour: HashMap::new(),
            alerts_by_day: HashMap::new(),
            first_alert: None,
            last_alert: None,
        },
    };

    for agent_result in &group_response.results {
        let mut agent_summary = AgentSummary {
            name: agent_result.agent_name.clone(),
            total_alerts: 0,
            highest_level: 0,
            alert_distribution: HashMap::new(),
            top_rules: Vec::new(),
            categories: Vec::new(),
            last_alert: None,
        };

        if let Some(hits) = agent_result.data.get("hits") {
            if let Some(hits_array) = hits.get("hits").and_then(|h| h.as_array()) {
                for hit in hits_array {
                    agent_summary.total_alerts += 1;
                    summary.total_alerts += 1;

                    if let Some(source) = hit.get("_source") {
                        // Process timestamp
                        if let Some(timestamp) = source.get("@timestamp").and_then(|t| t.as_str()) {
                            if let Ok(dt) = DateTime::parse_from_rfc3339(timestamp) {
                                let utc_dt = dt.with_timezone(&Utc);
                                let hour = dt.with_timezone(&Utc).hour();
                                
                                // Update time analysis
                                *summary.time_analysis.alerts_by_hour.entry(hour).or_insert(0) += 1;
                                *summary.time_analysis.alerts_by_day.entry(dt.format("%Y-%m-%d").to_string()).or_insert(0) += 1;
                                
                                if summary.time_analysis.first_alert.is_none() || 
                                   summary.time_analysis.first_alert.unwrap() > utc_dt {
                                    summary.time_analysis.first_alert = Some(utc_dt);
                                }
                                
                                if summary.time_analysis.last_alert.is_none() || 
                                   summary.time_analysis.last_alert.unwrap() < utc_dt {
                                    summary.time_analysis.last_alert = Some(utc_dt);
                                }

                                // Update agent's last alert
                                if agent_summary.last_alert.is_none() || 
                                   agent_summary.last_alert.unwrap() < utc_dt {
                                    agent_summary.last_alert = Some(utc_dt);
                                }
                            }
                        }

                        // Process rule information
                        if let Some(rule) = source.get("rule") {
                            // Track alert level
                            if let Some(level) = rule.get("level").and_then(|l| l.as_i64()) {
                                *summary.alerts_by_level.entry(level).or_insert(0) += 1;
                                *agent_summary.alert_distribution.entry(level).or_insert(0) += 1;
                                agent_summary.highest_level = agent_summary.highest_level.max(level);
                            }

                            // Track rule description
                            if let Some(desc) = rule.get("description").and_then(|d| d.as_str()) {
                                *summary.alerts_by_rule.entry(desc.to_string()).or_insert(0) += 1;
                            }

                            // Track rule groups/categories
                            if let Some(groups) = rule.get("groups").and_then(|g| g.as_array()) {
                                for group in groups {
                                    if let Some(group_str) = group.as_str() {
                                        *summary.alerts_by_category.entry(group_str.to_string()).or_insert(0) += 1;
                                        if !agent_summary.categories.contains(&group_str.to_string()) {
                                            agent_summary.categories.push(group_str.to_string());
                                        }
                                    }
                                }
                            }

                            // Track MITRE ATT&CK information
                            if let Some(mitre) = rule.get("mitre") {
                                if let Some(tactics) = mitre.get("tactic").and_then(|t| t.as_array()) {
                                    for tactic in tactics {
                                        if let Some(tactic_str) = tactic.as_str() {
                                            *summary.alerts_by_mitre.entry(tactic_str.to_string()).or_insert(0) += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Generate agent's top rules
        let mut agent_rules: Vec<(String, usize)> = agent_summary.alert_distribution.iter()
            .map(|(k, v)| (k.to_string(), *v))
            .collect();
        agent_rules.sort_by(|a, b| b.1.cmp(&a.1));
        agent_summary.top_rules = agent_rules.into_iter().take(5).collect();

        summary.agents_overview.push(agent_summary);
    }

    // Generate overall top rules
    let mut rules: Vec<(String, usize)> = summary.alerts_by_rule.iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    rules.sort_by(|a, b| b.1.cmp(&a.1));
    summary.top_rules = rules.into_iter().take(5).collect();

    Ok(summary)
}
