use scrypto::prelude::*;

#[blueprint]
mod test_runtime_usd {        
        
    struct TestRuntimeUsd {
            latest_usd_price: Decimal,                                       
    }    

    impl TestRuntimeUsd {        
        pub fn instantiate_pyro(                  
            )             
            -> Global<TestRuntimeUsd> 
        {                                                                                                                                                
            // init usd price
            let usd_price = Runtime::get_usd_price();                 
        
            // Instantiate a Pyro component
            let x = Self {                                         
                latest_usd_price: usd_price,                                 
            }
            .instantiate()                                    
            .prepare_to_globalize(OwnerRole::None)
            .globalize();
            x
        }                
    }
}