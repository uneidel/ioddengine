
#[cfg(test)]
mod tests {
   
    use ioddengine::{engine::{DataPoint, Engine}, parser::Parser, catalog::Catalog};
  
    
    

    pub const LANGLOCALE : &str = "de";
    ///Vendor: 310, ProductName:O5D150, Device:372
    #[tokio::test]
    pub async fn test_05d150() {
        let vendorid = 310;
        let deviceid = 372;
        let productname = "05D150";
        let vector = parseraw("01B1", &deviceid, productname, &vendorid).await;
        assert_eq!("27.0", "27.0");
    }

    pub async fn parseraw(hex_data : &str, deviceid : &i32, productname : &str, vendorid:&i32 )-> Vec<DataPoint>{
        let catalog = Catalog::new_with_db(None);
        let (drivername,files) = catalog.queryfordriver(*deviceid, productname.to_owned(), *vendorid).await;
        let p = Parser::new(drivername, files);
        let e = Engine::new(&p.iodevice, LANGLOCALE);
        println!("HexData: {}", hex_data);
        let entries = match e.parse(hex_data){
            Ok(x) => x,
            Err(err) => {panic!("Error: {:?}", err)}
        };
        for h in &entries {
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
        entries
    }

    #[tokio::test]
    async fn test_AS001_Read() {
        let hexdata = "008D00D3";
        let catalog = Catalog::new_with_db(None);
        let (drivername, files) = 
        catalog.queryfordriver(16, "AS001".to_string(), 837).await;
        let p = ioddengine::parser::Parser::new(drivername, files);
        let e = Engine::new(&p.iodevice, "de");

        let res = e.parse(hexdata).unwrap();
        
        todo!("Implement me");
    }
}
