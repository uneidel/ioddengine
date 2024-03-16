# IODD Engine 

This repository houses the implementation of an IoDD Engine, featuring a parser command-line interface (CLI) and a catalog system. The IoDD Engine is designed to facilitate seamless interaction and management of digital devices within interconnected networks. 

It also has a implementation of a Client for IFM IoTCore.

The parser CLI offers a user-friendly interface for executing commands and queries, enabling efficient communication with the IoDD Engine.
This Engine only targets Version 1.1 of IODD.

[IODD Specs](https://io-link.com/share/Downloads/Spec-IODD/IO-Device-Desc-Spec_10012_V113_Feb24.zip)


Catalog:
    - Store Information about VendorId, DeviceId and Productname and the correspoding IODD Zip File.    
    - FIXED! <del>Please note: Due to License restriction you need to manually download missing IODD Zip Files from [IODD-FInder](https://ioddfinder.io-link.com/) and place it into /data/download.</del>

Engine:
    - allows to parse PDIn and settings of variables. ObserverMenu and ParameterMenu are implemented.

Cli:    
    - allows cli based interactions with IODD Sensor

IoTCoreConnect: 
    - Client for interaction with IFM IotCore


This Code is licensed under Apache 2.0.

Due to potential License Issues you need to download the [Standard Definitions](https://io-link.com/share/Downloads/Spec-IODD/IO-Device-Desc-Spec_10012_V113_Feb24.zip) manually in place it into /data/specs.

Please see CLI for further information how to work with Library.

Tested with the following  Devices:


| Company  | deviceid   |  productname  | PDIN  | Write  | Test |
|----------|------------|---------------|-------|--------|------|
|   IFM    |    372     |  05d150       |   X   |        |   X  |
|   IFM    |            |  LMT121       |   X   |        |      |
|   IFM    |            |  LR3300       |   X   |        |      |
|   IFM    |            |  PN7592       |   X   |        |      |
|   IFM    |            |  TN2405       |   X   |        |      |
|   IFM    |            |      KQ5100   |   X   |        |  X   |
|   AUTOSEN|    85      |      AL009    |   X   |        |  X   |
|   AUTOSEN|    16      |      AS001    |   X   |        |  X   |
|  IFM     |  872       |     SD1540    |   X   |        | X    |


## Vendor: 837, ProductName:AL009, Device:85
HexData: 0046FD0000190001   
Name: Entfernung, Desc: Aktuelle Entfernung des Objekts, Value: 70.0, Unit: mm   
Name: Reflectivity, Desc: Current object reflectivity, Value: 25.0, Unit: %   
Name: Gerätestatus, Desc: Aktueller Gerätestatus, eine Kopie der Variable [Gerätestatus] im Prozessdatenkanal, Value: 0.0   
Name: OUT2, Desc: Status abhängig von [OU2], Value: "inaktiv"  
Name: OUT1, Desc: Status abhängig von [OU1], Value: "inaktiv"   

## Vendor: 837, ProductName:AS001, Device:16
HexData: 008D00D3    
Name: Durchfluss, Desc: Aktueller Durchfluss, Value: "OL", Unit: %    
Name: Temperatur, Desc: Aktuelle Temperatur, Value: 52.0, Unit: %   
Name: Schaltzustand [OUT2], Desc: Status abhängig von [OU2], Value: "inactiv"   
Name: Schaltzustand [OUT1], Desc: Status abhängig von [OU1], Value: "inactiv"   

## Vendor: 310, ProductName:O5D150, Device:372
HexData: 0251    
Name: Abstand, Desc: Abb. PDV1. Aktueller Abstand., Value: 37.0, Unit: cm    
Name: Schaltzustand [OUT1], Desc: Abb. BDC1. Zustand abhängig von Einstellungen für BDC1., Value: "Inaktiv"   

## Vendor: 310, ProductName:SD1540, Device:872
HexData: 0000000000000000FE00079EFE0000000300
Name: Totalisator, Desc: Durchflussmenge. Der Wert entspricht der aktuellen Verbrauchsmenge seit dem letzten Reset, Value: 0.0, Unit: m³
Name: Durchfluss, Desc: Aktueller Durchfluss, Value: 0.0, Unit: m³/h
Name: Temperatur, Desc: Aktuelle Temperatur, Value: 19.5, Unit: °C
Name: Druck, Desc: Aktueller Druck, Value: 0.0, Unit: bar
Name: Gerätestatus, Desc: Aktueller Gerätestatus, eine Kopie des Parameters [Gerätestatus, Index 36] im Prozessdatenkanal, Value: 0.0
Name: OUT2, Desc: Aktueller Zustand des digitalen Signals [OUT2], Value: "OFF"
Name: OUT1, Desc: Aktueller Zustand des digitalen Signals [OUT1], Value: "OFF"


## Vendor: 310, ProductName:SI5010, Device:54
HexData: FFF6005A
Name: Durchfluss, Desc: Aktueller Durchfluss, Value: -10.0, Unit: %
Name: Temperatur, Desc: Aktuelle Temperatur, Value: 22.0, Unit: °C
Name: Schaltzustand [OUT2] nicht anwendbar, Desc: Das [OUT2] Bit ist zwar belegt, es wird aber nicht verwendet, Value: "aktiv"
Name: Schaltzustand [OUT1], Desc: Status abhängig von [OU1], Value: "aktiv"

## Vendor: 310, ProductName:KQ5100, Device:371
HexData: 0020
Name: Prozesswert, Desc: Abb. PDV1. Aktueller Prozesswert., Value: 2.0
Name: Schaltzustand [OUT1], Desc: Abb. BDC1. Zustand abhängig von Einstellungen für BDC1., Value: "inaktiv"



# TODO
1) Read Variables from Device before printing PDIN eg. Device could be set to Fahrenheit instead of Celsisus
2) Do CRC32 Check - if possible
3) Refactor code 
4) Github Actions
5) Some more Documentation