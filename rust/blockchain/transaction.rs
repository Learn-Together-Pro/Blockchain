#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use serde::de::*;
use serde::ser::*;
use std::borrow::{Borrow, BorrowMut};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::num::ParseIntError;
use wtools as wt;

use super::chain::*;
use super::digest::*;
use super::system::*;
use super::wallet::*;

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionGeneric<T: ?Sized> {
    pub sender: Digest,
    #[serde(
        serialize_with = "ordered_map_serialize",
        deserialize_with = "hash_map_deserialize"
    )]
    pub receiver: HashMap<Digest, f64>,
    pub amount: f64,
    pub time: i64,
    pub body: T,
}

fn ordered_map_serialize<S>(value: &HashMap<Digest, f64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut ordered: BTreeMap<String, f64> = BTreeMap::new();
    for (k, v) in value {
        ordered.insert(bytes_to_string_hex(k), *v);
    }
    ordered.serialize(serializer)
}

fn hash_map_deserialize<'de, D>(deserializer: D) -> Result<HashMap<Digest, f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf: BTreeMap<String, f64> = BTreeMap::deserialize(deserializer)?;

    let mut hash_map: HashMap<Digest, f64> = HashMap::new();
    for (k, v) in buf {
        hash_map.insert(
            Digest::from(
                (0..k.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&k[i..i + 2], 16))
                    .collect::<Result<Vec<u8>, ParseIntError>>()
                    .unwrap(),
            ),
            v,
        );
    }
    Ok(hash_map)
}
pub type TransactionHeader = TransactionGeneric<()>;

//

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionRestricts {
    pub hash: Digest,
}

pub type Transaction = TransactionGeneric<TransactionRestricts>;

//

impl TransactionHeader {
    pub fn form(&self) -> Transaction {
        Transaction::new(self)
    }
}

//

impl Transaction {
    pub fn new(body: &TransactionHeader) -> Transaction {
        let hash = hash_single(&body);
        Transaction {
            sender: body.sender.clone(),
            receiver: body.receiver.clone(),
            amount: body.amount,
            time: body.time,
            body: TransactionRestricts { hash },
        }
    }

    //

    pub fn header(&self) -> &TransactionHeader {
        self.borrow()
    }

    //

    pub fn header_mut(&mut self) -> &mut TransactionHeader {
        self.borrow_mut()
    }

    //

    pub fn valid_is(&self) -> bool {
        if hash_single(&self.header()) != self.body.hash {
            return false;
        }
        true
    }
}

//

impl Borrow<TransactionHeader> for Transaction {
    fn borrow<'a>(&'a self) -> &'a TransactionHeader {
        // we want zero-copy. instead of making a copy we reinterpret the larget structure containg smaller
        unsafe { std::mem::transmute::<&'a Transaction, &'a TransactionHeader>(self) }
    }
}

//

impl BorrowMut<TransactionHeader> for Transaction {
    fn borrow_mut<'a>(&'a mut self) -> &'a mut TransactionHeader {
        // we want zero-copy. instead of making a copy we reinterpret the larget structure containg smaller
        unsafe { std::mem::transmute::<&'a mut Transaction, &'a mut TransactionHeader>(self) }
    }
}

//

impl System {
    //

    pub fn transaction_create(&mut self, sender: String, receiver: String, amount: f64) {
        let sender_digest = Digest::from(Vec::from(sender.as_bytes()));
        let receiver_digest = Digest::from(Vec::from(receiver.as_bytes()));
        self.chain
            .transaction_create(sender_digest, receiver_digest, amount);
    }

    //
}

//

impl Chain {
    //

    pub fn transaction_create(&mut self, _sender: Digest, _receiver: Digest, _amount: f64) {
        /*
        issue : https://github.com/Learn-Together-Pro/Blockchain/issues/7
        test : https://github.com/Learn-Together-Pro/Blockchain/blob/main/rust/blockchain/test/transaction_test.rs#L106
        complexity : difficult
        stage : early
        */
        unimplemented!("not implemented");
    }

    //

    pub fn balance_get(&self, public_key: &Digest) -> f64 {
        /*
        issue : https://github.com/Learn-Together-Pro/Blockchain/issues/1
        test : https://github.com/Learn-Together-Pro/Blockchain/blob/main/rust/blockchain/test/transaction_test.rs#L187
        complexity : difficult
        stage : late
        */

        let mut balance_of_the_searched_user: f64 = 0.0;

        let potential_transaction = self
            .transactions_pool
            .clone()
            .into_iter()
            .filter(|(_digest, transaction)| {
                transaction.receiver.get(public_key).is_some() && &transaction.sender == public_key
            })
            .nth(0);

        match potential_transaction {
            Some((_digest, transaction)) => {
                balance_of_the_searched_user += transaction.receiver.values().sum::<f64>();
                return balance_of_the_searched_user;
            }
            None => (),
        }

        for block in self.blocks.iter() {
            let potential_transaction: Option<&TransactionGeneric<TransactionRestricts>> = block
                .body
                .transactions
                .iter()
                .filter(|transaction| {
                    &transaction.sender == public_key
                        && transaction.receiver.get(public_key).is_some()
                })
                .nth(0);

            match potential_transaction {
                Some(transaction) => {
                    balance_of_the_searched_user += transaction.receiver.values().sum::<f64>();
                    return balance_of_the_searched_user;
                }
                None => (),
            }
        }

        balance_of_the_searched_user
    }

    //
}
