
#![ allow( non_snake_case ) ]

use serde_derive::*;
use std::collections::HashMap;
use lt_blockchain::blockchain::{ digest, transaction };

//

#[ test ]
fn new()
{
  println!( "check constructor" );
  let got = digest::Digest::new();
  assert_eq!( got.len(), 0 );
  let exp : Vec<u8> = vec![];
  assert_eq!( got.to_vec(), exp );
}

//

#[ test ]
fn from()
{
  println!( "from Vec<u8>" );
  let src = Vec::from( "abc".as_bytes() );
  let got = digest::Digest::from( src );
  assert_eq!( got.len(), 3 );
  assert_eq!( got.to_vec(),  vec![ 97, 98, 99 ] );
}

//

#[ test ]
#[ ignore ]
fn hash_single()
{
  #[ derive( Clone, Serialize, Deserialize ) ]
  struct A
  {
    f1 : u8,
    f2 : u8,
  }

  #[ derive( Clone, Serialize, Deserialize ) ]
  struct B
  {
    a : u8,
    b : u8,
    c : String,
  }

  /* */

  println!( "hash serializable struct" );
  let src = A { f1 : 1, f2 : 2 };
  let got = digest::hash_single( &src );
  assert_eq!( got.len(), 32 );
  assert_eq!( digest::bytes_to_string_hex( &got ), "9131e60c31533f813d5edff6ebb75c2ebba0f130e121e5aa63e6f7db2be5adfe" );

  println!( "hash struct twice" );
  let src = A { f1 : 1, f2 : 2 };
  let got = digest::hash_single( &src );
  assert_eq!( got.len(), 32 );
  assert_eq!( digest::bytes_to_string_hex( &got ), "9131e60c31533f813d5edff6ebb75c2ebba0f130e121e5aa63e6f7db2be5adfe" );
  let got = digest::hash_single( &src );
  assert_eq!( got.len(), 32 );
  assert_eq!( digest::bytes_to_string_hex( &got ), "9131e60c31533f813d5edff6ebb75c2ebba0f130e121e5aa63e6f7db2be5adfe" );

  println!( "check length of hashes from different structures" );
  let src = A { f1 : 1, f2 : 2 };
  let got = digest::hash_single( &src );
  assert_eq!( got.len(), 32 );
  let src = B { a : 1, b : 2, c : String::from( "abc" ) };
  let got = digest::hash_single( &src );
  assert_eq!( got.len(), 32 );
}

//

#[ test ]
#[ ignore ]
fn hash_every()
{
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  struct A
  {
    f1 : u8,
    f2 : u8,
  }

  /* */

  println!( "hash single item, should be identical to hash_single" );
  let src = A { f1 : 1, f2 : 2 };
  let got = digest::hash_every( &vec![ src.clone() ] );
  assert_eq!( got.len(), 32 );
  let exp = digest::hash_single( &src.clone() );
  assert_eq!( got, exp );

  println!( "hash several items" );
  let src = A { f1 : 1, f2 : 2 };
  let got = digest::hash_every( &vec![ src.clone(), src.clone() ] );
  assert_eq!( got.len(), 32 );
  let exp = digest::hash_single( &src.clone() );
  assert_ne!( &got, &exp );
  assert_eq!( digest::bytes_to_string_hex( &got ), "bd6eb60738e54d64f94e07f0dd906a548daa7a00521afeff905fce49aede2d1b" );
}

//

#[ test ]
#[ ignore ]
fn bytes_to_string_hex()
{
  println!( "convert vector of bytes to string" );
  let src = vec![ 5, 23, 255, 143, 64, 128 ];
  let got = digest::bytes_to_string_hex( &src );
  assert_eq!( got, String::from( "0517ff8f4080" ) );
}

//

#[ test ]
#[ ignore ]
fn merkle_calc()
{
  fn transaction_empty_create() -> transaction::Transaction
  {
    let transaction_header = transaction::TransactionHeader
    {
      sender : digest::Digest::new(),
      receiver : HashMap::new(),
      amount : 1.0,
      time : 100,
      body : ()
    };
    transaction_header.form()
  }

  /* */

  println!( "calculate single hash" );
  let transaction = transaction_empty_create();
  let src = vec![ transaction.clone() ];
  let got = digest::merkle_calc( &src );
  assert_eq!( got.len(), 32 );
  let exp = digest::hash_single( &transaction.body.hash.clone() );
  assert_eq!( got, exp );

  println!( "calculate several hashes" );
  let transaction = transaction_empty_create();
  let src = vec![ transaction.clone(), transaction.clone() ];
  let got = digest::merkle_calc( &src );
  assert_eq!( got.len(), 32 );
  let exp = digest::hash_single( &transaction.body.hash.clone() );
  assert_ne!( got, exp );
}

