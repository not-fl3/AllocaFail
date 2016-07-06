extern crate rustc_serialize;
extern crate nalgebra;
//extern crate test;

//use test::*;
use nalgebra::*;

use rustc_serialize::*;

#[derive(Debug, RustcDecodable)]
pub struct Collider {
    pub collider_mesh     : String,
    pub collider_location : [f32; 3],
    pub collider_rotation : [f32; 3],
    pub collider_scale    : [f32; 3]
}

#[derive(Debug, RustcDecodable)]
pub struct Mesh {
    pub is_dynamic : bool,
    pub is_visual  : bool,
    pub mesh       : String,
    pub density    : f32,
    pub name       : String,
    pub colliders  : Vec<Collider>

}

#[derive(Debug, RustcDecodable)]
pub struct Position {
    pub pos    : Vector3<f32>,
    pub look   : Vector3<f32>,
}

#[derive(RustcDecodable)]
pub struct Velocity {
    pub speed  : f32,
    pub strafe : f32
}


