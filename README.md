# IODD Engine 

This repository houses the implementation of an IoDD Engine, featuring a parser command-line interface (CLI) and a catalog system. The IoDD Engine is designed to facilitate seamless interaction and management of digital devices within interconnected networks. 

It also has a implementation of a Client for IFM IoTCore.

The parser CLI offers a user-friendly interface for executing commands and queries, enabling efficient communication with the IoDD Engine.
This Engine only targets Version 1.1 of IODD.

[IODD Specs](https://io-link.com/share/Downloads/Spec-IODD/IO-Device-Desc-Spec_10012_V113_Feb24.zip)


Catalog:
    - Store Information about VendorId, DeviceId and Productname and the correspoding IODD Zip File.
    - Please note: Due to License restriction you need to manually download missing IODD Zip Files from [IODD-FInder](https://ioddfinder.io-link.com/) and place it into /data/download.

Engine:
    - allows to parse PDIn and settings of variables. ObserverMenu and ParameterMenu are implemented.

Cli:
    - allows cli based interactions with IODD Sensor

IoTCoreConnect: 
    - Client for interaction with IFM IotCore


This Code is licensed under Apache 2.0.

Due to potential License Issues you need to download the [Standard Definitions](https://io-link.com/share/Downloads/Spec-IODD/IO-Device-Desc-Spec_10012_V113_Feb24.zip) manually in place it into /data/specs.

Please see CLI for further information how to work with Library.

Tested with the following IFM Devices:
- 05D
- LMT121
- LR3300
- PN7592
- TN2405
- KQ5100


| Company  | deviceid   |  productname  | PDIN  | Write  |
|----------|------------|---------------|-------|--------|
|   IFM    |            |  05d150       |   X   |        |
|   IFM    |            |  LMT121       |   X   |        |
|   IFM    |            |      LR3300   |   X   |        |
|   IFM    |            |      PN7592   |   X   |        |
|   IFM    |            |      TN2405   |   X   |        |
|   IFM    |            |      KQ5100   |   X   |        |
|   AUTOSEN|            |      AL009    |   X   |        |
|   AUTOSEN|            |      AS001    |   X   |        |


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