import json
import sys
from datetime import datetime
import matplotlib.pyplot as plt
import seaborn as sns
from reportlab.lib import colors
from reportlab.lib.pagesizes import A4
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Image, Table, TableStyle
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib.units import inch
import os
import matplotlib
matplotlib.use('Agg')

def create_time_series_chart(data, output_path):
    plt.figure(figsize=(10, 6))
    sns.set_style("whitegrid")
    
    hours = list(data['time_analysis']['alerts_by_hour'].keys())
    counts = list(data['time_analysis']['alerts_by_hour'].values())
    
    plt.plot(hours, counts, marker='o')
    plt.title('Alerts by Hour', pad=20)
    plt.xlabel('Hour of Day')
    plt.ylabel('Number of Alerts')
    
    plt.tight_layout()
    plt.savefig(output_path)
    plt.close()

def create_severity_chart(data, output_path):
    plt.figure(figsize=(8, 8))
    
    levels = list(data['alerts_by_level'].keys())
    counts = list(data['alerts_by_level'].values())
    colors = ['#16a34a', '#d97706', '#dc2626']
    
    plt.pie(counts, labels=[f'Level {l}' for l in levels], 
            autopct='%1.1f%%', colors=colors)
    plt.title('Alert Severity Distribution')
    
    plt.tight_layout()
    plt.savefig(output_path)
    plt.close()

def create_mitre_chart(data, output_path):
    if not data['alerts_by_mitre']:
        return None
        
    plt.figure(figsize=(12, 6))
    
    tactics = list(data['alerts_by_mitre'].keys())
    counts = list(data['alerts_by_mitre'].values())
    
    plt.bar(tactics, counts, color='#9c27b0')
    plt.title('MITRE ATT&CK Distribution')
    plt.xticks(rotation=45, ha='right')
    plt.ylabel('Number of Alerts')
    
    plt.tight_layout()
    plt.savefig(output_path)
    plt.close()

