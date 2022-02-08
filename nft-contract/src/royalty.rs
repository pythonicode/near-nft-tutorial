use crate::*;

pub trait NonFungibleTokenCore {
    //calculates the payout for a token given the passed in balance. This is a view method
  	fn nft_payout(&self, token_id: String, balance: U128, max_len_payout: u32) -> Payout;
    
    //transfers the token to the receiver ID and returns the payout object that should be payed given the passed in balance. 
    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: u64,
        memo: String,
        balance: U128,
        max_len_payout: u32,
    ) -> Payout;
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {

    //calculates the payout for a token given the passed in balance. This is a view method
    fn nft_payout(&self, token_id: TokenId, balance: U128, max_len_payout: u32) -> Payout {
		let token = self.tokens_by_id.get(&token_id).expect("No Token.");
        let owner_id = token.owner_id;
        let royalty = token.royalty;
        let mut combined_royalty = 0;
        let mut payout_object = Payout {
            payout: HashMap::new(),
        };
        let bal = u128::from(balance);
        assert!(royalty.len() as u32 <= max_len_payout, "Market cannot payout to that many receivers");
        for (k, v) in royalty.iter() {
            let key = k.clone();
            let this_payout = royalty_to_payout(*v, bal);
            if key != owner_id {
                payout_object.payout.insert(key, this_payout);
                combined_royalty += *v; 
            }
        }
        payout_object.payout.insert(owner_id, royalty_to_payout(10000 - combined_royalty, bal));
        payout_object
	}

    //transfers the token to the receiver ID and returns the payout object that should be payed given the passed in balance. 
    #[payable]
    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: u64,
        memo: String,
        balance: U128,
        max_len_payout: u32,
    ) -> Payout {
        assert_one_yocto();
        let sender_id = env::predecessor_account_id();
        let previous_token = self.internal_transfer(
            &sender_id,
            &receiver_id,
            &token_id,
            Some(approval_id),
            Some(memo),
        );
        refund_approved_account_ids(
            previous_token.owner_id.clone(),
            &previous_token.approved_account_ids,
        );
        let payout = self.nft_payout(token_id, balance, max_len_payout);
        payout
    }
}
