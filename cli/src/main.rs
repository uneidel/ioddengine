#![allow(dead_code)]
use clap::{Parser, Subcommand};
pub mod functions;
use crate::functions::*;


#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,   
}

#[derive(Subcommand)]
enum Commands {
    /// parses a single iodd xml
    Parse { file: String },
    /// parses a zipped iodd 
    Parsezip{ file: Option<String> },
    /// eval one port
    Eval {
        /// IP of IoTCore 
        #[arg(short, long)]
        ip: String,
        /// Io Link Port
        #[arg(short, long)]   
        port : usize },
    /// parse from raw
    Parseraw{
        #[arg(short, long)]  
        hex_data: String, 
        #[arg(short, long)]
        vendorid : i32,
        #[arg(short, long)]  
        deviceid: i32, 
        #[arg(short, long)]  
        productname : String 
    },
    /// Get Menu
    GetMenu{
        #[arg(short, long)]
        vendorid : i32, 
        #[arg(short, long)]
        deviceid: i32, 
        #[arg(short, long)]
        productname : String,
        // possible values :; oberserver, maintenance,specialist
        #[arg(short, long)]
        role : String,
        // possible values: observeration, diagnosis, parameter, identification
        #[arg(short, long)]
        menu: String,
        #[arg(short, long)]
        accessrights : Option<String>,    
    },

   
    /// Read Parameter from IO Device
    ReadParameter{
        #[arg(short, long)]  
        ip: String, 
        #[arg(short, long)]  
        port : usize, 
        #[arg(long)]  
        index: usize, 
        #[arg(short, long)]  
        subindex :usize 
    },
    SetParameter{ 
        #[arg(short, long)]
        ip: String,
        #[arg(short, long)]
        port : usize, 
        #[arg(long)]
        index: usize, 
        #[arg(long)]
        subindex :usize, 
        #[arg(long)]
        hexdata:String },
    /// show variable
    Variable {
        #[arg(short, long)]  
        vendorid : i32,
        #[arg(short, long)]  
        deviceid: i32,
        #[arg(short, long)]  
        productname : String,
        /// Optional either name or id
        #[arg(short, long)] 
        name : Option<String>,
        /// Optional either name or id
        #[arg(short, long)] 
        id : Option<String>        
    },  
    /// list all variables
    ListVariable {
        #[arg(short, long)]
        vendorid : i32, 
        #[arg(short, long)]
        deviceid: i32, 
        #[arg(short, long)]
        productname : String,
        /// Possible Values rw, ro, wo
        #[arg(short, long)]
        accessrights : Option<String>,
    },
    EncodeVariable{
        #[arg(short, long)]
        vendorid : i32,  
        #[arg(short, long)]
        deviceid: i32, 
        #[arg(short, long)]
        productname : String,
        #[arg(short, long)]
        id :String,
        #[arg(long)]
        param : String},
}

pub const LANGLOCALE :&str ="en";

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Parse { file } => {
            println!("Parse : {file:?}");
            parse::parse(file.clone().as_str());
        },
        Commands::Parsezip { file } => {
            println!("Parse zip: {file:?}")
        },
        Commands::Eval { ip, port} => {
            let iot = iotcoreconnect::iotcoreclient::HttpClient::new(ip.to_string());
            let res = eval::query(&iot, *port).await;
            match res{
                Ok(_) => {},
                Err(x) => println!("Error: {}", x)
            }
        },
        Commands::GetMenu { vendorid, deviceid, productname, role, menu, accessrights } => {
            menu::getmenu(vendorid, deviceid, productname, role, menu, accessrights).await;
        }
        Commands::Parseraw { hex_data, vendorid, deviceid, productname} => {
            let res = eval::parseraw(hex_data, deviceid, productname, vendorid ).await;
            match res{
                Ok(_) => {},
                Err(x) => println!("Error: {}", x)
            }
        },
       
        Commands::Variable { vendorid, deviceid, productname, name, id } =>{
            match (name.as_ref(), id.as_ref()) {
                (None, None) => {
                    panic!("Both options are empty.");
                },
                (Some(_name), None) => {
                    // get id by name
                    //let id = name;
                    todo!("to be implemented.");
                    //variable::get_variable(deviceid, productname, vendorid, &id).await;
                }, 
                (None, Some(id)) =>{
                    variable::get_variable(deviceid, productname, vendorid, id).await;
                }
                _ => {}
            }
            
            
        },
        Commands::ListVariable { vendorid, deviceid, productname, accessrights } =>{
            variable::getall_variable(deviceid,productname,vendorid, accessrights).await;
        }
        Commands::ReadParameter { ip, port, index, subindex } =>{
            let iot = iotcoreconnect::iotcoreclient::HttpClient::new(ip.to_string());
            let _ = functions::parameter::readparameter(&iot, port, index, subindex).await;
        }, 
        Commands::SetParameter { ip, port, index, subindex, hexdata } =>{
            let iot = iotcoreconnect::iotcoreclient::HttpClient::new(ip.to_string());
            let res = functions::parameter::setparameter(&iot, port, index, subindex, hexdata).await;
            match res{
                Ok(_) => {},
                Err(x) => println!("Error: {}", x)
            }
        },
        Commands::EncodeVariable { vendorid, deviceid, productname,id, param} => {
            functions::variable::encode_variable(deviceid, productname, vendorid,id, param).await;   
        }
    }
}

#[cfg(test)]
mod tests {
    use ioddengine::catalog::Catalog;
    use ioddengine::engine::{DataPoint, Engine};

    use super::*;
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