use scrypto::prelude::*;

#[derive(ScryptoSbor, PartialEq)]
pub enum StatusSale {
    SaleNotStarted,
    SaleOngoing,
    SalePaused,
}

#[derive(ScryptoSbor, PartialEq)]
pub enum PriceStrategy {
    Runtime,
    Manual(Decimal)
}

#[derive(ScryptoSbor, PartialEq)]
pub enum CouponState {
    Reserved(u16),
    Claimed(u16)
}

#[blueprint]
mod pyrosale {    

    enable_method_auth! { 
        roles {
            super_admin => updatable_by: [];
            admin => updatable_by: [super_admin];
        },        

        methods { 
            // Minting
            add_nfts_for_sale => restrict_to: [admin, super_admin, OWNER];  // hot wallet                        
                        
            // Stati            
            start_sale => restrict_to: [super_admin, OWNER];  
            pause_sale => restrict_to: [super_admin, OWNER];        
            continue_sale => restrict_to: [super_admin, OWNER];        
            
            // Sale XRD
            set_price => restrict_to: [super_admin, OWNER];                                     
            use_manual_usd_price => restrict_to: [super_admin, OWNER];  
            use_runtime_usd_price => restrict_to: [super_admin, OWNER];                          
            
            // Buying with XRD
            buy_placeholders => PUBLIC;             
            assign_placeholders_to_nfts => restrict_to: [admin, super_admin, OWNER];             
            swap_placeholders => PUBLIC;
            
            // Sale USD            
            reserve_nfts_for_usd_sale => restrict_to: [admin, super_admin, OWNER];            
            claim_nfts_for_usd_sale => restrict_to: [admin, super_admin, OWNER];            
            
            // Earnings            
            withdraw_xrd => restrict_to: [OWNER];             

            // just for the case the SC is broken so we can still get the minted NFTs out and sell them otherwise
            collect_nfts_in_emergency_situation => restrict_to: [OWNER];                                                                             

            // test helpers
            get_placeholder_bucket => PUBLIC;
            get_pyro_bucket => PUBLIC;
            get_latest_usd_price => PUBLIC;
            get_internal_state => PUBLIC;
            set_do_check_for_same_transaction => PUBLIC;                       
        }
    }

        //struct Pyro<'a> {
        struct PyroSale {

            // parameters
            max_amount_nfts_per_buy_or_change:u16,                                     

            // addresses
            pyro_nfts_address: ResourceAddress, 
            placeholder_nfts_address:ResourceAddress, 

            // status            
            status_sale: StatusSale,             

            // Price stages
            price_nft_usd_stage1: Decimal,              
            price_nft_usd_stage2: Decimal,              
            price_nft_usd_stage3: Decimal, 
            amount_stage1: u16, 
            amount_stage2: u16,         

            price_strategy: PriceStrategy,
            // use_manual_usd_price: bool,
            // manual_usd_price: Decimal,                 

            // vaults
            pyro_nfts_vault : Vault,       
            placeholder_nfts_vault : Vault,
            collected_xrd_vault: Vault,                     

            // vectors
            nft_ids: Vec<u16>,                      // list/vector with all minted nft ids                        
            mapping_placeholder_nft: Vec<u16>,      // mapping PH id -> Pyro Id

            // placeholders sold vs. mapped - needed for assigning placeholder to pyro nfts
            placeholders_sold_or_used_up_total: u16,           // total amount of phs "sold" including the ones for and USD   
            placeholders_kept_outside: u16,        
            placeholders_mapped: u16,               // indicating that ids 1...placeholders_mapped were assigned a real nft id (stored in mapping)                                    
                                            
            // availability            
            sold_usd_just_reserved: u16,       
            cap_left_sale: Decimal,                 // = placeholder_nfts_vault.amount() - sold_usd_just_reserved     

            // coupons
            coupons: KeyValueStore<String, CouponState>,   // make sure coupon code is not redeemed twice (so method is not called twice for same coupon)                                 

            // for disallowing assign_placeholders with getting placeholders in one transaction
            last_random_based_on_trans_hash: u32,             
                        