def generate_pdf(data, output_path):
    # Create directory for charts
    charts_dir = os.path.join(os.path.dirname(output_path), 'charts')
    os.makedirs(charts_dir, exist_ok=True)
    
    # Generate charts
    time_series_path = os.path.join(charts_dir, 'time_series.png')
    severity_path = os.path.join(charts_dir, 'severity.png')
    mitre_path = os.path.join(charts_dir, 'mitre.png')
    
    create_time_series_chart(data, time_series_path)
    create_severity_chart(data, severity_path)
    create_mitre_chart(data, mitre_path)
    
    # Create PDF
    doc = SimpleDocTemplate(
        output_path,
        pagesize=A4,
        rightMargin=72,
        leftMargin=72,
        topMargin=72,
        bottomMargin=72
    )
    
    # Styles
    styles = getSampleStyleSheet()
    title_style = ParagraphStyle(
        'CustomTitle',
        parent=styles['Heading1'],
        fontSize=24,
        spaceAfter=30
    )
    heading_style = ParagraphStyle(
        'CustomHeading',
        parent=styles['Heading2'],
        fontSize=16,
        spaceAfter=12
    )
    normal_style = styles['Normal']
    
    # Build content
    story = []
    
    # Title
    story.append(Paragraph(f"Security Analysis Report", title_style))
    story.append(Paragraph(f"Group: {data['group']}", heading_style))
    story.append(Spacer(1, 20))
    
    # Executive Summary
    story.append(Paragraph("Executive Summary", heading_style))
    story.append(Paragraph(f"Total Alerts: {data['total_alerts']}", normal_style))
    story.append(Spacer(1, 12))
    
    # Time Analysis
    story.append(Paragraph("Time Analysis", heading_style))
    first_alert = data['time_analysis']['first_alert']
    last_alert = data['time_analysis']['last_alert']
    story.append(Paragraph(f"First Alert: {first_alert}", normal_style))
    story.append(Paragraph(f"Last Alert: {last_alert}", normal_style))
    story.append(Spacer(1, 12))
    
    # Add time series chart
    story.append(Image(time_series_path, width=6*inch, height=4*inch))
    story.append(Spacer(1, 20))
    
    # Severity Distribution
    story.append(Paragraph("Alert Severity Distribution", heading_style))
    story.append(Image(severity_path, width=5*inch, height=5*inch))
    story.append(Spacer(1, 20))
    
    # MITRE ATT&CK Analysis
    if os.path.exists(mitre_path):
        story.append(Paragraph("MITRE ATT&CK Analysis", heading_style))
        story.append(Image(mitre_path, width=6*inch, height=4*inch))
        story.append(Spacer(1, 20))
    
    # Categories
    story.append(Paragraph("Top Alert Categories", heading_style))
    categories = sorted(data['alerts_by_category'].items(), 
                       key=lambda x: x[1], reverse=True)[:5]
    
    cat_data = [[Paragraph("Category", heading_style), 
                 Paragraph("Count", heading_style), 
                 Paragraph("Percentage", heading_style)]]
    
    for cat, count in categories:
        percentage = (count / data['total_alerts']) * 100
        cat_data.append([
            Paragraph(cat, normal_style),
            Paragraph(str(count), normal_style),
            Paragraph(f"{percentage:.1f}%", normal_style)
        ])
    
    table = Table(cat_data, colWidths=[3*inch, 1.5*inch, 1.5*inch])
    table.setStyle(TableStyle([
        ('BACKGROUND', (0, 0), (-1, 0), colors.grey),
        ('TEXTCOLOR', (0, 0), (-1, 0), colors.whitesmoke),
        ('ALIGN', (0, 0), (-1, -1), 'CENTER'),
        ('FONTNAME', (0, 0), (-1, 0), 'Helvetica-Bold'),
        ('FONTSIZE', (0, 0), (-1, 0), 14),
        ('BOTTOMPADDING', (0, 0), (-1, 0), 12),
        ('BACKGROUND', (0, 1), (-1, -1), colors.white),
        ('TEXTCOLOR', (0, 1), (-1, -1), colors.black),
        ('FONTNAME', (0, 1), (-1, -1), 'Helvetica'),
        ('FONTSIZE', (0, 1), (-1, -1), 12),
        ('ALIGN', (0, 0), (-1, -1), 'CENTER'),
        ('GRID', (0, 0), (-1, -1), 1, colors.black)
    ]))
    story.append(table)
    story.append(Spacer(1, 20))
    
    # Agent Analysis
    story.append(Paragraph("Agent Analysis", heading_style))
    for agent in data['agents_overview']:
        story.append(Paragraph(f"Agent: {agent['name']}", heading_style))
        agent_data = [
            [Paragraph("Total Alerts", normal_style), 
             Paragraph(str(agent['total_alerts']), normal_style)],
            [Paragraph("Highest Level", normal_style), 
             Paragraph(str(agent['highest_level']), normal_style)],
            [Paragraph("Last Alert", normal_style), 
             Paragraph(str(agent['last_alert']), normal_style)]
        ]
        
        agent_table = Table(agent_data, colWidths=[2*inch, 4*inch])
        agent_table.setStyle(TableStyle([
            ('ALIGN', (0, 0), (-1, -1), 'LEFT'),
            ('FONTNAME', (0, 0), (-1, -1), 'Helvetica'),
            ('FONTSIZE', (0, 0), (-1, -1), 12),
            ('BOTTOMPADDING', (0, 0), (-1, -1), 6),
            ('TOPPADDING', (0, 0), (-1, -1), 6),
            ('GRID', (0, 0), (-1, -1), 1, colors.lightgrey)
        ]))
        story.append(agent_table)
        story.append(Spacer(1, 12))
        
        if agent['categories']:
            story.append(Paragraph("Categories:", normal_style))
            for category in agent['categories']:
                story.append(Paragraph(f"• {category}", normal_style))
        story.append(Spacer(1, 20))
    
    # Build PDF
    doc.build(story)

if __name__ == '__main__':
    if len(sys.argv) != 3:
        print("Usage: python generate_report.py <input_json> <output_pdf>")
        sys.exit(1)
        
    input_file = sys.argv[1]
    output_file = sys.argv[2]
    
    with open(input_file, 'r') as f:
        data = json.load(f)
    
    generate_pdf(data, output_file)
