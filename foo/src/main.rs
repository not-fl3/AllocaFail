extern crate rustc_serialize;
extern crate nalgebra;
extern crate bar;
extern crate toml;

use bar::*;
use nalgebra::*;

use std::collections::BTreeMap;
use rustc_serialize::Decodable;
use toml::*;

use std::marker::PhantomData;

pub trait Deserializers {
    fn visit(&self, toml : &BTreeMap<String, Value>);

    fn deserialize(&self, source : &str) {
        let toml = Parser::new(source).parse().unwrap();
        self.visit(&toml);
    }
}

pub struct DeserializersStorage<T : Decodable,
                            U : Deserializers> {
    name : String,
    marker : PhantomData<T>,
    next : U
}


impl<T : Decodable,
     U : Deserializers> Deserializers for DeserializersStorage<T, U> {
    fn visit(&self, source : &BTreeMap<String, Value>) {
        if let Some(value) = source.get(&self.name) {
            let c = decode::<T>(value.clone());
            match c {
                Some(component) => {},
                None => println!("{} deserialization failed", self.name)
            }
        }
        self.next.visit(source);
    }
}

impl Deserializers for () {
    fn visit(&self, _ :  &BTreeMap<String, Value>) {
    }
}

impl<T : Decodable> DeserializersStorage<T, ()> {
    pub fn new(name : String, _ : PhantomData<T>) ->
        DeserializersStorage<T, ()> {
            DeserializersStorage {
                name   : name,
                marker : PhantomData,
                next   : ()
            }
        }
}

impl<T : Decodable,
     U : Deserializers> DeserializersStorage<T, U> {
    pub fn add<T1 : Decodable>(self, name : String) ->
        DeserializersStorage<T1, DeserializersStorage<T, U>> {
            DeserializersStorage {
                name : name,
                marker : PhantomData,
                next : self
            }
        }
}

#[macro_export]
macro_rules! deserializers {
    ($t:ty) => {
        DeserializersStorage::new(stringify!($t).to_string(), ::std::marker::PhantomData::<$t>)
    };
    ($t1:ty, $($t:ty),+) => {
        {
            let deserializers = DeserializersStorage::new(stringify!($t1).to_string(), ::std::marker::PhantomData::<$t1>);
            $(
                let deserializers = deserializers.add::<$t>(stringify!($t).to_string());
            )+
            let deserializers : Box<Deserializers> = Box::new(deserializers);
            deserializers
        }
    };
}



fn main() {
    deserializers!(Mesh, Position, Velocity);
}
