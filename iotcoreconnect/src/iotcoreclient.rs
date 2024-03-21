use serde::Deserialize;

use log::info;
use reqwest::*;
use crate::errors::*;


pub struct HttpClient {
    client: Client,
    url : String
}

impl HttpClient {

    pub fn new(url:String) -> Self{
        let client = Client::new();
        let mut fixedurl = url;
        if !fixedurl.starts_with("http://") {
            fixedurl = format!("http://{}", fixedurl);
        }
       
        HttpClient{ client, url: fixedurl}
    }
    
    async fn getdeviceinfo(&self,request : &str) -> HttpResult<Value> {
        let json_data = format!(
            r#"{{
                "code": "request",
                "cid": 1, 
                "adr": "/deviceinfo/{}/getdata"
            }}"#,
            request
        );
        info!("JSON: {}", json_data);
        let data =  self.dopost(json_data).await.unwrap();
        info!("Response: {:?}", data);
        let response: ResponseData = match serde_json::from_str(&data) {
            Ok(res) => res,
            Err(err) => return Err(IotCoreError::OtherError(err.to_string())) 
        };
       
        match response.code {
            200 => Ok(response.data.value),
            _ => Err(IotCoreError::HttpError(format!("ResponseCode: {}", response.code)))
        }
    }

    pub async fn readparameter(&self, portnumber : &usize, index: &usize, subindex : &usize) -> HttpResult<String>{
        let data = format!(
            r#"{{
                "index": {},
                "subindex" :{}
                }}"#,
            index, subindex
        );
        
        let response =self.senddata(portnumber,4711, "iolinkdevice/iolreadacyclic", &data).await;
        match  response{
            Ok(Value::String(string_value)) => Ok(string_value),
            Err(err) => Err(IotCoreError::OtherError(err.to_string())), 
            _ => panic!("TODO")
        }
    }
    pub async fn setparameter(&self, portnumber : &usize, index: &usize, subindex : &usize, hexdata :&String){
        let data = format!(
            r#"{{
                "index": {},
                "subindex" :{},
                "value" :"{}"
                }}"#,
            index, subindex, hexdata
        );
        
        let response =self.senddata(portnumber,4711, "iolinkdevice/iolwriteacyclic", &data).await;
        println!("Response: {:?}", response);
    }
    pub async fn getblob(&self, portnumber : &usize){
        let res = self.getportinfo(portnumber, "datastorage/chunksize/getdata").await;
        let chunksize: i32 = match res{
         Ok(Value::Integer(int_value)) => int_value, 
            _ => {panic!("Error reading chunksize")}
        };
        info!("Chunksize: {}", chunksize);
        let size = self.getportinfo(portnumber, "datastorage/size/getdata").await;
        let fullsize = match size{
            Ok(Value::Integer(int_value)) => int_value,
            _ => {panic!("Error reading chunksize")}
        };
        info!("FullSize: {}", fullsize);
        let callscounter = (fullsize  / chunksize)+1 ;
        info!("Calls needed: {}", callscounter);
        for _i in 1..callscounter{
            //{"code": "request", "cid": -1, "adr": "/iolinkmaster/port[2]/datastorage/getblobdata", "data": {"pos": 0, "length": h}}
            //{"code": "request", "cid": -1, "adr": "/iolinkmaster/port[2]/datastorage/getblobdata", "data": {"pos": h, "length": h}}
            //{"code": "request", "cid": -1, "adr": "/iolinkmaster/port[2]/datastorage/getblobdata", "data": {"pos": 2*h, "length": h
            /*if fullsize < chunksize{

            }
            */
            todo!("Implement me");
            //self.senddata(portnumber, "datastorage", "")
        }
    }
    async fn senddata(&self, portnumber: &usize,cid : i32, info: &str,data : &str) -> HttpResult<Value> {
        let json_data = format!(
            r#"{{
                "code" : "request",
                "cid"  : {}, 
                "adr"  : "/iolinkmaster/port[{}]/{}",
                "data" : {}
            }}"#,
            cid,portnumber, info, data
        );
        info!("JSON: {}", json_data);
        let data =  self.dopost(json_data).await.unwrap();
        info!("Response: {:?}", data);
        let response: ResponseData = match serde_json::from_str(&data) {
            Ok(res) => res,
            Err(err) => return Err(IotCoreError::OtherError(err.to_string())) 
        };
       
        match response.code {
            200 => Ok(response.data.value),
            _ => Err(IotCoreError::HttpError(format!("ResponseCode: {}", response.code)))
        }
    }
   
    async fn getportinfo(&self, portnumber: &usize, info: &str) -> HttpResult<Value> {
        let json_data = format!(
            r#"{{
                "code": "request",
                "cid": 1, 
                "adr": "/iolinkmaster/port[{}]/{}"
            }}"#,
            portnumber, info
        );
        info!("JSON: {}", json_data);
        let data =  self.dopost(json_data).await.unwrap();
        info!("Response: {:?}", data);
        let response: ResponseData = match serde_json::from_str(&data) {
            Ok(res) => res,
            Err(err) => return Err(IotCoreError::OtherError(err.to_string())) 
        };
       
        match response.code {
            200 => Ok(response.data.value),
            _ => Err(IotCoreError::HttpError(format!("ResponseCode: {}", response.code)))
        }
    }
    pub async fn get_vendor_at_port(&self, portnumber: usize) -> HttpResult<i32> {
        let vendor = self.getportinfo(&portnumber, "iolinkdevice/vendorid/getdata").await;
        match vendor {
            Ok(Value::Integer(int_value)) => Ok(int_value),
            _ => Err(IotCoreError::DeviceNotFound),
        }
    }

    pub async fn getdevicefamily(&self) -> String{
        let productcode = self.getdeviceinfo("devicefamily/getdata").await;
        match productcode {
            Ok(Value::String(string_value)) => string_value,
            _ => String::new(),
        }
    }
    pub async fn get_product_code(&self) -> String{
        let productcode = self.getdeviceinfo("productcode/getdata").await;
        match productcode {
            Ok(Value::String(string_value)) => string_value,
            _ => String::new(),
        }
    }
    pub async fn get_device_at_port(&self, port_number: usize) -> HttpResult<i32> {
        let vendor = self.getportinfo(&port_number, "iolinkdevice/deviceid/getdata").await;
        match vendor {
            Ok(Value::Integer(int_value)) => Ok(int_value),
            _ => Err(IotCoreError::DeviceNotFound), // Don't need to box the enum variant
        }
    }

    pub async fn get_data_at_port(&self, port_number: usize) -> HttpResult<String> {
        let data = self.getportinfo(&port_number, "iolinkdevice/pdin/getdata").await;
        info!("DataIN at Port {}: {:?}", port_number, data);
        match data {
            Ok(Value::String(string_value)) => Ok(string_value),
            _ => Err(IotCoreError::DeviceNotFound),
        }
    }
    
    pub async fn get_product_name(&self, portnumber: usize) -> HttpResult<String> {
        let vendor = self.getportinfo(&portnumber, "iolinkdevice/productname/getdata").await;
        match vendor {
            Ok(Value::String(string_value)) => Ok(string_value),
            _ => Err(IotCoreError::DeviceNotFound),
        }
    }

    pub async fn get_device_identifier(&self, portnumber : usize) -> (i32,  i32, String,) {
        let productname = self.get_product_name(portnumber).await.unwrap();
        let vendorid : i32 = self.get_vendor_at_port(portnumber).await.unwrap();
        let deviceid : i32 = self.get_device_at_port(portnumber).await.unwrap();
        (vendorid, deviceid, productname)
    }

    pub async fn getportstatus(&self, portnumber : usize) -> i8 {
        let status = self.getportinfo(&portnumber, "iolinkdevice/status/getdata").await;
       
        match status {
            Ok(Value::Integer(status)) => status as i8,
            _ => -1
        }
    }
    pub fn get_msg_port_status(&self,status : i8)-> String{
        let result = match status{
            0 => "State not connected",
            1 => "State preoperate",
            2 => "State operate",
            3 => "State communication error",
            _ => "Error"
        };
        result.to_string()
    }
    async fn dopost(&self, payload: String) -> HttpResult<String> {
        
        // Send the POST request asynchronously
        let response = self.client
            .post(self.url.clone())
            .header("Content-Type", "application/json")
            .body(payload)
            .send()
            .await?;
    
        if response.status().is_success() {
            let response_text = response.text().await?;
            Ok(response_text) // Return response text if successful
        } else {
            // If request fails, construct an error with the response body
            Err(IotCoreError::HttpError(response.status().to_string()))
        }
    }
    
}


#[derive(Debug, Deserialize)]
pub struct ResponseData {
    #[allow(dead_code)]
    cid: u32,
    data: Data,
    #[allow(dead_code)]
    code: u32,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    value: Value,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Integer(i32),
    String(String),
    None,
}