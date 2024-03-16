use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::copy;

use scraper::{Html, Selector};
const TARGET_FOLDER: &str = "../data/download";


pub fn download_files(url: &str) -> Result<(), Box<dyn Error>> {
    // Send a GET request to the URL and get the HTML content
    let html_content = reqwest::blocking::get(url)?.text()?;

    // Parse the HTML content
    let document = Html::parse_document(&html_content);
    let href_selector = Selector::parse("a[href]").unwrap();

    // Iterate over the links and download zip files
    for element in document.select(&href_selector) {
        let href_attr = element.value().attr("href").unwrap_or("");
        if href_attr.ends_with(".zip") {
            // Construct the URL of the zip file
            let zip_url = if href_attr.starts_with("http") {
                href_attr.to_string()
            } else {
                format!("{}/{}", "http://www.ifm.com/download", href_attr)
            };

            //https://www.ifm.com/download/files/IODD_PIUx05_IO-Link-11/$file/IODD_PIUx05_IO-Link-11.zip
            println!("Zip Url: {}",zip_url);
            // Download the zip file
            let response = reqwest::blocking::get(&zip_url)?;
            let zip_filename = Path::new(&href_attr).file_name().unwrap().to_str().unwrap();
            let  dest_path = Path::new(TARGET_FOLDER).join(zip_filename);
            let mut dest_file = File::create(&dest_path)?;

            // Copy the response body to the destination file
            copy(&mut response.bytes().unwrap().as_ref(), &mut dest_file)?;
        }
    }

    Ok(())
}
