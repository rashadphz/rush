use indexmap::IndexMap;

use super::{
    descriptor::Descriptor,
    primary::{RshObject, Value},
};

#[derive(Debug)]
pub struct DataDict {
    dict: IndexMap<String, Value>,
}

impl DataDict {
    pub fn default() -> Self {
        DataDict {
            dict: IndexMap::default(),
        }
    }

    pub fn insert(&mut self, name: impl Into<String>, value: Value) {
        self.dict.insert(name.into(), value);
    }
}

impl RshObject for DataDict {
    fn data_descriptors(&self) -> Vec<Descriptor> {
        self.dict
            .iter()
            .map(|(name, _)| Descriptor::new(name))
            .collect()
    }

    fn get_data(&self, desc: &Descriptor) -> &Value {
        match self.dict.get(&desc.name) {
            Some(val) => val,
            None => panic!(),
        }
    }
}
