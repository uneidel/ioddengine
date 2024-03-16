use yaserde::*;


#[derive(YaDeserialize,Default, Debug)]
#[yaserde(rename_all = "PascalCase")]
pub struct CommNetworkProfile{

    #[yaserde(rename = "xsi:type", attribute)]
    pub commtype : String,
    #[yaserde(rename = "iolinkRevision", attribute)]
    pub iolink_revision : String,
    #[yaserde(rename = "TransportLayers")]
    pub transport_layers : TransportLayers
  

}

#[derive(YaDeserialize,Default, Debug)]
#[yaserde(rename_all = "PascalCase")]
pub struct TransportLayers{
    #[yaserde(rename = "PhysicalLayer")]
    pub physical_layer : PhysicalLayer
}

#[derive(YaDeserialize,Default, Debug)]
#[yaserde(rename_all = "PascalCase")]
pub struct PhysicalLayer{
    #[yaserde(rename = "physics", attribute)]
    pub physics : String,
    #[yaserde(rename = "minCycleTime",attribute)]
    pub mincycletime :u32,
    #[yaserde(rename = "sioSupported",attribute)]
    pub siosupported : bool,
    #[yaserde(rename = "baudrate",attribute)]
    pub baudrate : String 

}