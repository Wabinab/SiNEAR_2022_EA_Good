use crate::*;
use near_sdk::require;


const GAS_PER_DONATE: Gas = Gas(20_000_000_000_000);
const GAS_FOR_MINTING: Gas = Gas(10_000_000_000_000);  // 10 TGas.


#[ext_contract(ext_self)]
trait ExtSelf {
    fn nft_mint(
      &mut self,
      token_id:TokenId,
      donate_amount_str: String, 
      metadata: TokenMetadata,
      receiver_id: AccountId,
      perpetual_royalties: Option<HashMap<AccountId, u16>>,
    );

    fn on_donate_update(
      &mut self,
      token_id: TokenId,
      old_donate_amount: f64,
    );

    /// These two functions can be called by non-contract. 
    /// We use callback because another function needs to call
    /// them via a callback. 
    fn donate_and_update(
      &mut self,
      token_id: TokenId,
      donate_amount: f64,
    );

    fn donate_and_mint(
      &mut self,
      token_id: TokenId,
      metadata: TokenMetadata,
      donate_amount: f64,
    );
}

#[near_bindgen]
impl Contract {
    /// Mass donate and mint or donate and update depending on whether token
    /// exist or not. 
    #[payable]
    pub fn minting_interface(
      &mut self,
      suffix_token_id: String,
      hash_of_amounts: HashMap<CategoryId, f64>,
      issued_at: Option<u64>,
    ) {
      require!(
        env::predecessor_account_id() == env::signer_account_id(),
        "This method can only be called by signer."
      );

      let mut token_id_list: HashMap<CategoryId, TokenId> = match 
          self.tokens_per_owner_ordered.get(&env::predecessor_account_id()) 
      {
        Some(value) => value,
        None => HashMap::new()
      };

      let mut total_use: u128 = 0;
      

      for (id, amount) in hash_of_amounts {
        // Check if token already exist. 
        if let Some(token_id) = token_id_list.get(&id) {
          // Due to some errors, we overwrite when cannot find. 
          if let Some(_token) = self.tokens_by_id.get(&token_id) {
            let attached = near_to_yoctonear(amount);
            
            total_use += attached;
            ext_self::donate_and_update(
              token_id.clone(),
              amount,
  
              env::current_account_id(),
              attached,
              GAS_PER_DONATE,
            );
          } else {
          let attached = near_to_yoctonear(amount + 0.1);
          total_use += attached;
            // Get metadata from lookupmap. 
          let mut metadata = expect_lightweight(
            self.token_metadata_by_cat_id.get(&id),
            "Found category but not its metadata. Maybe forgot to map?"
          );

          metadata.issued_at = issued_at;

            ext_self::donate_and_mint(
              token_id.clone(),
              metadata,
              amount,
  
              env::current_account_id(),
              attached,
              GAS_PER_DONATE
            );
          }
          
        } else {
          let attached = near_to_yoctonear(amount + 0.1);
          total_use += attached;

          // Get prefix
          let prefix: String = expect_lightweight(
            self.categories.get(id.clone() as u64),
            "Cannot find category. Please contact support."
          );

          // Create token_id based on suffix. 
          let token_id: TokenId = prefix + suffix_token_id.as_str();

          // Add to list
          token_id_list.insert(id.clone(), token_id.clone());

          // Get metadata from lookupmap. 
          let mut metadata = expect_lightweight(
            self.token_metadata_by_cat_id.get(&id),
            "Found category but not its metadata. Maybe forgot to map?"
          );

          metadata.issued_at = issued_at;

          // Cross contract call. 
          ext_self::donate_and_mint(
            token_id,
            metadata,
            amount,

            env::current_account_id(),
            attached,
            GAS_PER_DONATE
          );
        }
      }

      let refund_amount = env::attached_deposit() - total_use;

      if refund_amount > 0 {
        Promise::new(env::signer_account_id())
            .transfer(refund_amount);
      }

      // Insert token id list back.
      // Weakness: if something fails, this will not get deleted... 
      // like during minting. 
      self.tokens_per_owner_ordered.insert(
        &env::predecessor_account_id(), 
        &token_id_list
      );
    }


    /// Donate money and we'll mint and nft for you. 
    /// donate_amount is not U128 to reduce gas cost? We infer from f64 instead. 
    /// Total pay will be donate_amount + 0.1N (which mostly refunded for storage).
    #[payable]
    pub fn donate_and_mint(
      &mut self,
      token_id: TokenId,
      metadata: TokenMetadata,
      donate_amount: f64,
    ) {
      let donate_amount_u128: u128 = near_to_yoctonear(donate_amount);
      require!(
        env::attached_deposit() >= (donate_amount_u128 + near_to_yoctonear(0.1)),
        "You attached less than you want to donate + 0.1N for storage (mostly refunded)."
      );
  
      require!(
        env::attached_deposit() <= (donate_amount_u128 + near_to_yoctonear(0.101)),
        "You attached too much money than you want to donate. Try again!"
      );

      Promise::new(env::current_account_id())
          .transfer(donate_amount_u128)
          .then(
            ext_self::nft_mint(
              token_id,
              donate_amount.to_string(),
              metadata,
              env::signer_account_id(),
              None,

              env::current_account_id(),
              near_to_yoctonear(0.1),
              GAS_FOR_MINTING
            )
      );
    }


