use scrypto::prelude::*;

#[derive(ScryptoSbor, PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct PriceData {
    pub price: Decimal,
    pub timestamp: i64,
}

#[blueprint]
mod dummy_oracle {
    struct DummyOracle {
        account_address: ComponentAddress,        
        price: Decimal,        
    }

    impl DummyOracle {
        pub fn instantiate( price:Decimal ) -> Global<DummyOracle> {
            let (address_reservation, account_address) = Runtime::allocate_component_address(
                DummyOracle::blueprint_id(), 
            );

            (Self {
                account_address,
                price 
            })
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .with_address(address_reservation)
                .globalize()
        }

        pub fn address(&self) -> ComponentAddress {
            self.account_address
        }

        pub fn get_price(&self) -> PriceData {            
            PriceData {
                price: self.price, 
                timestamp: 0i64
            }
        }

        pub fn set_price(&mut self, new_price:Decimal) {            
            self.price = new_price;            
        }
    }
}