use std::io::{Error, ErrorKind};
use borsh::{BorshSerialize, BorshDeserialize};
use wincode::{SchemaRead, SchemaWrite};
use wincode::config::DefaultConfig;
use serde::{Deserialize, Serialize};

use std::marker::PhantomData;

pub mod test;

pub trait Serializer<'de, T> {
    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Error>;
    fn from_bytes(&self, bytes: &'de [u8]) -> Result<T, Error>;
}

//#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct Borsh;

impl<'de, T> Serializer<'de, T> for Borsh where T: BorshSerialize + BorshDeserialize {

    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Error> {
        borsh::to_vec(data).map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
    }

    fn from_bytes(&self, bytes: &[u8]) -> Result<T, Error> {
        borsh::from_slice(bytes).map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
    }

}

//#[derive(SchemaWrite, SchemaRead, PartialEq, Debug)]
pub struct Wincode;

impl<'de, T> Serializer<'de, T> for Wincode 
where 
    T: SchemaWrite<DefaultConfig, Src=T> + SchemaRead<'de, DefaultConfig, Dst=T>{

    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Error> {
        wincode::serialize(data).map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
    }

    fn from_bytes(&self, bytes: &'de [u8]) -> Result<T, Error> {
        wincode::deserialize(bytes).map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
    }

}

//#[derive(Serialize, Deserialize, Debug)]
pub struct Json;

impl<'de, T> Serializer<'de, T> for Json where T: Serialize + Deserialize<'de> {

    fn to_bytes(&self, data: &T) -> Result<Vec<u8>, Error> {
        serde_json::to_vec(data).map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
    }

    fn from_bytes(&self, bytes: &'de [u8]) -> Result<T, Error> {
        serde_json::from_slice(bytes).map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
    }

}



pub trait AllFormats: 
    BorshSerialize +BorshDeserialize +
    SchemaWrite<DefaultConfig> + for<'a> SchemaRead<'a, DefaultConfig> +
    Serialize + for<'a> Deserialize<'a>
 {}

 impl <T> AllFormats for T where 
    T:  BorshSerialize +BorshDeserialize +
        SchemaWrite<DefaultConfig> + for<'a> SchemaRead<'a, DefaultConfig> +
        Serialize + for<'a> Deserialize<'a>
    {}


pub struct Storage<T, S> 
where 
    T: AllFormats, 
    for<'de> S: Serializer<'de, T> 
{
    pub data: Vec<u8>,
    pub serializer: S,
    _phantom: PhantomData<T>
}


impl <T, S> Storage<T, S> 
where 
    T: AllFormats, 
    for<'de> S: Serializer<'de, T> {

    pub fn new(serialize: S) -> Self {
        Self {
            data: Vec::new(), 
            serializer: serialize, 
            _phantom: PhantomData 
        }
    }

    pub fn save(&mut self, value: &T) -> Result<(), Error> {
        let bytes = self.serializer.to_bytes(value)?;
        self.data = bytes;
        Ok(())
    }

    pub fn change<NewS>(&mut self, new_serializer: S) -> Result<T, Error>
    where T: AllFormats,
        for<'de> NewS: Serializer<'de, T>  {

        self.serializer = new_serializer;
        self.serializer.from_bytes(&self.data)
    }

    pub fn load(&self) -> Result<T, Error> {

        self.serializer.from_bytes(&self.data)
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
    }

    pub fn has_data(&self) -> bool {
        !self.data.is_empty()
    }

}



fn main() {
    println!("Hello, world!");
}