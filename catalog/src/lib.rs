#!
mod downloader;


#[allow(dead_code)]
fn downloadallfromifm() {
    let ifmurl = "https://www.ifm.com/download/read_io-link_EN";
    #[allow(dead_code)]
    match downloader::download_files(ifmurl) {
        Ok(()) => println!("Files downloaded successfully"),
        Err(err) => eprintln!("Error downloading files: {}", err),
    }
}


