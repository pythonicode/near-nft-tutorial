use crate::*;
use near_sdk::{CryptoHash};
use std::mem::size_of;

pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

pub(crate) fn refund_deposit(storage_used: u64) {
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    let attached_deposit = env::attached_deposit();
    assert!(attached_deposit >= required_cost, "Must attach {} yoctoNEAR to cover NFT storage.", required_cost);
    let refund = attached_deposit - required_cost;
    if refund > 1 { 
        Promise::new(env::predecessor_account_id()).transfer(refund); 
    }
}

pub(crate) fn assert_one_yocto() {
    assert_eq!(
        env::attached_deposit(),
        1,
        "Requires attached deposit of exactly 1 yoctoNEAR",
    )
}

impl Contract {
    pub(crate) fn internal_transfer(&mut self, sender_id: &AccountId, receiver_id: &AccountId, token_id: &TokenId, memo: Option<String>) -> Token{
        let token = self.tokens_by_id.get(&token_id).expect("No Token");    
        assert_eq!(&token.owner_id, sender_id, "Unauthorized: You can't transfer that token!");
        assert_ne!(sender_id, receiver_id, "Error: You can't send a token to yourself.");
        self.internal_remove_token_from_owner(sender_id, token_id);
        self.internal_add_token_to_owner(receiver_id, token_id);
        let new_token = Token {
            owner_id: receiver_id.clone(),
        };
        self.tokens_by_id.insert(token_id, &new_token);
        if let Some(memo) = memo {
            env::log_str(&format!("Memo: {}", memo).to_string());
        }
        token
    }

    pub(crate) fn internal_add_token_to_owner(&mut self, account_id: &AccountId, token_id: &TokenId) {
        let mut tokens_set = self.tokens_per_owner.get(account_id).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::TokenPerOwnerInner {
                    //we get a new unique prefix for the collection
                    account_id_hash: hash_account_id(&account_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });
        tokens_set.insert(token_id);
        self.tokens_per_owner.insert(account_id, &tokens_set);
    }

    pub(crate) fn internal_remove_token_from_owner(&mut self, account_id: &AccountId, token_id: &TokenId){
        let mut tokens = self.tokens_per_owner.get(account_id).expect("Error: No Tokens.");
        tokens.remove(token_id);
        if tokens.is_empty() {
            self.tokens_per_owner.remove(account_id);
        } else {
            self.tokens_per_owner.insert(account_id, &tokens);
        }
    }
}