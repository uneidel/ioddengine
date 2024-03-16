

use crate::ioddmodel11::ProfileBody::VariableCollection::*;
use crate::ioddmodel11::ExternalTextCollection::*;

pub struct Definitions{

    pub variablecollection : VariableCollection,
    //pub errorcollection : Option<None>
    //pub eventcollection:
    pub externaltextcollection : Vec<Language>
}