            // for displaying only            
            added_nfts_total: u16,
            sold_xrd_total: u16,    
            sold_usd_total: u16,    
            latest_usd_price: Decimal,                           
            placeholders_unassigned:u16,            // = self.placeholders_sold_or_used_up_total + placeholders_kept_outside - self.placeholders_mapped

            // test helper
            do_check_for_same_transaction:bool,
        }
    

    impl PyroSale {        
        pub fn instantiate_pyro(
            owner_badge_address:ResourceAddress, super_admin_badge_address:ResourceAddress, admin_badge_address:ResourceAddress,
            pyro_nfts_address: ResourceAddress, placeholder_nfts_address:ResourceAddress, 
            price: Decimal,            
            dapp_definition_address:ComponentAddress, max_amount_nfts_per_buy_or_change:u16,            
            )             
            -> Global<PyroSale> 
        {                                    
            // we need component_address to allow the component to mint new badges
            let (address_reservation, _component_address) =
                Runtime::allocate_component_address(PyroSale::blueprint_id());                                                                                                 

            // init usd price
            let usd_price = Runtime::get_usd_price();                 
        
            // Instantiate a Pyro component
            let x = Self {
                max_amount_nfts_per_buy_or_change,                                 

                pyro_nfts_address, 
                placeholder_nfts_address, 

                status_sale: StatusSale::SaleNotStarted,

                price_nft_usd_stage1: price, 
                price_nft_usd_stage2: price, 
                price_nft_usd_stage3: price, 
                amount_stage1: 0, 
                amount_stage2: 0, 

                price_strategy: PriceStrategy::Runtime, 

                pyro_nfts_vault: Vault::new(pyro_nfts_address),
                placeholder_nfts_vault: Vault::new(placeholder_nfts_address), 
                collected_xrd_vault: Vault::new(XRD),

                nft_ids: Vec::new(),                 
                mapping_placeholder_nft: Vec::new(), 
                coupons: KeyValueStore::new(),                 
                
                placeholders_sold_or_used_up_total: 0u16,
                placeholders_kept_outside: 0u16,
                placeholders_mapped: 0u16,                                

                sold_usd_just_reserved: 0u16, 
                cap_left_sale: Decimal::ZERO, 

                last_random_based_on_trans_hash: 0u32, 

                added_nfts_total: 0u16,                                 
                sold_xrd_total: 0u16, 
                sold_usd_total: 0u16,                                                 
                latest_usd_price: usd_price,                 
                placeholders_unassigned: 0u16, 
                
                do_check_for_same_transaction: true
            }
            .instantiate()            
            .prepare_to_globalize(OwnerRole::Fixed(
                rule!(require(owner_badge_address))))
            .roles( 
                roles!(
                    super_admin => rule!(require_amount(dec!(1), super_admin_badge_address)); 
                    admin => rule!(require(admin_badge_address));
                )
            )
            .with_address(address_reservation)                 
            .metadata(
                metadata!(                  
                  init {
                    "dapp_definition" =>
                      GlobalAddress::from(dapp_definition_address), updatable;                    
                  }
                )
              )
            .globalize();

            x

        }        

        pub fn start_sale(&mut self) {
            
            assert!(self.status_sale == StatusSale::SaleNotStarted, "Sale is already started.");

            self.status_sale = StatusSale::SaleOngoing;            
        }

        pub fn pause_sale(&mut self) {
                        
            assert!(self.status_sale == StatusSale::SaleOngoing, "Sale is not started.");

            self.status_sale = StatusSale::SalePaused;
        }   

        pub fn continue_sale(&mut self) {
                        
            assert!(self.status_sale == StatusSale::SalePaused, "Sale is not paused.");

            self.status_sale = StatusSale::SaleOngoing;
        }            

