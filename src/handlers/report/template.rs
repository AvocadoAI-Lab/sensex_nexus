use crate::handlers::wql::GroupResponse;
use super::summary::Summary;

pub fn generate_html(group_response: &GroupResponse, summary: &Summary) -> Result<String, String> {
    let mut html = format!(r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Security Report - Group {}</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            line-height: 1.6;
            margin: 0;
            padding: 20px;
            color: #333;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
        }}
        .header {{
            background-color: #f8f9fa;
            padding: 20px;
            border-radius: 5px;
            margin-bottom: 20px;
        }}
        .section {{
            background-color: white;
            padding: 20px;
            border-radius: 5px;
            margin-bottom: 20px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }}
        .alert-level {{
            display: inline-block;
            padding: 5px 10px;
            border-radius: 3px;
            color: white;
            margin: 2px;
        }}
        .level-high {{
            background-color: #dc3545;
        }}
        .level-medium {{
            background-color: #ffc107;
            color: #000;
        }}
        .level-low {{
            background-color: #28a745;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
            margin-top: 10px;
        }}
        th, td {{
            padding: 12px;
            text-align: left;
            border-bottom: 1px solid #ddd;
        }}
        th {{
            background-color: #f8f9fa;
        }}
        .agent-card {{
            background-color: white;
            border-radius: 5px;
            padding: 15px;
            margin-bottom: 15px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }}
        .agent-header {{
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 10px;
        }}
        .timestamp {{
            color: #666;
            font-size: 0.9em;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>Security Report - Group {}</h1>
            <p>Total Alerts: {}</p>
            <p class="timestamp">Generated: {}</p>
        </div>
"#, group_response.group, group_response.group, summary.total_alerts, 
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));

    // Summary Section
    html.push_str(r#"
        <div class="section">
            <h2>Summary</h2>
            <h3>Alert Levels Distribution</h3>
            <table>
                <tr>
                    <th>Level</th>
                    <th>Count</th>
                    <th>Percentage</th>
                </tr>
"#);

    let mut levels: Vec<_> = summary.alerts_by_level.iter().collect();
    levels.sort_by_key(|&(k, _)| k);
    for (level, count) in levels {
        let percentage = (*count as f64 / summary.total_alerts as f64 * 100.0).round();
        let level_class = match level {
            0..=4 => "level-low",
            5..=7 => "level-medium",
            _ => "level-high",
        };
        html.push_str(&format!(r#"
                <tr>
                    <td><span class="alert-level {}">{}</span></td>
                    <td>{}</td>
                    <td>{}%</td>
                </tr>
"#, level_class, level, count, percentage));
    }

    html.push_str(r#"
            </table>

            <h3>Top 5 Rules</h3>
            <table>
                <tr>
                    <th>Rule Description</th>
                    <th>Count</th>
                    <th>Percentage</th>
                </tr>
"#);

    for (rule, count) in &summary.top_rules {
        let percentage = (*count as f64 / summary.total_alerts as f64 * 100.0).round();
        html.push_str(&format!(r#"
                <tr>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}%</td>
                </tr>
"#, rule, count, percentage));
    }

    html.push_str("</table></div>");

    // Agents Overview Section
    html.push_str(r#"
        <div class="section">
            <h2>Agents Overview</h2>
"#);

    for agent in &summary.agents_overview {
        let level_class = match agent.highest_level {
            0..=4 => "level-low",
            5..=7 => "level-medium",
            _ => "level-high",
        };
        
        html.push_str(&format!(r#"
            <div class="agent-card">
                <div class="agent-header">
                    <h3>{}</h3>
                    <span class="alert-level {}">Level {}</span>
                </div>
                <p>Total Alerts: {}</p>
            </div>
"#, agent.name, level_class, agent.highest_level, agent.total_alerts));
    }

    html.push_str("</div></div></body></html>");

    Ok(html)
}
