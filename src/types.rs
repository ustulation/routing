// Copyright 2015 MaidSafe.net limited
//
// This MaidSafe Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the MaidSafe Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0, found in the root
// directory of this project at LICENSE, COPYING and CONTRIBUTOR respectively and also
// available at: http://www.maidsafe.net/licenses
//
// Unless required by applicable law or agreed to in writing, the MaidSafe Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS
// OF ANY KIND, either express or implied.
//
// See the Licences for the specific language governing permissions and limitations relating to
// use of the MaidSafe Software.

#![allow(unused_assignments)]
extern crate sodiumoxide;
extern crate rand;

use sodiumoxide::crypto;
use cbor::CborTagEncode;
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};

pub fn array_as_vector(arr: &[u8]) -> Vec<u8> {
  let mut vector = Vec::new();
  for i in arr.iter() {
    vector.push(*i);
  }
  vector
}

pub fn vector_as_u8_64_array(vector: Vec<u8>) -> [u8;64] {
  let mut arr = [0u8;64];
  for i in (0..64) {
    arr[i] = vector[i];
  }
  arr
}

pub fn vector_as_u8_32_array(vector: Vec<u8>) -> [u8;32] {
  let mut arr = [0u8;32];
  for i in (0..32) {
    arr[i] = vector[i];
  }
  arr
}

pub fn generate_random_vec_u8(size: usize) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::with_capacity(size);
    for i in 0..size {
        vec.push(rand::random::<u8>());
    }
    vec
}

pub static GROUP_SIZE: u32 = 23;
pub static QUORUM_SIZE: u32 = 19;

#[derive(PartialEq, Eq, Hash, Clone, RustcEncodable, RustcDecodable, PartialOrd, Ord, Debug)]
pub struct DhtId(pub Vec<u8>);

impl DhtId {
    pub fn new(slice: [u8; 64]) -> DhtId {
        unimplemented!();
    }

