

use ioddengine::{engine::Engine,  parser::Parser, catalog::Catalog};

use std::error::Error;

pub async fn readparameter(
    iot: &iotcoreconnect::iotcoreclient::HttpClient,
    port: &usize,
    index: &usize,
    subindex: &usize,
    locale: &str
) -> Result<(), Box<dyn Error>> {
    let deviceid = iot.get_device_at_port(*port).await?;
    let productname = iot.get_product_name(*port).await?;
    let vendorid = iot.get_vendor_at_port(*port).await?;
    println!(
        "Vendor: {}, ProductName:{}, Device:{}",
        vendorid, productname, deviceid
    );

    let catalog = Catalog::new_with_db(None);
    let (drivername, files) = catalog
        .queryfordriver(deviceid, productname.clone(), vendorid)
        .await;
    let p = Parser::new(drivername, files);
    let e = Engine::new(&p.iodevice, locale);

    let res = iot.readparameter(port, index, subindex).await;
    let m = match res {
        Ok(x) => x,
        Err(_) => panic!("Something wrong"),
    };
    
    let resasint = match i32::from_str_radix(&m, 16) {
        Ok(num) => num,
        Err(_e) => {
            panic!("not expected.");
        }
    };

    let v = e.findvariablebyindex(*index as i32);
    let s  = &v.datatype.singlevalue.iter().find(| sv | sv.value == resasint.to_string()).unwrap();
    println!("Name: {} - RawValue: {} - Translated_ {}",e.resolve_textid(v.Name.textid.as_str(), "de"), m, e.resolve_textid(s.name.textid.as_str(), "de"));

    Ok(())
}

pub async fn setparameter(
    iot: &iotcoreconnect::iotcoreclient::HttpClient,
    port: &usize,
    index: &usize,
    subindex: &usize,
    hexdata: &String,
    )    
    -> Result<(), Box<dyn Error>> {
    let deviceid = iot.get_device_at_port(*port).await?;
    let productname = iot.get_product_name(*port).await?;
    let vendorid = iot.get_vendor_at_port(*port).await?;
    println!(
        "Vendor: {}, ProductName:{}, Device:{}",
        vendorid, productname, deviceid
    );

    iot.setparameter(port, index, subindex, hexdata).await;
    Ok(())
}
