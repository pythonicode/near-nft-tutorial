use crate::*;
use near_sdk::{ext_contract, Gas};

const GAS_FOR_NFT_APPROVE: Gas = Gas(10_000_000_000_000);
const NO_DEPOSIT: Balance = 0;

pub trait NonFungibleTokenCore {
    //approve an account ID to transfer a token on your behalf
    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>);

    //check if the passed in account has access to approve the token ID
	fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    );

    //revoke a specific account from transferring the token on your behalf
    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId);

    //revoke all accounts from transferring the token on your behalf
    fn nft_revoke_all(&mut self, token_id: TokenId);
}

#[ext_contract(ext_non_fungible_approval_receiver)]
trait NonFungibleTokenApprovalsReceiver {
    //cross contract call to an external contract that is initiated during nft_approve
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    );
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {

    //allow a specific account ID to approve a token on your behalf
    #[payable]
    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>) {
        assert_at_least_one_yocto();
        let mut token = self.tokens_by_id.get(token_id).expect("No Token!");
        assert_eq!(&env::predecessor_account_id, &token.owner_id, "Caller must own the token.");
        let approval_id: u64 = token.next_approval_id;
        let is_new_approval = token.approved_account_ids.insert(account_id.clone(), approval_id).is_none();
        let storage_used = if is_new_approval {
            bytes_for_approved_account_id(&account_id);
        } else {
            0
        }
        token.next_approval_id += 1;
        self.tokens_by_id.insert(token_id, &token);
    }

    //check if the passed in account has access to approve the token ID
	fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) {
        /*
            FILL THIS IN
        */
    }

    //revoke a specific account from transferring the token on your behalf 
    #[payable]
    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId) {
        /*
            FILL THIS IN
        */
    }

    //revoke all accounts from transferring the token on your behalf
    #[payable]
    fn nft_revoke_all(&mut self, token_id: TokenId) {
        /*
            FILL THIS IN
        */
    }
}