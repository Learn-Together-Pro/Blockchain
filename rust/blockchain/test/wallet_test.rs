
#![ allow( non_snake_case ) ]

use std::collections::HashMap;
use lt_blockchain::blockchain::{ system, wallet };

//

#[ test ]
#[ ignore ]
fn new()
{
  println!( "add wallet to empty map" );
  let mut wallets_map : HashMap<String, wallet::Wallet> = HashMap::new();
  let name = String::from( "user" );
  let got = wallet::Wallet::new( &mut wallets_map, &name );
  assert!( got.is_some() );
  let wallet = got.unwrap();
  assert_eq!( wallet.name, name.clone() );
  assert_eq!( wallet.public_key.len(), 270 );
  assert!( wallet.private_key.len() >= 1100 );

  println!( "add two different wallets" );
  let mut wallets_map : HashMap<String, wallet::Wallet> = HashMap::new();
  let name = String::from( "user" );
  let got = wallet::Wallet::new( &mut wallets_map, &name );
  assert!( got.is_some() );
  let wallet = got.unwrap();
  assert_eq!( wallet.name, name.clone() );
  assert_eq!( wallet.public_key.len(), 270 );
  assert!( wallet.private_key.len() >= 1100 );
  let name = String::from( "my" );
  let got = wallet::Wallet::new( &mut wallets_map, &name );
  assert!( got.is_some() );
  let wallet = got.unwrap();
  assert_eq!( wallet.name, name.clone() );
  assert_eq!( wallet.public_key.len(), 270 );
  assert!( wallet.private_key.len() >= 1100 );

  println!( "add two same wallets" );
  let mut wallets_map : HashMap<String, wallet::Wallet> = HashMap::new();
  let name = String::from( "user" );
  let got = wallet::Wallet::new( &mut wallets_map, &name );
  assert!( got.is_some() );
  let wallet = got.unwrap();
  assert_eq!( wallet.name, name.clone() );
  assert_eq!( wallet.public_key.len(), 270 );
  assert!( wallet.private_key.len() >= 1100 );
  let got = wallet::Wallet::new( &mut wallets_map, &name );
  assert!( got.is_none() );
}

//

#[ ignore ]
#[ test ]
fn keys_pair_generate()
{
  println!( "generate pair of keys" );
  let ( public_key, private_key ) = wallet::Wallet::keys_pair_generate();
  assert_eq!( public_key.len(), 270 );
  assert!( private_key.len() >= 1100 );
}

//

#[ test ]
#[ ignore ]
fn wallet_create()
{
  let sys = system::System::new();

  /* */

  println!( "add wallet to empty map" );
  let mut system = sys.clone();
  let name = String::from( "user" );
  let got = system.wallet_create( &name );
  assert!( got.is_some() );
  let wallet = got.unwrap();
  assert_eq!( wallet.name, name.clone() );
  assert_eq!( wallet.public_key.len(), 270 );
  assert!( wallet.private_key.len() >= 1100 );

  println!( "add two different wallets" );
  let mut system = sys.clone();
  let name = String::from( "user" );
  let got = system.wallet_create( &name );
  assert!( got.is_some() );
  let wallet = got.unwrap();
  assert_eq!( wallet.name, name.clone() );
  assert_eq!( wallet.public_key.len(), 270 );
  assert!( wallet.private_key.len() >= 1100 );
  let name = String::from( "my" );
  let got = system.wallet_create( &name );
  assert!( got.is_some() );
  let wallet = got.unwrap();
  assert_eq!( wallet.name, name.clone() );
  assert_eq!( wallet.public_key.len(), 270 );
  assert!( wallet.private_key.len() >= 1100 );

  println!( "add two same wallets" );
  let mut system = sys.clone();
  let name = String::from( "user" );
  let got = system.wallet_create( &name );
  assert!( got.is_some() );
  let wallet = got.unwrap();
  assert_eq!( wallet.name, name.clone() );
  assert_eq!( wallet.public_key.len(), 270 );
  assert!( wallet.private_key.len() >= 1100 );
  let got = system.wallet_create( &name );
  assert!( got.is_none() );
}