        pub fn assign_placeholders_to_nfts(&mut self, max_assign_at_once:u16)  {            
            
            self.check_for_same_transaction(); // you cannot do this within same transaction as buying placeholder nfts or getting placeholder nfts for team members

            
            assert!(self.status_sale != StatusSale::SaleNotStarted, "Sale is not started yet. That is why assignment from placeholders cannot be done yet.");

            let mut i = self.placeholders_mapped;

            let mut cnt_mapped = 0u16;  

            let placeholders_to_be_mapped =   self.placeholders_sold_or_used_up_total + self.placeholders_kept_outside;        

            while i < placeholders_to_be_mapped && cnt_mapped < max_assign_at_once
            {
                let max = self.get_nft_ids_left() as u32;

                let random = self.get_random_nr(max);

                let id = self.get_nft_id_from_list(random as usize); // removes element from nft_ids

                self.mapping_placeholder_nft.push(id);

                i+=1;
                cnt_mapped+=1;

                self.placeholders_mapped+=1;
            }

            self.check_assertions();
            self.set_placeholders_unassigned();
        }        
        
        fn add_nft_to_list(&mut self, nft: Bucket)
        {
            self.pyro_nfts_vault.put(nft);
        }

        fn add_placeholder_to_list(&mut self, ph: Bucket)
        {            
            if ph.amount() == Decimal::ZERO
            {
                // the placeholder is outside and therefore must be mapped - we need to ensure that placeholders 1..n are outside and n+1..total are put into SC
                self.placeholders_kept_outside +=1;
            }  

            self.cap_left_sale += ph.amount();
            self.placeholder_nfts_vault.put(ph);          
        }
        
        fn get_nft_id_from_list(&mut self, pos:usize) -> u16
        {
            return self.nft_ids.remove(pos);
        }

        fn get_nft_ids_left(&mut self) -> usize
        {
            return self.nft_ids.len();
        }

        fn get_placeholder_id_from_list(&mut self) -> u16
        {
            return self.placeholders_sold_or_used_up_total + self.placeholders_kept_outside + 1;
        }

        fn get_placeholder_from_list(&mut self) -> Bucket
        {
            let id = self.get_placeholder_id_from_list() as u64;            

            // create nf id
            let key = &NonFungibleLocalId::Integer( id.into() );
                    
            // retrieve nft based on key
            let nft:Bucket = self.placeholder_nfts_vault.as_non_fungible().take_non_fungible(key).into();

            self.cap_left_sale -= Decimal::ONE;

            nft
        }            

        // Mint a new Pyro NFT and keep it in blueprint for sale            
        pub fn add_nfts_for_sale(&mut self, nft_id:u16, pyro_nft:Bucket, placeholder_nft:Bucket)  { 

            assert!(pyro_nft.amount()==Decimal::ONE, "pyro_nft bucket must contain exactly one pyro nft, but it contains {}.", pyro_nft.amount());              

            assert!(placeholder_nft.amount()<=Decimal::ONE, "placeholder_nft bucket must contain one or zero placeholder nft, but it contains {}.", placeholder_nft.amount());              
                        
            // add nft_id into list of nft_ids, nfts and placeholder_ids        
            self.nft_ids.push(nft_id);
            
            // add nft into vector of nfts
            self.add_nft_to_list(pyro_nft); 

            // add nft into vector of phs
            self.add_placeholder_to_list(placeholder_nft);             

            // increase counter of minted nfts (meant to be sold against xrd by default) and decrease left over nfts 
            self.added_nfts_total +=1;                        

            self.check_assertions();
        }        

        fn transaction_hash_to_number() ->u32
        {
            let hash = Runtime::transaction_hash();                        

            let bytes = hash.as_slice();

            let result = 
                (bytes[0] as u32) << 24 |
                (bytes[1] as u32) << 16 |
                (bytes[2] as u32) << 8 |
                (bytes[3] as u32);

            result
        }

        fn get_random_nr(&mut self, max:u32) -> u32 {
            
            let random = Self::transaction_hash_to_number();
                                   
            let number = random % max;

            number            
        }             

