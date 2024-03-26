
use ioddengine::{engine::Engine,catalog::Catalog, ioddmodel11::ProfileBody::VariableCollection::Variable, parser::Parser};




pub async fn get_variable(deviceid : &i32, productname : &str, vendorid:&i32, id:&str , locale : &str){
    let catalog = Catalog::new_with_db(None);
    let (drivername,files) = catalog.queryfordriver(*deviceid, productname.to_owned(), *vendorid).await;
    let p = Parser::new(drivername, files);
    let e = Engine::new(&p.iodevice, locale);
    let v = e.findvariablebyid(id).expect("Variable not found");
    let realname = e.resolve_textid(v.Name.textid.as_str(), locale);
    let realdesc = e.resolve_textid(v.Description.textid.as_str(), locale);
    println!("VariableName: {} -  Desc: {}", realname, realdesc);

    for rc in &v.datatype.recorditem{
        let rcname = e.resolve_textid(rc.name.textid.as_str(), locale);
        println!("Name: {}, DataType: {} with Index: {} ",rcname, rc.datatype.datatype, rc.subindex);        
        if let Some(vr) =   &rc.datatype.valuerange {            
            println!("LowerValue: {}, UpperValue: {}", vr.lowervalue, vr.uppervalue);
        }        
        for xx in &rc.datatype.singlevalue{
            let xyz = e.resolve_textid(xx.name.textid.as_str(), locale);
            println!("  Name: {} - Value: {}", xyz, xx.value);
        }
    }
}
pub async fn getall_variable(deviceid : &i32, productname : &str, vendorid:&i32,accessrights : &Option<String>, locale:&str){
    let cmp = match accessrights{
        Some(x) => x.clone(),
        None => String::new(),
    };
    
    let catalog = Catalog::new_with_db(None);
    let (drivername,files) = catalog.queryfordriver(*deviceid, productname.to_owned(), *vendorid).await;
    let p = Parser::new(drivername, files);
    let e = Engine::new(&p.iodevice, locale);

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
        println!("id: {} - Name: {} - Index: {} - accessrights: {}",v.id.clone(),e.resolve_textid(v.Name.textid.as_str(), "en"),  v.index, v.accessrights);
    }
}


pub async fn encode_variable(deviceid : &i32, productname : &str, vendorid:&i32,id :&str, param:&str, locale:&str ){
    let catalog = Catalog::new_with_db(None);
    let (drivername,files) = catalog.queryfordriver(*deviceid, productname.to_owned(), *vendorid).await;
    let p = Parser::new(drivername, files);
    let e = Engine::new(&p.iodevice, locale);
    println!("HexString: {}", e.encode_variable(id, param));
}