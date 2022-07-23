use scrypto::prelude::*; //use scrypto standard lib

blueprint! {
    struct TokenSale {
       team_token_vault:Vault,
       xrd_tokens_vault:Vault,
       price_per_token:Decimal
        }
    impl TokenSale{
        pub fn new(price_per_token:Decimal)->(ComponentAddress,Bucket){
            let bucket:Bucket=ResourceBuilder::new_fungible()
            .divisibility(DIVISIBILITY_MAXIMUM)
            .metadata("name","team_token")
            .metadata("symbol","TTN")
            .metadata("team-member-1-ticket-number","4115508479")
            .metadata("team-member-2-ticket-number","4142374689")
            .metadata("team-member-3-ticket-number","4134717069")
            .metadata("team-member-4-ticket-number","4134732459")            
            .initial_supply( 100000);
            
            let seller_badge: Bucket=ResourceBuilder::new_fungible()
            .divisibility(DIVISIBILITY_MAXIMUM)
            .metadata("name","Selerbadge")
            .metadata("symbol","Seller")
            .initial_supply(1);
            
            let access_rules:AccessRules=AccessRules::new()
            .method("withdraw_funds",rule!(require(seller_badge.resource_address())))  
            .method("change_price",rule!(require(seller_badge.resource_address())))
            .default(rule!(allow_all));            
            
            
            let component_address: ComponentAddress= Self            
            {
                team_token_vault:Vault::with_bucket(bucket),
                xrd_tokens_vault:Vault::new(RADIX_TOKEN),
                price_per_token:price_per_token
            }
            .instantiate()
            .globalize();
            (component_address,seller_badge)
        }
        pub fn buy(&mut self,funds: Bucket) -> Bucket{
            let purchase_ammount:Decimal=funds.amount()/self.price_per_token;
            self.xrd_tokens_vault.put(funds);
            self.team_token_vault.take(purchase_ammount)

        }
        
        pub fn withdraw_funds(&mut self,amount: Decimal) -> Bucket{
            self.xrd_tokens_vault.take(amount)

        }
        
        pub fn change_price(&mut self,price:Decimal){
            self.price_per_token=price
        }
    }

    
}