        fn get_mapped_nfts(&mut self, mut placeholders:Bucket, amount:u16) -> (NonFungibleBucket, Bucket) {

            let mut bucket = Bucket::new(self.pyro_nfts_address).as_non_fungible();

            let mut i = 1u16;
            while i <= amount {  

                // get id of placeholder
                let ph = placeholders.take(1);
                
                let placeholder_nf = ph.as_non_fungible();

                let id: NonFungibleLocalId = placeholder_nf.non_fungible_local_id();

                // convert id to u64
                let placeholder_id : u64 = match id { 
                    NonFungibleLocalId::Integer(int_id) => { int_id.value().into() } 
                    _ => { panic!("not an integer nft") }
                  };
                                  
                // get the id of the mapped nft                
                let nft_id_option = self.mapping_placeholder_nft.get((placeholder_id - 1) as usize);

                assert!(nft_id_option.is_some(), 
                "The placeholder with id = {} was not yet assigned to a real pyro. Please try again later.  self.placeholders_mapped = {}, self.placeholders_sold_or_used_up_total = {}, self.placeholders_kept_outside = {}", placeholder_id, self.placeholders_mapped, self.placeholders_sold_or_used_up_total, self.placeholders_kept_outside);
                                
                let nft_id = *nft_id_option.unwrap();

                let nft_non_fung_id = IntegerNonFungibleLocalId::new(nft_id as u64);

                // get nft and save it in buckets for returning
                let one_nft = self.pyro_nfts_vault.as_non_fungible().take_non_fungible(&NonFungibleLocalId::Integer(nft_non_fung_id));                
                bucket.put(one_nft);                

                // burn placeholder
                // self.rm_pyro_ph.burn(ph);
                ph.burn(); // CHECK                                                            

                i+=1;
            };

            (bucket, placeholders)

        }

        fn get_one_placeholder(&mut self) -> Bucket {                        

            let ph: Bucket = self.get_placeholder_from_list();

            assert!(ph.amount() == Decimal::ONE, "method get_one_placeholder should return ONE nft, but doesn't.");

            return ph; 
        }

        fn get_placeholders(&mut self, amount: u16) -> Bucket {

            let mut bucket = Bucket::new(self.placeholder_nfts_address);

            let mut i = 1u16;

            while i <= amount {                                            
                let bucket_one = self.get_one_placeholder();

                bucket.put(bucket_one);

                i+=1;                

                self.placeholders_sold_or_used_up_total +=1;                 
            };

            bucket 
        }  


        fn get_total_price(&mut self, mut amount:u16) -> Decimal {

            let mut amount_sold = self.placeholders_sold_or_used_up_total + self.sold_usd_just_reserved; // don't count team pyros, giveaways are not contained in placeholders_sold_or_used_up_total            
            
            let mut use_stage1 = 0u16;            
            let mut use_stage2 = 0u16;
            let mut use_stage3 = 0u16;

            let amount_start = amount;

            // calc usage of stage 1 price
            if amount_sold < self.amount_stage1
            {
                let av_stage1 = self.amount_stage1 - amount_sold;
                use_stage1 = std::cmp::min(amount, av_stage1);

                amount      -= use_stage1;
                amount_sold += use_stage1;
            }
            
            // calc usage of stage 2 price
            if amount_sold < self.amount_stage2
            {
                let av_stage2 = self.amount_stage2 - amount_sold;
                use_stage2 = std::cmp::min(amount, av_stage2);

                amount      -= use_stage2;
                // amount_sold += use_stage2;
            }

            // calc usage of stage 3 price
            if amount > 0
            {
                use_stage3 = amount;
                amount -= use_stage3;
            }

            assert!(amount == 0, "Price cannot be calculated. amount should be zero here, but is {}.", amount);

            assert!(amount_start == use_stage1 + use_stage2 + use_stage3);

            // total price in usd
            let total_price_usd =   use_stage1 * self.price_nft_usd_stage1 + 
                                             use_stage2 * self.price_nft_usd_stage2 +
                                             use_stage3 * self.price_nft_usd_stage3;
                                
            let usd_price = match self.price_strategy 
            {
                PriceStrategy::Runtime => Runtime::get_usd_price(), 
                PriceStrategy::Manual( manual_price) => manual_price,                 
            };

            
            assert!(usd_price > Decimal::ZERO, "USD price must be greater than zero.");
            
            self.latest_usd_price = usd_price;

            let total_price_usd_dec = Decimal::from(total_price_usd);            
            let price_xrd = total_price_usd_dec * usd_price;      

            // let's be a bit pathetic here to make sure we don't sell anything for free :-)
            assert!(price_xrd > Decimal::ZERO, "USD price must be greater than zero.");      

            price_xrd
        }
        
