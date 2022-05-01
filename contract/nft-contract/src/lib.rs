use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet, Vector};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, CryptoHash, 
    PanicOnDefault, Promise, PromiseOrValue, ext_contract, Gas
};
use near_sdk::utils::is_promise_success;

use near_helper::{near_to_yoctonear, expect_lightweight, yoctonear_to_near};

use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::approval::*;
pub use crate::royalty::*;
pub use crate::events::*;

mod approval; 
mod enumeration; 
mod internal;
mod metadata; 
mod mint; 
mod nft_core; 
mod royalty; 
mod events;

pub const NFT_METADATA_SPEC: &str = "1.0.0";
pub const NFT_STANDARD_NAME: &str = "nep171";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // Contract owner
    pub owner_id: AccountId,

    // Keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    // keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,
    
    // keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    // keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,

    /// Vector of the categories
    pub categories: Vector<Category>,

    /// Each owner map to a category, to keep track of what tokens
    /// they already have, in order. Key is a number (signifying
    /// position in `categories` mapped to TokenId)
    pub tokens_per_owner_ordered: LookupMap<AccountId, HashMap<CategoryId, TokenId>>,

    /// List of token metadata creator
    pub token_metadata_by_cat_id: UnorderedMap<CategoryId, TokenMetadata>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
    Categories,
    TokensOrdered,
    MetadataCatId,
}

#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        this initializes the contract with default metadata so the
        user doesn't have to manually type metadata.
    */
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        // calls the other function "new" with some default metadata and
        // the owner_id passed in. 

        Self::new(
          owner_id,
          NFTContractMetadata {
            spec: "nft-1.0.0".to_string(),
            name: "NFT Tutorial Contract".to_string(),
            symbol: "GONEAR".to_string(),
            icon: None,
            base_uri: None,
            reference: None,
            reference_hash: None,
          }
        )
    }

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id. All other collections will default empty. 
    */
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        // create a variable of type Self with all the fields initialized. 
        let this = Self {
          // Storage keys are simply the prefixes used for the collection.
          // THis helps avoid data collision. 
          tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
          tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
          token_metadata_by_id: UnorderedMap::new(
            StorageKey::TokenMetadataById.try_to_vec().unwrap(),
          ),

          // set owner_id field equal to the passed in owner_id
          owner_id,
          metadata: LazyOption::new(
            StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
            Some(&metadata),  // if extra passed in
          ),

          categories: Vector::new(StorageKey::Categories.try_to_vec().unwrap()),
          tokens_per_owner_ordered: LookupMap::new(
            StorageKey::TokensOrdered.try_to_vec().unwrap()
          ),

          token_metadata_by_cat_id: UnorderedMap::new(
            StorageKey::MetadataCatId.try_to_vec().unwrap()
          ),
        };

        // return the contract object
        this
    }


    pub fn get_owner_donation(&self, account_id: AccountId) -> HashMap<String, String> {
      if let Some(hashmap) = self.tokens_per_owner_ordered.get(&account_id) {
        let mut return_map = HashMap::new();

        for (id, token_id) in hashmap {
          let metadata = expect_lightweight(
            self.token_metadata_by_cat_id.get(&id),
            "Found category but not its metadata. Maybe forgot to map?"
          );
          let title = metadata.title.unwrap();

          let token = expect_lightweight(
            self.tokens_by_id.get(&token_id),
            "Cannot find this token in library"
          );
          let donate_amount = token.donate_amount;

          return_map.insert(title, donate_amount);
        }

        return_map  // donate amount stringified.
      } else {
        HashMap::new()
      }
    }


    pub fn get_categories(&self) -> Vec<Category> {
      self.categories.to_vec()
    }


    pub fn view_metadatas(&self) -> Vec<TokenMetadata> {
      self.token_metadata_by_cat_id.values_as_vector().to_vec()
    }


    pub fn get_list_to_donate(&self) -> HashMap<String, String> {
      let mut temp = HashMap::new();

      for (k, v) in self.token_metadata_by_cat_id.iter() {
        let id = self.categories.get(k as u64).unwrap();
        let title = v.title.unwrap();
        temp.insert(id, title);
      }

      temp
    }


    pub fn get_id_by_category(&self) -> HashMap<String, u16> {
      let mut temp = HashMap::new();

      for (k, _v) in self.token_metadata_by_cat_id.iter() {
        let id = self.categories.get(k.clone() as u64).unwrap();
        temp.insert(id, k);
      }

      temp
    }
}