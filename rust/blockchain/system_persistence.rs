#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use std::fs;
use std::process;
use std::env;
use std::path::PathBuf;
use anyhow::Result;
use serde_json;

use super::system::*;
use std::fs::File;
use std::io::Write;

impl System
{

  //

  pub fn Make() -> System
  {
    Self::new()
  }

  //

  pub fn MakePersistant() -> System
  {
    Self::new()
    /*
    issue : https://github.com/Learn-Together-Pro/Blockchain/issues/12
    complexity : easy
    stage : mid
    */
  }

  //

  pub fn StorePathDefault() -> PathBuf
  {
    PathBuf::new()
    /*
    issue : https://github.com/Learn-Together-Pro/Blockchain/issues/16
    complexity : mid
    stage : early
    */
  }

  //

  pub fn Load() -> Result< System >
  {
    let path = Self::StorePathDefault();
    Self::LoadFromFile( &path )
  }

  //

  pub fn LoadFromFile( _path : &PathBuf ) -> Result< System >
  {
    /*
    Issue : https://github.com/Learn-Together-Pro/Blockchain/issues/3
    complexity : mid
    stage : mid
    */
    Ok( System::new() )
  }

  //

  pub fn store( &self )
  {
    self.store_to( &self.store_path );
  }

  //

  pub fn store_to( &self, path : &PathBuf )
  {
    let data = serde_json::to_string(self);
    if data.is_err() {
      eprintln!("Can not format System as JSON: {:?}", data.unwrap_err());
      return;
    }
    let file = File::create(path);
    if file.is_err() {
      eprintln!("Can not create file: {:?}", file.unwrap_err());
      return;
    }
    let mut file = file.unwrap();
    let res = file.write_all(data.unwrap().as_bytes());
    if res.is_err() {
      eprintln!("Can not write to the file: {:?}", res.unwrap_err());
    }
  }

  //

}