        // buy place holders via XRD
        pub fn buy_placeholders (&mut self, mut payment: Bucket, amount: u16) -> (Bucket, Bucket) {                          

            assert!(self.status_sale == StatusSale::SaleOngoing, "Sale is not started or paused.");

            assert!(amount<=self.max_amount_nfts_per_buy_or_change, "You can only buy {} NFTs at once.", self.max_amount_nfts_per_buy_or_change);    

            assert!(payment.resource_address() == XRD, "You can only buy with XRD.");

            assert!(amount > 0, "Amount should be greater than zero, but is {}.", amount);

            // check if amount less or equal than available nfts             
            assert!(Decimal::from(amount)<=self.cap_left_sale, "Only {} NFTs left for sale, but {} needed.", self.cap_left_sale, amount);

            // calc price, take it from payment and put it into collected xrd vault
            let total = self.get_total_price(amount);                        

            self.collected_xrd_vault.put(payment.take(total));                                
                                    
            // retrieve placeholder nfts which can be changed into real nfts in a 2nd step
            let sold_phs= self.get_placeholders(amount);                                                                                            

            // make sure caller gets requested amount of placeholder NFTs back
            assert!(sold_phs.amount() == Decimal::from(amount), "{} placeholder NFTs should be bought, but only {} would be returned", amount, sold_phs.amount());                                            

            // make sure you cannot assign placeholders in same transaction
            self.set_last_random_based_on_trans_hash(); 

            self.sold_xrd_total += amount;
            
            self.check_assertions();

            self.set_placeholders_unassigned();

            // Return the NFT and change
            (sold_phs, payment)  
        }
        
        // change placeholders into real pyro nfts
        pub fn swap_placeholders(&mut self, placeholders: Bucket, amount: u16) -> (NonFungibleBucket, Bucket) {                                     
            
            assert!(amount > 0, "Amount should be greater than zero, but is {}.", amount);            
            assert!(amount<=self.max_amount_nfts_per_buy_or_change, "You can only chage {} Placeholderss at once.", self.max_amount_nfts_per_buy_or_change);
            // check if placeholders resource address is the right one.
            assert!(placeholders.resource_address() == self.placeholder_nfts_vault.resource_address(), "Placeholder adress is {} but should be {}", placeholders.resource_address().to_hex(), self.placeholder_nfts_vault.resource_address().to_hex() );
            assert!(self.status_sale != StatusSale::SaleNotStarted, "Sale is not started yet. That is why changing placeholders into individual NFTs cannot be done yet.");
            // check if there are enough placeholder nfts
            assert!(placeholders.amount()>= Decimal::from(amount), "Only {} placeholders in bucket, but {} needed.", placeholders.amount(), amount);
            assert!(self.placeholders_mapped == self.placeholders_sold_or_used_up_total + self.placeholders_kept_outside, "Not all placeholders were mapped to real NFTs. {} sold, {} outside, but only {} mapped", self.placeholders_sold_or_used_up_total, self.placeholders_kept_outside, self.placeholders_mapped );
                        
            // retrieve "random" nfts which were actually mapped in assign method before
            let (nfts, placeholders) = self.get_mapped_nfts(placeholders, amount);
            
            // make sure caller gets requested amount of NFTs
            assert!(nfts.amount() == Decimal::from(amount), "{} nfts should be bought, but only {} would be returned", amount, nfts.amount());            
            
            self.check_assertions();

            // Return the NFT and placeholders
            (nfts, placeholders) 
        }                    

        // get nfts for team       
        /*
        pub fn get_placeholders_for_team(&mut self, amount:u16) -> Bucket
        {                                       
            assert!(self.status_minting_finished, "Minting is not finished yet.");

            assert!(self.placeholders_for_team_sent + amount <= self.amount_nfts_for_team, 
                "There are only {} NFTs for team, but you want to get {} in total", self.amount_nfts_for_team, self.placeholders_for_team_sent + amount);                                    

            assert!(amount > 0, "Amount should be greater than zero, but is {}.", amount);

            let phs = self.get_placeholders(amount, ReasonPlaceholder::TeamMember);

            self.placeholders_for_team_sent += amount;

            self.cap_left_sale -= amount; 

            self.set_last_random_based_on_trans_hash(); // make sure you cannot assign placeholders in same transaction
            
            self.check_assertions();
            self.set_placeholders_unassigned();

            phs
        }
        */

