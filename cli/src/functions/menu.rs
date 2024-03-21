
use std::env::var;

use ioddengine::ioddmodel11::ProfileBody::VariableCollection::Variable;
use ioddengine::{engine::Engine,  parser::Parser, catalog::Catalog};
use ioddengine::ioddmodel11::IODevice::*;
pub async fn getmenu(vendorid : &i32, deviceid : &i32, productname : &str, role :&str, menu : &str,accessrights : &Option<String>){
    
    let roleset = match role{
        "observer" => RoleSet::Observer,
        "maintenance" => RoleSet::Maintenance,
        "specialist" => RoleSet::Specialist,
        _ => panic!("RoleSet not defined.")
    };
    let rolemenu = match menu{
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
    println!("MenuID: {}", menu.menuid.clone());
    let menus = e.getmenubyid(menu.menuid.clone());
    // now get all variables for menu
    let mut variables: Vec<&Variable> = Vec::new();
    for x in menus {
        let trans = e.resolve_textid(x.name.textid.as_str(),super::super::LANGLOCALE);
        println!("\x1b[1mMenuname:\x1b[1m {} - \x1b[1mDescription:\x1b[1m {}", x.id, trans);


        for rcref in &x.recorditemref{            
            let v =  e.findvariablebyid(&rcref.variableId).unwrap();
            variables.push(v);
           
        }           
        for v in &x.variableref {           
            let rv = e.findvariablebyid(&v.variableId).unwrap();
            variables.push(rv);
        } 
      
        for v in &variables{      
            let realname = e.resolve_textid(v.Name.textid.as_str(),super::super::LANGLOCALE);
            let realdesc = e.resolve_textid(v.Description.textid.as_str(),super::super::LANGLOCALE);
           
            match &accessrights {
                Some(x) => {
                    if x.to_lowercase() == v.accessrights {
                        println!(
                            "   \x1b[1mVariable:\x1b[0m {}  \x1b[1mRealName:\x1b[0m {}  \x1b[1mDesc:\x1b[0m {}  \x1b[1mACCESS:\x1b[0m {}",
                            v.id, realname, realdesc, v.accessrights
                        );
                    }
                }
                None => {
                    println!(
                        "   \x1b[1mVariable:\x1b[0m {}  \x1b[1mRealName:\x1b[0m {}  \x1b[1mDesc:\x1b[0m {}  \x1b[1mACCESS: {}",
                        v.id, realname, realdesc, v.accessrights
                    );
                }
            };
        }
    }
}