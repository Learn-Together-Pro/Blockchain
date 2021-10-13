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
    /*
    issue : https://github.com/Learn-Together-Pro/Blockchain/issues/21
    test : https://github.com/Learn-Together-Pro/Blockchain/blob/main/rust/blockchain/test/system_test.rs#L69
    complexity : difficult
    stage : mid
    */

    Self::new()
  }

  //

  pub fn MakePersistant() -> System
  {
    /*
    issue : https://github.com/Learn-Together-Pro/Blockchain/issues/12
    test : https://github.com/Learn-Together-Pro/Blockchain/blob/main/rust/blockchain/test/system_test.rs#L96
    complexity : easy
    stage : mid
    */

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
    test : https://github.com/Learn-Together-Pro/Blockchain/blob/main/rust/blockchain/test/system_test.rs#L124
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
    issue : https://github.com/Learn-Together-Pro/Blockchain/issues/13
    test : https://github.com/Learn-Together-Pro/Blockchain/blob/main/rust/blockchain/test/system_test.rs#L191
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
    issue : https://github.com/Learn-Together-Pro/Blockchain/issues/11
    test : https://github.com/Learn-Together-Pro/Blockchain/blob/main/rust/blockchain/test/system_test.rs#L256
    complexity : mid
    stage : mid
    */
  }
}
