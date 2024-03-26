
use ioddengine::{engine::Engine, parser::Parser, catalog::Catalog};

use std::error::Error;

pub async fn query(iot: &iotcoreconnect::iotcoreclient::HttpClient, port: usize)     -> Result<(), Box<dyn Error>>{
    let deviceid = iot.get_device_at_port(port).await?;
    let productname = iot.get_product_name(port).await?;
    let vendorid = iot.get_vendor_at_port(port).await?;
    println!(
        "Vendor: {}, ProductName:{}, Device:{}",
        vendorid, productname, deviceid
    );
    let hexdata = iot.get_data_at_port(port).await?;
    parseraw(&hexdata, &deviceid, &productname, &vendorid).await?;
    
    Ok(())
}


pub async fn parseraw(hex_data : &str, deviceid : &i32, productname : &str, vendorid:&i32 )-> Result<(), Box<dyn Error>>{
    let catalog = Catalog::new_with_db(None);
    let (drivername,files) = catalog.queryfordriver(*deviceid, productname.to_owned(), *vendorid).await;
    let p = Parser::new(drivername, files);
    let e = Engine::new(&p.iodevice, super::super::LANGLOCALE);
    println!("HexData: {}", hex_data);
    let entries = match e.parse(hex_data){
        Ok(x) => x,
        Err(err) => {panic!("Error: {:?}", err)}
    };
    for h in entries {
        println!(
            "Name: {}, Desc: {}, Value: {}{}",
            h.name,
            h.description,
            h.value,
            if !h.unit.is_empty() {
                format!(", Unit: {}", h.unit)
            } else {
                String::new() // Empty string if unit is empty
            }
        );
    }
    Ok(())
}


/*
pub async fn parseraw(hex_data : &str, deviceid : &i32, productname : &str, vendorid:&i32 )-> Result<Vec<DataPoint>, Box<dyn Error>>{
    let catalog = Catalog::new(None);
    let (drivername,files) = catalog.queryfordriver(*deviceid, productname.to_owned(), *vendorid).await;
    let p = Parser::new(drivername, files);
    let e = Engine::new(&p.iodevice, super::super::LANGLOCALE);
    println!("HexData: {}", hex_data);
    let entries = match e.parse(hex_data){
        Ok(x) => x,
        Err(err) => {panic!("Error: {:?}", err)}
    };
    
    Ok(entries)
}
*/