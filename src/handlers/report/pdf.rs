use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use uuid::Uuid;

pub async fn generate_pdf(html_content: &str) -> Result<String, String> {
    println!("Starting PDF generation process");

    // Create a temporary directory to store the HTML file
    let temp_dir = TempDir::new()
        .map_err(|e| format!("Failed to create temp directory: {}", e))?;
    println!("Created temporary directory: {:?}", temp_dir.path());
    
    // Create a unique filename for the HTML file
    let html_path = temp_dir.path().join(format!("{}.html", Uuid::new_v4()));
    println!("HTML file path: {:?}", html_path);
    
    // Write HTML content to the temporary file
    fs::write(&html_path, html_content)
        .map_err(|e| format!("Failed to write HTML file: {}", e))?;
    println!("Wrote HTML content to temporary file");

    // Create PDF output directory if it doesn't exist
    let pdf_dir = PathBuf::from("reports");
    fs::create_dir_all(&pdf_dir)
        .map_err(|e| format!("Failed to create PDF directory: {}", e))?;
    println!("Ensured PDF directory exists: {:?}", pdf_dir);

    // Generate unique filename for PDF
    let pdf_filename = format!("report_{}.pdf", Uuid::new_v4());
    let pdf_path = pdf_dir.join(&pdf_filename);
    println!("PDF will be saved as: {:?}", pdf_path);

    // Launch headless Chrome with detailed options
    println!("Configuring Chrome launch options");
    let options = LaunchOptionsBuilder::default()
        .headless(true)
        .window_size(Some((1920, 1080)))
        .sandbox(false) // Disable sandbox for better compatibility
        .build()
        .map_err(|e| format!("Failed to build Chrome launch options: {}", e))?;

    println!("Launching headless Chrome");
    let browser = Browser::new(options)
        .map_err(|e| format!("Failed to launch Chrome: {}\nMake sure Chrome is installed on the system", e))?;

    println!("Creating new tab");
    // Create a new tab
    let tab = browser.new_tab()
        .map_err(|e| format!("Failed to create new tab: {}", e))?;

    // Navigate to the HTML file
    let html_url = format!("file://{}", html_path.to_string_lossy());
    println!("Navigating to HTML file: {}", html_url);
    tab.navigate_to(&html_url)
        .map_err(|e| format!("Failed to navigate to HTML file: {}", e))?;

    // Wait for network idle to ensure everything is loaded
    println!("Waiting for page to load");
    tab.wait_until_navigated()
        .map_err(|e| format!("Failed to wait for navigation: {}", e))?;

    // Generate PDF with specific options
    println!("Generating PDF");
    let pdf_options = headless_chrome::types::PrintToPdfOptions {
        landscape: Some(false),
        display_header_footer: Some(false),
        print_background: Some(true),
        scale: Some(1.0),
        paper_width: Some(8.27), // A4 width in inches
        paper_height: Some(11.69), // A4 height in inches
        margin_top: Some(0.4),
        margin_bottom: Some(0.4),
        margin_left: Some(0.4),
        margin_right: Some(0.4),
        page_ranges: Some("1-".to_string()),
        ignore_invalid_page_ranges: Some(true),
        prefer_css_page_size: Some(true),
        ..Default::default()
    };

    let pdf_data = tab.print_to_pdf(Some(pdf_options))
        .map_err(|e| format!("Failed to generate PDF: {}", e))?;

    // Save PDF to file
    println!("Saving PDF to: {:?}", pdf_path);
    fs::write(&pdf_path, pdf_data)
        .map_err(|e| format!("Failed to write PDF file: {}", e))?;

    println!("PDF generation completed successfully");
    // Return the filename (not the full path) for security
    Ok(pdf_filename)
}
