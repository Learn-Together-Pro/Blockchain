#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::str;

use rsa::RsaPublicKey;
use rsa::pkcs1::{ToRsaPrivateKey, ToRsaPublicKey};
use rsa::{PublicKey, RsaPrivateKey, PaddingScheme};
use rand::rngs::OsRng;

use super::digest::*;
use super::system::*;

//

#[ derive( Debug, Clone, Serialize, Deserialize ) ]
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

  fn keys_pair_generate() -> ( Digest, Digest )
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
    let pub_key = RsaPublicKey::from(&private_key).to_pkcs1_pem()
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