        pub fn reserve_nfts_for_usd_sale(&mut self, coupon_code:String, amount: u16)
        {                           
            assert!(self.status_sale == StatusSale::SaleOngoing, "Sale is not started or paused.");              
            assert!(Decimal::from(amount)<=self.cap_left_sale, "Only {} NFTs left for sale, but {} needed for reservation.", self.cap_left_sale, amount);
            assert!(amount > 0, "Amount should be greater than zero, but is {}.", amount);
            
            // check if coupon was already redeemed            
            /*  coupons(Code)=
                a) undefined -> coupon was not yet used at all
                b) false -> coupon was used to reserve, but no NFTs were returned yet
                c) true  -> coupon was used and NFTs were already returned
            */

            let coupon_bool = self.coupons.remove(&coupon_code);

            let allowed:bool;

            match coupon_bool {
                Some(_)=>
                {
                    allowed = false; // reservation for this coupon was already done or nfts were even redeemed

                }, 
                None => 
                {
                    allowed = true; // coupon was not yet used at all - fine      
                   

                    self.sold_usd_just_reserved += amount;
                    self.cap_left_sale          -= amount; // this is the actual reservation                    

                    self.sold_usd_total += amount;
                }
            }

            if allowed
            {
                self.coupons.insert(coupon_code, CouponState::Reserved(amount)); //false = only reseverd, not yet redeemed.
            }
            else
            {
                assert!(allowed, "NFTs for this coupon code {} were already reserved.", coupon_code);
            }         

            self.check_assertions();                                   

        }
        
        // get nfts for sending them to coupon owners
        pub fn claim_nfts_for_usd_sale(&mut self, coupon_code:String, amount: u16, max_assign_at_once:u16) -> Bucket
        {                                       
            assert!(amount > 0, "Amount should be greater than zero, but is {}.", amount);            
                        
            /* check if coupon was already redeemed            
               coupons(Code)=
                a) undefined -> coupon was not yet used at all
                b) false -> coupon was used to reserve, but no NFTs were returned yet
                c) true  -> coupon was used and NFTs were already returned
            */

            let coupon_bool = self.coupons.remove(&coupon_code);            

            let allowed:bool;

            match coupon_bool {
                Some(state)=>
                {
                    allowed = state==CouponState::Reserved(amount);   // coupon was only used to reserve NFTs, but NFTs were not yet claimed

                    if allowed 
                    {
                        self.sold_usd_just_reserved -= amount; 
                        self.cap_left_sale += Decimal::from(amount); // // cap_left_sale will be reduced again when returning placeholders
                    }                    
                }, 
                None => 
                {
                    allowed = true; // coupon was not yet used at all.    

                    assert!(self.status_sale == StatusSale::SaleOngoing, "Sale is not started or paused.");                       

                    // TC 6.27a
                    assert!(Decimal::from(amount) <= self.cap_left_sale, "Only {} NFTs left for sale, but {} needed.", self.cap_left_sale, amount);
                                        
                    self.sold_usd_total += amount;
                }
            }

            if allowed
            {
                self.coupons.insert(coupon_code, CouponState::Claimed(amount));
            }
            else
            {
                assert!(allowed, "Coupon code {} was already redeemed or amounts do mismatch.", coupon_code);
            }                                            

            let phs = self.get_placeholders(amount);

            self.assign_placeholders_to_nfts(max_assign_at_once);

            let (nfts, placeholders_left) = self.swap_placeholders(phs, amount);

            assert!(placeholders_left.is_empty(), "There should be no placeholders left, but there are {} left.", placeholders_left.amount());    

            self.check_assertions();        

            placeholders_left.drop_empty();

            nfts.into()

        }        

