use crate::handlers::wql::GroupResponse;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Summary {
    pub total_alerts: usize,
    pub alerts_by_level: HashMap<i64, usize>,
    pub alerts_by_rule: HashMap<String, usize>,
    pub top_rules: Vec<(String, usize)>,
    pub agents_overview: Vec<AgentSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentSummary {
    pub name: String,
    pub total_alerts: usize,
    pub highest_level: i64,
}

pub fn generate_summary(group_response: &GroupResponse) -> Result<Summary, String> {
    let mut summary = Summary {
        total_alerts: 0,
        alerts_by_level: HashMap::new(),
        alerts_by_rule: HashMap::new(),
        top_rules: Vec::new(),
        agents_overview: Vec::new(),
    };

    for agent_result in &group_response.results {
        let mut agent_total = 0;
        let mut agent_highest_level = 0;

        if let Some(hits) = agent_result.data.get("hits") {
            if let Some(hits_array) = hits.get("hits").and_then(|h| h.as_array()) {
                for hit in hits_array {
                    agent_total += 1;
                    summary.total_alerts += 1;

                    if let Some(source) = hit.get("_source") {
                        // Process rule information
                        if let Some(rule) = source.get("rule") {
                            // Track alert level
                            if let Some(level) = rule.get("level").and_then(|l| l.as_i64()) {
                                *summary.alerts_by_level.entry(level).or_insert(0) += 1;
                                agent_highest_level = agent_highest_level.max(level);
                            }

                            // Track rule description
                            if let Some(desc) = rule.get("description").and_then(|d| d.as_str()) {
                                *summary.alerts_by_rule.entry(desc.to_string()).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }

        // Add agent summary
        summary.agents_overview.push(AgentSummary {
            name: agent_result.agent_name.clone(),
            total_alerts: agent_total,
            highest_level: agent_highest_level,
        });
    }

    // Generate top rules
    let mut rules: Vec<(String, usize)> = summary.alerts_by_rule.iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    rules.sort_by(|a, b| b.1.cmp(&a.1));
    summary.top_rules = rules.into_iter().take(5).collect();

    Ok(summary)
}
