# IODD Engine 

This repository houses the implementation of an IoDD Engine, featuring a parser command-line interface (CLI) and a catalog system. The IoDD Engine is designed to facilitate seamless interaction and management of digital devices within interconnected networks. 

It also has a implementation of a Client for IFM IoTCore.

The parser CLI offers a user-friendly interface for executing commands and queries, enabling efficient communication with the IoDD Engine.
This Engine only targets Version 1.1 of IODD.

[IODD Specs](https://io-link.com/share/Downloads/Spec-IODD/IO-Device-Desc-Spec_10012_V113_Feb24.zip)


**Catalog**:
    - Store Information about VendorId, DeviceId and Productname and the correspoding IODD Zip File.    
    

**Engine:**
    - allows to parse PDIn and settings of variables. ObserverMenu and ParameterMenu are implemented.

**Cli:**    
    - allows cli based interactions with IODD Sensor

**IoTCoreConnect:**
    - Client for interaction with IFM IotCore


This Code is licensed under Apache 2.0.

Additional needed files [Standard Definitions](https://io-link.com/share/Downloads/Spec-IODD/IO-Device-Desc-Spec_10012_V113_Feb24.zip) manually in place it into /data/specs.

Please see CLI for further information how to work with Library.
This library was tested with serveral io-link devices.

### example output
Vendor: 310, ProductName:SD1540, Device:872
HexData: 0000000000000000FE00079EFE0000000300     
Name: Totalisator, Desc: Durchflussmenge. Der Wert entspricht der aktuellen Verbrauchsmenge seit dem letzten Reset, Value: 0.0, Unit: m³    
Name: Durchfluss, Desc: Aktueller Durchfluss, Value: 0.0, Unit: m³/h    
Name: Temperatur, Desc: Aktuelle Temperatur, Value: 19.5, Unit: °C    
Name: Druck, Desc: Aktueller Druck, Value: 0.0, Unit: bar    
Name: Gerätestatus, Desc: Aktueller Gerätestatus, eine Kopie des Parameters [Gerätestatus, Index 36] im Prozessdatenkanal, Value: 0.0    
Name: OUT2, Desc: Aktueller Zustand des digitalen Signals [OUT2], Value: "OFF"    
Name: OUT1, Desc: Aktueller Zustand des digitalen Signals [OUT1], Value: "OFF"    

### CLI examples
#### Read Current values from Device
ioddcli eval --ip 192.168.56.89 --port 1
#### Read Device variable by index
ioddcli read-parameter 192.168.56.89 2 551 0 

#### List all Variables
ioddcli  list-variable --vendorid 310 --deviceid 706 --productname TN2405 --accessrights rw

#### Show variable by Name (id)
ioddcli  variable --vendorid 310 --deviceid 706 --productname TN2405 --id V_diS

#### Read Parameter from IO DEVICE
cli read-parameter --ip 192.168.56.89 --port 2 --index 551 --subindex 0

#### Encode Variable
ioddcli  encode-variable --vendorid 310 --deviceid 706 --productname TN2405 --id V_diS --param false, true, 4

#### Set - Parameter 
cargo run set-parameter --ip 192.168.56.89 --port 2 --index 552 --subindex 0 --hexdata 4400

#### Convert RawValues to Hex
ioddcli  encode-variable --vendorid 310 --deviceid 706 --productname TN2405 --id V_uni --param "0"
or 
ioddcli  encode-variable --vendorid 310 --deviceid 706 --productname TN2405 --id V_diS --param "true, false, 4" 

#### Read Menus
cargo run get-menu  --vendorid 310 --deviceid 706 --productname TN2405 --role observer --menu parameter 


### example 
```rust
    let catalog = Catalog::new_with_db(None);
    let (drivername,files) = catalog.queryfordriver(*deviceid, productname.to_owned(), *vendorid).await;
    let p = Parser::new(drivername, files);
    let e = Engine::new(&p.iodevice, super::super::LANGLOCALE);
    println!("HexData: {}", hex_data);
    let datapoints = match e.parse(hex_data){
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
```

### TODO
- [ ] Read Variables from Device before printing PDIN eg. Device could be set to Fahrenheit instead of Celsisus
- [ ] Do CRC32 Check - if possible
- [ ] Update Github Actions
- [ ] Some more Documentation
- [ ] Adding more Encoder DataTypes

please note: 
In the event of any potential copyright violations, we kindly request that you contact the owner or administrator of the content in question before taking any further action. It is essential to address such matters through respectful and constructive communication to ensure that any concerns are properly addressed and resolved in accordance with applicable laws and regulations. By reaching out to the content owner first, we aim to foster a collaborative approach to resolving any copyright issues while upholding the integrity of intellectual property rights. Thank you for your understanding and cooperation in this matter.