        // change nft price
        pub fn set_price(&mut self, price1:Decimal, price2:Decimal, price3:Decimal, amount_stage1:u16, amount_stage2:u16)
        {
            assert!(price1 > Decimal::ZERO, "Price1 should be greater than zero, but is {}.", price1);
            assert!(price2 > Decimal::ZERO, "Price2 should be greater than zero, but is {}.", price2);
            assert!(price3 > Decimal::ZERO, "Price3 should be greater than zero, but is {}.", price3);                        
            assert!(amount_stage1 <= amount_stage2, "Amount for stage 2 must be greater or equal amount_stage1, but is {} > {}.", amount_stage1, amount_stage2);

            self.price_nft_usd_stage1 = price1;
            self.price_nft_usd_stage2 = price2;
            self.price_nft_usd_stage3 = price3;

            self.amount_stage1 = amount_stage1;
            self.amount_stage2 = amount_stage2;            
        }        

        // withdraw all collected xrds
        pub fn withdraw_xrd(&mut self) -> Bucket 
        {
            return self.collected_xrd_vault.take_all();
        }        

        fn set_last_random_based_on_trans_hash(&mut self)
        {
            self.last_random_based_on_trans_hash = Self::transaction_hash_to_number();
        }

        fn check_for_same_transaction(&mut self)
        {
            let latest_trans = self.last_random_based_on_trans_hash;
            let this_trans = Self::transaction_hash_to_number();

            if self.do_check_for_same_transaction
            {
                assert!(latest_trans != this_trans, "You cannot buy placeholder/get placeholder for team within the same transaction.");
            }            
        }

        fn check_assertions(&mut self)
        {                        
            assert!(self.cap_left_sale == self.placeholder_nfts_vault.amount() - self.sold_usd_just_reserved, 
                        "Internal State is invalid: cap_left_sale ({}) should be equal to placeholder_nfts_vault.amount()({}) - sold_usd_just_reserved ({}) .", 
                        self.cap_left_sale, self.placeholder_nfts_vault.amount(), self.sold_usd_just_reserved);
        }

        pub fn collect_nfts_in_emergency_situation(&mut self, mut amount:Decimal) -> Bucket
        {           
            assert!(amount > Decimal::ZERO, "Amount should be greater than zero, but is {}.", amount);
            assert!(self.status_sale == StatusSale::SalePaused, "Sales must be paused/stopped before you can get NFTs from SC.");

            if amount > self.pyro_nfts_vault.amount()
            {
                amount = self.pyro_nfts_vault.amount();
            }

            self.pyro_nfts_vault.take (amount)
        }        

        pub fn use_manual_usd_price(&mut self, manual_usd_price: Decimal) {
                        
            assert!(manual_usd_price > Decimal::ZERO, "Price must be greater than zero.");              

            self.price_strategy = PriceStrategy::Manual(manual_usd_price);            
            self.latest_usd_price = manual_usd_price;
        }

        pub fn use_runtime_usd_price(&mut self) {                                                

            self.price_strategy = PriceStrategy::Runtime;            

            self.latest_usd_price = Runtime::get_usd_price();

        }

        fn set_placeholders_unassigned(&mut self)
        {
            self.placeholders_unassigned = self.placeholders_sold_or_used_up_total + self.placeholders_kept_outside - self.placeholders_mapped;
        }

        // the following functions are testing helpers and only needed for testing

        pub fn get_placeholder_bucket(&self) -> Bucket
        {
            Bucket::new(self.placeholder_nfts_address)
        }

        pub fn get_pyro_bucket(&self) -> Bucket
        {
            Bucket::new(self.pyro_nfts_address)
        }

        pub fn get_latest_usd_price(&self) -> Decimal 
        {
            self.latest_usd_price
        }

        pub fn get_internal_state(&self) -> (Decimal, u16, u16, Decimal, u16, u16, Decimal)
        {
            let a = self.placeholder_nfts_vault.amount();

            let b = self.placeholders_sold_or_used_up_total;

            let c = self.mapping_placeholder_nft.len() as u16;

            let d = self.pyro_nfts_vault.amount();

            let e = self.nft_ids.len() as u16;

            let f = self.sold_usd_just_reserved;            

            let g = self.collected_xrd_vault.amount();

            (a, b, c, d, e, f, g)

        }

        pub fn set_do_check_for_same_transaction(&mut self, do_check:bool)
        {
            self.do_check_for_same_transaction = do_check;
        }
    }
}