    /// Donate and Update: We already have a minted donation, so we'll update it. 
    /// First check whether have already minted, then pull the token id to update it
    /// before pushing back. 
    /// 
    /// Checks will be done with regex in frontend. 
    /// 
    /// Subsequent storage usage, we'll pay for them, taken out from their donations. 
    /// This value is so small it's ignorable. (less than 1 cent most probably).
    #[payable]
    pub fn donate_and_update(
      &mut self,
      token_id: TokenId,
      donate_amount: f64,
    ) {
      let donate_amount_u128: u128 = near_to_yoctonear(donate_amount);
      require!(
        env::attached_deposit() >= donate_amount_u128,
        "You attached less than you want to donate + 0.1N for storage (mostly refunded)."
      );
  
      require!(
        env::attached_deposit() <= donate_amount_u128,
        "You attached too much money than you want to donate. Try again!"
      );

      let mut token = expect_lightweight(
        self.tokens_by_id.get(&token_id),
        "This token_id cannot be found."
      );

      let old_donate_amount: f64 = token.donate_amount.parse().unwrap();
      let new_donate_amount: u128 = near_to_yoctonear(old_donate_amount) + donate_amount_u128;

      // Function only converts to first 5 decimals.
      token.donate_amount = yoctonear_to_near(new_donate_amount).to_string(); 

      self.tokens_by_id.insert(&token_id, &token);

      Promise::new(env::current_account_id())
            .transfer(env::attached_deposit())
            .then(
              ext_self::on_donate_update(
                token_id,
                old_donate_amount,

                env::current_account_id(),
                0,
                GAS_FOR_MINTING  // just use this is fine.
              )
      );
    }


    #[private]
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id:TokenId,
        donate_amount_str: String,
        metadata: TokenMetadata,
        receiver_id: AccountId,
        perpetual_royalties: Option<HashMap<AccountId, u16>>,
    ) {
        if !is_promise_success() {
          // Ok, I don't know what would happen, so let's just "contact support."
          // assuming there's a support. 
          env::panic_str("Transfer donation failed. Please contact support.");
        }

        // For this specific use case, only the contract can mint. 
        // Hence, this will be used as a callback function. 
        require!(
          env::predecessor_account_id() == env::current_account_id(),
          concat!(
            "Only the contract can mint an nft via a cross-contract call. ",
            "End users should call 'donate_and_mint' instead."
          )
        );

        // measure the initial storage being used on contract.
        let initial_storage_usage = env::storage_usage();

        // create royalty map to store the token.
        let mut royalty = HashMap::new();

        // if perpetual royalties were passed into the function. 
        if let Some(perpetual_royalties) = perpetual_royalties {
          // our max payout is 7 people, otherwise not enough GAS. 
          require!(
            perpetual_royalties.len() < 7,
            "Cannot add more than 6 perpetual royalty amounts"
          );

          for (account, amount) in perpetual_royalties {
            royalty.insert(account, amount);
          }
        }


        // We need contract be approved so it can pull back nft
        // when user donate the next time, to update it. 


        // specify the token struct that contains the owner ID. 
        let mut token = Token {
          owner_id: receiver_id,
          donate_amount: donate_amount_str,
          approved_account_ids: Default::default(),  // default value is empty map.
          next_approval_id: 1,  // 0 will be inserted later.
          royalty,
        };

        let _ = token
              .approved_account_ids
              .insert(env::current_account_id(), 0)  // approved id of 0 inserted here.
              .is_none();

        // insert token ID and token struct and make sure token
        // doesn't exist. 
        require!(
          self.tokens_by_id.insert(&token_id, &token).is_none(),
          "Token already exists."
        );

        self.token_metadata_by_id.insert(&token_id, &metadata);

        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        // Log the minting as per events standard. 
        let nft_mint_log: EventLog = EventLog {
          standard: NFT_STANDARD_NAME.to_string(),
          version : NFT_METADATA_SPEC.to_string(),
          event   : EventLogVariant::NftMint(vec![NftMintLog {
            owner_id : token.owner_id.to_string(),
            token_ids: vec![token_id.to_string()],
            memo     : None,  // optional
          }]),
        };

        // log serialized json
        env::log_str(&nft_mint_log.to_string());

        // calculate required storage
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        // refund excess storage if user attached too much. 
        // Panic if they didn't attach enough. 
        refund_deposit(required_storage_in_bytes, env::signer_account_id());
    }


    #[private]
    pub fn on_donate_update(
      &mut self,
      token_id: TokenId,
      old_donate_amount: f64,
    ) {
      // If promise failed
      if !is_promise_success() {
        let mut token = expect_lightweight(
          self.tokens_by_id.get(&token_id),
          "This token_id cannot be found."
        );
  
        token.donate_amount = old_donate_amount.to_string();
  
        self.tokens_by_id.insert(&token_id, &token);
      }

      // One isn't sure, if transfer failed, supposingly we can't 
      // transfer back, because the money is not with us yet? 
      // NEAR Protocol might refund automatically if promise failed? 

    }
}