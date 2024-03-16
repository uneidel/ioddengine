use flate2::read::GzDecoder;
use log::{info, warn};
use memmap::MmapOptions;
use microkv::MicroKV;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::{self, Cursor, Write, Read};
use std::path::Path;
use std::path::PathBuf;
use zip::ZipArchive;

pub struct Catalog {
    db: MicroKV,
}
const BASEPATH: &str = "../data/db";
const IODDBASEPATH: &str = "../data/sensors/";
impl Catalog {
    

    pub fn new_with_db(passwd: Option<String>) -> Self {
        let p = match passwd {
            Some(password) => password,
            None => String::new(),
        };
        info!("BasePath: {}", BASEPATH);
        let db = Catalog::load_db(p, BASEPATH);

        Catalog { db }
    }

    fn load_db(pwd: String, basepath: &str) -> MicroKV {
        MicroKV::open_with_base_path("iodd11", PathBuf::from(basepath))
            .expect("Failed to create MicroKV from a stored file or create MicroKV for this file")
            .set_auto_commit(true)
            .with_pwd_clear(pwd)
    }

    pub async fn queryfordriver(
        &self,
        deviceid: i32,
        product_name: String,
        vendorid: i32,
    ) -> (String, Vec<(String, Vec<u8>)>) {
        info!(
            "DeviceId: {}, Product_Name:{}, VendorId:{}",
            deviceid,
            product_name.clone(),
            vendorid
        );
        let entry = self.query(deviceid, product_name, vendorid).await.unwrap();
        info!("Driver: {:?}", &entry.content);
        let drivername: String = entry.get_drivername().unwrap();
        let filepath = format!("{}{}.zip", IODDBASEPATH, drivername);
        if !Path::new(&filepath).is_file() {
            info!("Need to download first");
            self.get_file_from_iodd_finder(vendorid, entry.content[0].iodd_id, entry.get_drivername().unwrap().as_str()).await;
        }

        let files = self.get_file(drivername.clone());
        (drivername, files)
    }
    async fn query(
        &self,
        deviceid: i32,
        product_name: String,
        vendorid: i32,
    ) -> Result<CatalogEntry, Box<dyn std::error::Error>> {
        let key = format!("{}_{}_{}", vendorid, deviceid, product_name);
        let existsindb = self.db.exists(key.clone());
        match existsindb {
            Ok(found) => {
                if found {
                    let val: CatalogEntry = self.db.get_unwrap(key.clone()).unwrap();

                    return Ok(val);
                }
            }
            Err(_) => {
                warn!("Not found.");
            }
        }

        let url = format!("https://ioddfinder.io-link.com/api/drivers?deviceId={deviceId}&vendorId={vendorId}&ioLinkRev=1.1&productName={productName}", deviceId = deviceid, vendorId = vendorid, productName = product_name);
        let response = reqwest::get(&url).await?; // Await the response

        if response.status().is_success() {
            let body = response.bytes().await?;

            let body_str = String::from_utf8(body.to_vec())?;
            let root: CatalogEntry = serde_json::from_str(&body_str)?;
            //adding to db
            let _ = self.db.put(key, &root);

            Ok(root)
        } else {
            println!("Request failed with status code: {}", response.status());
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Request failed",
            )))
        }
    }

    async fn get_file_from_iodd_finder(&self, vendorid: i32, iodd_id: i64, targetfilename: &str) {
        let path = format!("{}{}.zip", IODDBASEPATH, targetfilename);
        let url : String = format!("https://ioddfinder.io-link.com/api/vendors/{vendorid}/iodds/{iodd_id}/files/zip/");
        info!("Downloading... from {} to {}", url, path);
        let response = reqwest::get(url).await.unwrap();
        let mut file = std::fs::File::create(path).unwrap();
        let mut content =  Cursor::new(response.bytes().await.unwrap());
        let _ = std::io::copy(&mut content, &mut file);
    }

    fn get_file(&self, drivername: String) -> Vec<(String, Vec<u8>)> {
        info!("Checking if file exists");
        let filepath = format!("../data/sensors/{}.zip", drivername);
        if !Path::new(&filepath).is_file() {
            println!(
                "You need to open iodd-link to download : {}",
                filepath.clone()
            );
            println!("IODD File does not exists");
            std::process::exit(-1);
        }

        let files = self.extract(filepath.as_str());
        let mut xml_files: Vec<&String> = files
            .iter()
            .filter(|(filename, _)| filename.ends_with(".xml"))
            .map(|(filename, _)| filename)
            .collect();

        xml_files.sort_by_key(|a| a.len());

        files
    }

    fn extract_zip_to_ramdisk(&self, zip_file_path: &str) -> io::Result<Vec<(String, Vec<u8>)>> {
        // Open the ZIP file
        let file = File::open(zip_file_path)?;
        let mut archive = ZipArchive::new(file)?;
        let mut ramdisk: Vec<u8> = Vec::new();
        let mut files: Vec<(String, Vec<u8>)> = Vec::new();

        for i in 0..archive.len() {
            let mut zip_file = archive.by_index(i)?;

            // Read the file content into a buffer
            let mut buffer = Vec::new();
            zip_file.read_to_end(&mut buffer)?;

            // Decompress if necessary (e.g., if the file is gzipped)
            if zip_file.name().ends_with(".gz") {
                let mut decoder = GzDecoder::new(Cursor::new(buffer));
                let mut decompressed_buffer = Vec::new();
                decoder.read_to_end(&mut decompressed_buffer)?;
                buffer = decompressed_buffer;
            }

            // Push the content of the file into the RAM disk
            ramdisk.extend_from_slice(&buffer);

            // Store the file name and its content
            files.push((zip_file.name().to_string(), buffer));
        }

        // Write the RAM disk content to a temporary file
        let mut tmpfile = tempfile::tempfile()?;
        tmpfile.write_all(&ramdisk)?;

        // Map the temporary file to memory
        let _mmap = unsafe { MmapOptions::new().map(&tmpfile)? };

        Ok(files)
    }

    #[allow(unused)]
    fn is_valid_filename(filename: &str) -> bool {
        // Define the regular expression pattern
        let pattern = r"^[^-]+-[^-]+-\d{8}-IODD\d+-[a-zA-Z]+\.xml$";
        // Compile the regular expression
        let regex = Regex::new(pattern).unwrap();
        // Check if the filename matches the pattern
        regex.is_match(filename)
    }

    fn extract(&self, fname: &str) -> Vec<(String, Vec<u8>)> {
        let filescnt = self.extract_zip_to_ramdisk(fname);
        let files = filescnt.unwrap();
        let mut xml_files: Vec<&(String, Vec<u8>)> = files
            .iter()
            .filter(|(filename, _)| filename.ends_with(".xml"))
            .collect();

        xml_files.sort_by_key(|(filename, _)| filename.len());
        files
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogEntry {
    pub content: Vec<Content>,
    pub number: i64,
    pub size: i64,
    pub number_of_elements: i64,
    pub sort: Vec<Value>,
    pub first: bool,
    pub last: bool,
    pub total_pages: i64,
    pub total_elements: i64,
}
impl CatalogEntry {
    pub fn get_drivername(&self) -> Result<String, Box<dyn std::error::Error>> {
        if self.content.len() != 1 {
            return Err("DriverName Length should one".into());
        }
        Ok(self.content[0].driver_name.clone())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub has_more_versions: bool,
    pub device_id: i64,
    pub io_link_rev: String,
    pub version_string: String,
    pub iodd_id: i64,
    pub product_id: String,
    pub product_variant_id: i64,
    pub product_name: String,
    pub vendor_name: String,
    pub upload_date: i64,
    pub vendor_id: i64,
    pub iodd_status: String,
    pub indication_of_source: String,
    pub has_md: bool,
    pub driver_name: String,
}
