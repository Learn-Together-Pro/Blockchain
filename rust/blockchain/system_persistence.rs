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
    let sys = Self::new();
    sys.store();
    sys
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

  pub fn store_to( &self, _path : &PathBuf )
  {
    /*
    Issue : https://github.com/Learn-Together-Pro/Blockchain/issues/11
    complexity : mid
    stage : mid
    */
  }

  //

}
