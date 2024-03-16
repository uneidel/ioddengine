
use ioddengine::{engine::Engine,  parser::Parser, catalog::Catalog};
use ioddengine::ioddmodel11::IODevice::*;
pub async fn getmenu(vendorid : &i32, deviceid : &i32, productname : &String, role :&String, menu : &String,accessrights : &Option<String>){
    
    let roleset = match role.as_str(){
        "observer" => RoleSet::Observer,
        "maintenance" => RoleSet::Maintenance,
        "specialist" => RoleSet::Specialist,
        _ => panic!("RoleSet not defined.")
    };
    let rolemenu = match menu.as_str(){
        "observation" => RoleMenu::Observation,
        "diagnosis" => RoleMenu::Diagnosis,
        "parameter" => RoleMenu::Parameter,
        "identification" => RoleMenu::Identification,
        _ => panic!("rolemenu does not exists")
    };

    let catalog = Catalog::new_with_db(None);
    let (drivername, files) = catalog
        .queryfordriver(*deviceid, productname.to_owned(), *vendorid)
        .await;
    let p = Parser::new(drivername, files);
    let e = Engine::new(&p.iodevice, super::super::LANGLOCALE);
    
    let menu = e.getmenu(&roleset, &rolemenu);
    
    let m = e.getmenubyid(menu.menuid.clone());
    //println!("Menu: {:?}", m);
    // now get all variables for menu
    for x in m {
        let trans = e.resolve_textid(x.name.textid.as_str(), "de");
        println!("\x1b[1mName:\x1b[1m {} - \x1b[1mDescription:\x1b[1m {}", x.id, trans);

        let vars = &x.variableref;
        
        for v in vars {
            let rv = e.get_iodevice()
                .profilebody
                .device_function
                .variablecollection
                .variable
                .iter()
                .find(|vs| vs.id == v.variableId);

                let resolvedrv = match rv {
                    Some(x) => x,
                    _ => match e.get_standards().definitions.variablecollection.variable.iter().find(|vs| vs.id == v.variableId) {
                        Some(x) => x,
                        None => panic!("no variable found"),
                    },
                };
                
            let realname = e.resolve_textid(resolvedrv.Name.textid.as_str(), "de");
            let realdesc = e.resolve_textid(resolvedrv.Description.textid.as_str(), "de");
           
            match &accessrights {
                Some(x) => {
                    if x.to_lowercase() == v.accessrights {
                        println!(
                            "\x1b[1mVariable:\x1b[0m {}  \x1b[1mRealName:\x1b[0m {}  \x1b[1mDesc:\x1b[0m {}  \x1b[1mACCESS:\x1b[0m {}",
                            v.variableId, realname, realdesc, v.accessrights
                        );
                    }
                }
                None => {
                    println!(
                        "\x1b[1mVariable:\x1b[0m {}  \x1b[1mRealName:\x1b[0m {}  \x1b[1mDesc:\x1b[0m {}  \x1b[1mACCESS: {}",
                        v.variableId, realname, realdesc, v.accessrights
                    );
                }
            };
        }
    }
}