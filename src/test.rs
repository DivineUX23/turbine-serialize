use borsh::{BorshSerialize, BorshDeserialize};
use wincode::{SchemaRead, SchemaWrite};
use serde::{Deserialize, Serialize};
use crate::{Serializer, Storage, Borsh, Json, Wincode};


#[cfg(test)]
mod tests {


use super::*;

    #[derive(BorshSerialize, BorshDeserialize, SchemaWrite, SchemaRead, Serialize, Deserialize, Debug, PartialEq, Clone)]
    pub struct Person {
        pub name: String,
        pub age: u64
    }


    fn test<S>(serializer: S) where for<'de> S: Serializer<'de, Person> {
        let data = Person {
            name: String::from("Turbine"),
            age: 120
        };

        let mut storage = Storage::new(serializer);
        assert!(!storage.has_data(), "Storage not empty");

        storage.save(&data).unwrap();
        assert!(storage.has_data(), "Storage empty");

        let serializer = Wincode;

        let return_changed = storage.change(serializer).unwrap();
        assert_eq!(return_changed, data, "Wrong data");

        let return_data = storage.load().unwrap();
        assert_eq!(return_data, data, "Wrong data");
    }


    #[test]
    fn test_borsh(){
        test(Borsh);
    }

    #[test]
    fn test_wincode(){
        test(Wincode);
    }

    #[test]
    fn test_json(){
        test(Json);
    }
}