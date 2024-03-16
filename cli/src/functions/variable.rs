
use ioddengine::{engine::Engine,catalog::Catalog, ioddmodel11::ProfileBody::VariableCollection::Variable, parser::Parser};




pub async fn get_variable(deviceid : &i32, productname : &str, vendorid:&i32, id:&str ){
    let catalog = Catalog::new_with_db(None);
    let (drivername,files) = catalog.queryfordriver(*deviceid, productname.to_owned(), *vendorid).await;
    let p = Parser::new(drivername, files);
    let e = Engine::new(&p.iodevice, super::super::LANGLOCALE);
    let v = e.findvariablebyid(id);
    let realname = e.resolve_textid(v.Name.textid.as_str(), super::super::LANGLOCALE);
    let realdesc = e.resolve_textid(v.Description.textid.as_str(), super::super::LANGLOCALE);
    println!("VariableName: {} -  Desc: {}", realname, realdesc);

    for rc in &v.datatype.recorditem{
        println!("DataType: {} with Index: {} ",rc.datatype.datatype, rc.subindex);
        for xx in &rc.datatype.singlevalue{
            let xyz = e.resolve_textid(xx.name.textid.as_str(), super::super::LANGLOCALE);
            println!("  Name: {} - Value: {}", xyz, xx.value);
        }
    }

    for xx in &v.datatype.singlevalue{
        let xyz = e.resolve_textid(xx.name.textid.as_str(), "de");
        println!("Name: {} - Value: {}", xyz, xx.value);
    }
}
pub async fn getall_variable(deviceid : &i32, productname : &str, vendorid:&i32,accessrights : &Option<String>){
    let cmp = match accessrights{
        Some(x) => x.clone(),
        None => String::new(),
    };
    
    let catalog = Catalog::new_with_db(None);
    let (drivername,files) = catalog.queryfordriver(*deviceid, productname.to_owned(), *vendorid).await;
    let p = Parser::new(drivername, files);
    let e = Engine::new(&p.iodevice, super::super::LANGLOCALE);

    let varcoll = &e.get_iodevice().profilebody.device_function.variablecollection;
    let mut fullvec: Vec<&Variable> = Vec::new();
    let s = e.get_standards();
    for v in &varcoll.stdvariableref{
        
        let lookup = s.definitions.variablecollection.variable.iter().find(| x | x.id == v.id.clone()).unwrap();
        if !cmp.is_empty() && lookup.accessrights == *cmp{
            fullvec.push(lookup)
        }
        else if cmp.is_empty(){
            fullvec.push(lookup);
        }
    }
    for vx in &varcoll.variable{
        if !cmp.is_empty() && vx.accessrights == *cmp{
            fullvec.push(vx)
        }
        else if cmp.is_empty(){
            fullvec.push(vx);
        }
    }
    fullvec.sort_by_key(| k | &k.index);

    for v in fullvec{
        println!("id: {} - Name: {} - Index: {} - accessrights: {}",v.id.clone(),e.resolve_textid(v.Name.textid.as_str(), "en"),  v.index, v.accessrights,);
    }
}


pub async fn encode_variable(deviceid : &i32, productname : &str, vendorid:&i32,id :&str, param:&str ){
    let catalog = Catalog::new_with_db(None);
    let (drivername,files) = catalog.queryfordriver(*deviceid, productname.to_owned(), *vendorid).await;
    let p = Parser::new(drivername, files);
    let e = Engine::new(&p.iodevice, super::super::LANGLOCALE);
    println!("HexString: {}", e.encode_variable(id, param));
}