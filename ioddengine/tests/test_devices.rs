use ioddengine::{catalog::Catalog, engine::Engine};

#[tokio::test]
async fn test_as001_read(){
    
let hexdata = "008D00D3";
let catalog = ioddengine::catalog::Catalog::new_with_db(None);
let (drivername, files) = 
catalog.queryfordriver(16, "AS001".to_string(), 837).await;
let p = ioddengine::parser::Parser::new(drivername, files);
let e = Engine::new(&p.iodevice, "de");

let res = e.parse(hexdata).unwrap();
assert_eq!(res[1].value,52.0);
}

#[tokio::test]
async fn test_al009_read(){
    
    let hexdata = "0046FD0000190001";
    let catalog = ioddengine::catalog::Catalog::new_with_db(None);
    let (drivername, files) = 
    catalog.queryfordriver(85, "AL009".to_string(), 837).await;
    let p = ioddengine::parser::Parser::new(drivername, files);
    let e = Engine::new(&p.iodevice, "de");
    
    let res = e.parse(hexdata).unwrap();
    assert_eq!(res[0].value,70.0);
}
#[tokio::test]
async fn test_o5d150_read(){
    
    let hexdata = "0251";
    let catalog = ioddengine::catalog::Catalog::new_with_db(None);
    let (drivername, files) = 
    catalog.queryfordriver(372, "O5D150".to_string(), 310).await;
    let p = ioddengine::parser::Parser::new(drivername, files);
    let e = Engine::new(&p.iodevice, "de");
    
    let res = e.parse(hexdata).unwrap();
    assert_eq!(res[0].value,37.0);
}

#[tokio::test]
async fn test_sd1540_read(){
    
    let hexdata = "0000000000000000FE00079EFE0000000300";
    let catalog = ioddengine::catalog::Catalog::new_with_db(None);
    let (drivername, files) = 
    catalog.queryfordriver(872, "SD1540".to_string(), 310).await;
    let p = ioddengine::parser::Parser::new(drivername, files);
    let e = Engine::new(&p.iodevice, "de");
    
    let res = e.parse(hexdata).unwrap();
    assert_eq!(res[2].value,19.5);
}

#[tokio::test]
async fn test_si5010_read(){
    
    let hexdata = "FFF6005A";
    let catalog = ioddengine::catalog::Catalog::new_with_db(None);
    let (drivername, files) = 
    catalog.queryfordriver(54, "SI5010".to_string(), 310).await;
    let p = ioddengine::parser::Parser::new(drivername, files);
    let e = Engine::new(&p.iodevice, "de");
    
    let res = e.parse(hexdata).unwrap();
    assert_eq!(res[1].value,22.0);
}

#[tokio::test]
async fn test_kq5100_read(){
    
    let hexdata = "0020";
    let catalog = ioddengine::catalog::Catalog::new_with_db(None);
    let (drivername, files) = 
    catalog.queryfordriver(371, "KQ5100".to_string(), 310).await;
    let p = ioddengine::parser::Parser::new(drivername, files);
    let e = Engine::new(&p.iodevice, "de");
    
    let res = e.parse(hexdata).unwrap();
    assert_eq!(res[0].value,2.0);
}

#[tokio::test]
async fn test_vvb001_status_b_read(){
    
    let hexdata = "004BFC00000EFF000005FF0000DFFF00001FFF00";
    let catalog = ioddengine::catalog::Catalog::new_with_db(None);
    let (drivername, files) = 
    catalog.queryfordriver(1367, "VVB001 Status B".to_string(), 310).await;
    let p = ioddengine::parser::Parser::new(drivername, files);
    let e = Engine::new(&p.iodevice, "de");
    
    let res = e.parse(hexdata).unwrap();
    assert_eq!(res[0].value,0.0075);
    assert_eq!(res[1].value,1.4);
}

#[tokio::test]
async fn test_2405_read(){
    
    let hexdata = "0330";
    let catalog = ioddengine::catalog::Catalog::new_with_db(None);
    let (drivername, files) = 
    catalog.queryfordriver(706, "TN2405".to_string(), 310).await;
    let p = ioddengine::parser::Parser::new(drivername, files);
    let e = Engine::new(&p.iodevice, "de");
    
    let res = e.parse(hexdata).unwrap();
    assert_eq!(res[0].value,20.4);
    assert_eq!(res[0].unit,"°C");
}

#[tokio::test]
async fn test_2405_without_db_read(){
    
    let hexdata = "0330";
    
    let (drivername, files) = 
    Catalog::query_in_memory(310,706, "TN2405").await.unwrap();
    let p = ioddengine::parser::Parser::new(drivername, files);
    let e = Engine::new(&p.iodevice, "de");
    
    let res = e.parse(hexdata).unwrap();
    assert_eq!(res[0].value,20.4);
    assert_eq!(res[0].unit,"°C");
}