#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::str;


use rand::rngs::OsRng;
use rsa::pkcs1::{ToRsaPrivateKey, ToRsaPublicKey};
use rsa::{RsaPrivateKey};

use super::digest::*;
use super::system::*;

//

#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub struct Wallet
{
  pub name : String,
  pub public_key : Digest,
  pub private_key : Digest,
}

//

impl Wallet
{

  //

  pub fn new< 'a, 'b >( _wallets : &'a mut HashMap< String, Wallet >, _name : &'b String ) -> Option< &'a Wallet >
  {
    None
    /*
    issue : https://github.com/Learn-Together-Pro/Blockchain/issues/4
    complexity : mid
    stage : late
    */
  }

  //

  pub fn keys_pair_generate() -> ( Digest, Digest )
  {
    /*
    issue : https://github.com/Learn-Together-Pro/Blockchain/issues/3
    complexity : mid
    stage : mid
    */

    let mut rng = OsRng;
    let bits = 2048;

    let private_key = RsaPrivateKey::new(&mut rng, bits)
    .expect("failed to generate private key");
    let pub_key = RsaPrivateKey::to_public_key(&private_key).to_pkcs1_pem()
    .expect("failed to generate public key");
    let private_key_pem = private_key.to_pkcs1_pem()
    .expect("failed to convert private to pem");
    ( Digest::from(private_key_pem.as_bytes().to_vec()), Digest::from( pub_key.as_bytes().to_vec() ))


  }

  //

}

//

impl System
{

  //

  pub fn wallet_create( &mut self, name : &String ) -> Option< &Wallet >
  {
    Wallet::new( &mut self.wallets, name )
  }

  //

}