    pub fn generate_random() -> DhtId {
        DhtId(generate_random_vec_u8(64))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Authority {
  ClientManager,  // from a node in our range but not routing table
  NaeManager,     // Target (name()) is in the group we are in 
  NodeManager,    // recieved from a node in our routing table (Handle refresh here)
  ManagedNode,    // in our group and routing table
  ManagedClient,  // in our group
  Client,         // detached
  Unknown
}

impl Encodable for Authority {
  fn encode<E: Encoder>(&self, e: &mut E)->Result<(), E::Error> {
    let mut authority = "";
    match *self {
      Authority::ClientManager => authority = "ClientManager",
      Authority::NaeManager => authority = "NaeManager",
      Authority::NodeManager => authority = "NodeManager",
      Authority::ManagedNode => authority = "ManagedNode",
      Authority::ManagedClient => authority = "ManagedClient",
      Authority::Client => authority = "Client",
      Authority::Unknown => authority = "Unknown",
    };
    CborTagEncode::new(5483_100, &(&authority)).encode(e)
  }
}

impl Decodable for Authority {
  fn decode<D: Decoder>(d: &mut D)->Result<Authority, D::Error> {
    try!(d.read_u64());
    let mut authority : String = String::new();
    authority = try!(Decodable::decode(d));
    match &authority[..] {
      "ClientManager" => Ok(Authority::ClientManager),
      "NaeManager" => Ok(Authority::NaeManager),
      "NodeManager" => Ok(Authority::NodeManager),
      "ManagedNode" => Ok(Authority::ManagedNode),
      "ManagedClient" => Ok(Authority::ManagedClient),
      "Client" => Ok(Authority::Client),
      _ => Ok(Authority::Unknown)
    }
  }
}

pub type MessageId = u32;
pub type NodeAddress = DhtId; // (Address, NodeTag)
pub type GroupAddress = DhtId; // (Address, GroupTag)
pub type SerialisedMessage = Vec<u8>;
pub type CloseGroupDifference = (Vec<DhtId>, Vec<DhtId>);
pub type PmidNode = DhtId;
pub type PmidNodes = Vec<PmidNode>;

pub trait RoutingTrait {
  fn get_name(&self)->Vec<u8>;
  fn get_owner(&self)->Vec<u8>;
  fn refresh(&self)->bool;
  fn merge(&self, &Vec<AccountTransferInfo>) -> Option<AccountTransferInfo>;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct NameAndTypeId {
  pub name : Vec<u8>,
  pub type_id : u32
}

impl NameAndTypeId {
    pub fn generate_random() -> NameAndTypeId {
        NameAndTypeId {
            name: generate_random_vec_u8(64),
            type_id: rand::random::<u32>(),
        }
    }
}

impl Encodable for NameAndTypeId {
  fn encode<E: Encoder>(&self, e: &mut E)->Result<(), E::Error> {
    CborTagEncode::new(5483_000, &(&self.name, &self.type_id)).encode(e)
  }
}

impl Decodable for NameAndTypeId {
  fn decode<D: Decoder>(d: &mut D)->Result<NameAndTypeId, D::Error> {
    try!(d.read_u64());
    let (name, type_id) = try!(Decodable::decode(d));
    Ok(NameAndTypeId { name: name, type_id: type_id })
  }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Signature {
  pub signature : Vec<u8> // Vec form of crypto::sign::Signature which is an array
}

impl Signature {
  pub fn generate_random() -> Signature {
      Signature { signature: generate_random_vec_u8(64), }
  }

  pub fn get_signature(&self) -> crypto::sign::Signature {
    crypto::sign::Signature(vector_as_u8_64_array(self.signature.clone()))
  }
}

impl Encodable for Signature {
  fn encode<E: Encoder>(&self, e: &mut E)->Result<(), E::Error> {
    CborTagEncode::new(5483_000, &(&self.signature)).encode(e)
  }
}

impl Decodable for Signature {
  fn decode<D: Decoder>(d: &mut D)->Result<Signature, D::Error> {
    try!(d.read_u64());
    let signature = try!(Decodable::decode(d));
    Ok(Signature { signature: signature })
  }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct PublicKey {
  pub public_key : Vec<u8>
}

impl PublicKey {
  pub fn generate_random() -> PublicKey {
      let mut vec: Vec<u8> = Vec::with_capacity(64);
      vec.push_all(&(crypto::asymmetricbox::gen_keypair().0).0);
      PublicKey { public_key: vec, }
  }

  pub fn get_public_key(&self) -> crypto::sign::PublicKey {
    crypto::sign::PublicKey(vector_as_u8_32_array(self.public_key.clone()))
  }
}

impl Encodable for PublicKey {
  fn encode<E: Encoder>(&self, e: &mut E)->Result<(), E::Error> {
    CborTagEncode::new(5483_000, &(&self.public_key)).encode(e)
  }
}

impl Decodable for PublicKey {
  fn decode<D: Decoder>(d: &mut D)->Result<PublicKey, D::Error> {
    try!(d.read_u64());
    let public_key = try!(Decodable::decode(d));
    Ok(PublicKey { public_key: public_key })
  }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct PublicPmid {
  public_key: PublicKey,
  validation_token: Signature
}

impl PublicPmid {
    pub fn generate_random() -> PublicPmid {
        PublicPmid {
            public_key: PublicKey::generate_random(),
            validation_token: Signature::generate_random(),
        }
    }
}

impl Encodable for PublicPmid {
  fn encode<E: Encoder>(&self, e: &mut E)->Result<(), E::Error> {
    CborTagEncode::new(5483_001, &(&self.public_key, &self.validation_token)).encode(e)
  }
}

impl Decodable for PublicPmid {
  fn decode<D: Decoder>(d: &mut D)->Result<PublicPmid, D::Error> {
    try!(d.read_u64());
    let (public_key, validation_token) = try!(Decodable::decode(d));
    Ok(PublicPmid { public_key: public_key, validation_token: validation_token })
  }
}

// #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
// TODO (ben 2015-04-01) : implement order based on name
pub struct Pmid {
  public_keys: (crypto::sign::PublicKey, crypto::asymmetricbox::PublicKey),
  secret_keys: (crypto::sign::SecretKey, crypto::asymmetricbox::SecretKey),
  name: Vec<u8> // should really all be `DhtId`
}

impl RoutingTrait for Pmid {
  fn get_name(&self) -> Vec<u8> { self.name.clone() }
  fn get_owner(&self)->Vec<u8> { Vec::<u8>::new() } // TODO owner
  fn refresh(&self)->bool { false } // TODO is this an account transfer type

   // TODO how do we merge these
  fn merge(&self, _ : &Vec<AccountTransferInfo>) -> Option<AccountTransferInfo> { None }
}

impl Pmid {
  pub fn new() -> Pmid {
    let (pub_sign_key, sec_sign_key) = sodiumoxide::crypto::sign::gen_keypair();
    let (pub_asym_key, sec_asym_key) = sodiumoxide::crypto::asymmetricbox::gen_keypair();
    
    let sign_arr = &pub_sign_key.0;
    let asym_arr = &pub_asym_key.0;
    
    let mut arr_combined = [0u8; 64 * 2];

    for i in 0..sign_arr.len() {
        arr_combined[i] = sign_arr[i];
    }
    for i in 0..asym_arr.len() {
        arr_combined[64 + i] = asym_arr[i];
    }

    let digest = crypto::hash::sha512::hash(&arr_combined);

    Pmid {
      public_keys : (pub_sign_key, pub_asym_key),
      secret_keys : (sec_sign_key, sec_asym_key),
      name : digest.0.to_vec()
    }
  }

  pub fn get_public_key(&self) -> crypto::asymmetricbox::PublicKey {
    self.public_keys.1.clone()
  }
  pub fn get_secret_key(&self) -> crypto::asymmetricbox::SecretKey {
    self.secret_keys.1.clone()
  }
  pub fn get_public_sign_key(&self) -> crypto::sign::PublicKey {
    self.public_keys.0.clone()
  }
  pub fn get_secret_sign_key(&self) -> crypto::sign::SecretKey {
    self.secret_keys.0.clone()
  }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct EndPoint {
  pub ip_addr : Vec<u8>,
  pub socket : u32,
}

impl EndPoint {
    pub fn generate_random() -> EndPoint {
        EndPoint { ip_addr: generate_random_vec_u8(4), socket: rand::random::<u32>(), }
    }
}

impl Encodable for EndPoint {
  fn encode<E: Encoder>(&self, e: &mut E)->Result<(), E::Error> {
    CborTagEncode::new(5483_001, &(&self.ip_addr, &self.socket)).encode(e)
  }
}

impl Decodable for EndPoint {
  fn decode<D: Decoder>(d: &mut D)->Result<EndPoint, D::Error> {
    try!(d.read_u64());
    let (ip_addr, socket) = try!(Decodable::decode(d));
    Ok(EndPoint { ip_addr: ip_addr, socket: socket })
  }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct AccountTransferInfo {
  pub name : Vec<u8>
}

impl Encodable for AccountTransferInfo {
  fn encode<E: Encoder>(&self, e: &mut E)->Result<(), E::Error> {
    CborTagEncode::new(5483_000, &(&self.name)).encode(e)
  }
}

impl Decodable for AccountTransferInfo {
  fn decode<D: Decoder>(d: &mut D)->Result<AccountTransferInfo, D::Error> {
    try!(d.read_u64());
    let name = try!(Decodable::decode(d));
    Ok(AccountTransferInfo { name: name })
  }
}

impl RoutingTrait for AccountTransferInfo {
  fn get_name(&self)->Vec<u8> { self.name.clone() }
  fn get_owner(&self)->Vec<u8> { Vec::<u8>::new() } // TODO owner
  fn refresh(&self)->bool { true } // TODO is this an account transfer type

   // TODO how do we merge these
  fn merge(&self, _ : &Vec<AccountTransferInfo>) -> Option<AccountTransferInfo> { None }
}

/// Address of the source of the message
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct SourceAddress {
  pub from_node : DhtId,
  pub from_group : Option<DhtId>,
  pub reply_to : Option<DhtId>
}

impl SourceAddress {
    pub fn generate_random() -> SourceAddress {
        SourceAddress {
            from_node: DhtId::generate_random(),
            from_group: None,
            reply_to: None,
        }
    }
}

impl Encodable for SourceAddress {
  fn encode<E: Encoder>(&self, e: &mut E)->Result<(), E::Error> {
    CborTagEncode::new(5483_102 , &(&self.from_node, &self.from_group, &self.reply_to)).encode(e)
  }
}

impl Decodable for SourceAddress {
  fn decode<D: Decoder>(d: &mut D)->Result<SourceAddress, D::Error> {
    try!(d.read_u64());
    let (from_node, from_group, reply_to) = try!(Decodable::decode(d));
    Ok(SourceAddress { from_node: from_node, from_group: from_group, reply_to: reply_to })
  }
}

/// Address of the destination of the message
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct DestinationAddress {
  pub dest : DhtId,
  pub reply_to : Option<DhtId>
}

impl Encodable for DestinationAddress {
  fn encode<E: Encoder>(&self, e: &mut E)->Result<(), E::Error> {
    CborTagEncode::new(5483_101, &(&self.dest, &self.reply_to)).encode(e)
  }
}

impl Decodable for DestinationAddress {
  fn decode<D: Decoder>(d: &mut D)->Result<DestinationAddress, D::Error> {
    try!(d.read_u64());
    let (dest, reply_to) = try!(Decodable::decode(d));
    Ok(DestinationAddress { dest: dest, reply_to: reply_to })
  }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum MessageTypeTag {
  Connect,
  ConnectResponse,
  FindGroup,
  FindGroupResponse,
  GetData,
  GetDataResponse,
  GetClientKey,
  GetClientKeyResponse,
  GetGroupKey,
  GetGroupKeyResponse,
  Post,
  PostResponse,
  PutData,
  PutDataResponse,
  PutKey,
  AccountTransfer
}


#[cfg(test)]
#[allow(deprecated)]
mod test {
  extern crate cbor;
  use super::*;
  use std::rand;
  use rustc_serialize::{Decodable, Encodable};

  pub fn generate_address() -> Vec<u8> {
    let mut address: Vec<u8> = vec![];
    for _ in (0..64) {
      address.push(rand::random::<u8>());
    }
    address
  }

  fn test_object<T>(obj_before : T) where T: for<'a> Encodable + Decodable + Eq {
    let mut e = cbor::Encoder::from_memory();
    e.encode(&[&obj_before]).unwrap();
    let mut d = cbor::Decoder::from_bytes(e.as_bytes());
    let obj_after: T = d.decode().next().unwrap().unwrap();
    assert_eq!(obj_after == obj_before, true)
  }

  #[test]
  fn test_authority() {
    test_object(Authority::ClientManager);
    test_object(Authority::NaeManager);
    test_object(Authority::NodeManager);
    test_object(Authority::ManagedNode);
    test_object(Authority::Client);
    test_object(Authority::Unknown);
  }

  #[test]
  fn test_destination_address() {
    test_object(DestinationAddress { dest: DhtId::generate_random(), reply_to: None });
  }

  #[test]
  fn test_source_address() {
    test_object(SourceAddress { from_node : DhtId::generate_random(),
                                from_group : None,
                                reply_to: None });
  }

}
