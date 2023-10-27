use scrypto::prelude::*;

#[derive(NonFungibleData, ScryptoSbor)]
pub struct PyroNFT {
    pub id: u16,     
    pub name: String, 
    pub description: String, 
    key_image_url: String,     
    pub collection: String,    
    pub bracers: String, 
    pub ear_ring: String, 
    pub glasses: String, 
    pub head_style: String, 
    pub necklace: String, 
    pub nose: String, 
    pub pants: String, 
    pub ring: String, 
    pub shirts: String, 
    pub tattoos: String, 
    pub wall: String, 
    pub traits: Vec<(String, String)>, 
    pub key_image_hash: String,     
}

#[derive(NonFungibleData, ScryptoSbor)]
pub struct PyroPlaceholder {
    pub id: u16,    
    pub description: String, 
    pub key_image_url: String, 
    pub collection: String
}

enum ReasonPlaceholder {
    SaleXrd,
    SaleUsd,
    TeamMember,
}



#[blueprint]
mod pyro {

    enable_method_auth! { 
        roles {
            super_admin => updatable_by: [];
            admin => updatable_by: [super_admin];
        },        

        methods { 
            // Badges
            get_another_admin_badge => restrict_to: [admin, super_admin, OWNER];
            get_another_super_admin_badge => restrict_to: [super_admin, OWNER];
            get_another_owner_badge => restrict_to: [OWNER];

            // Minting
            mint_nft => restrict_to: [admin, super_admin, OWNER];  // hot wallet
            mint_individual_nft_and_return_for_giveaway => restrict_to: [admin, super_admin, OWNER]; // 
            get_placeholders_for_team => restrict_to: [super_admin, OWNER]; //             
            
            
            // Stati
            set_status_minting_finished => restrict_to: [super_admin, OWNER];       
            set_sale_started => restrict_to: [super_admin, OWNER];  
            set_sale_paused => restrict_to: [super_admin, OWNER];        
            
            // Sale XRD
            set_price => restrict_to: [super_admin, OWNER];                                     
            use_manual_usd_price => restrict_to: [super_admin, OWNER];  
            use_runtime_usd_price => restrict_to: [super_admin, OWNER];                          
            
            buy_placeholders => PUBLIC;             
            assign_placeholders_to_nfts => restrict_to: [admin, super_admin, OWNER];             
            change_placeholders_into_nfts => PUBLIC;
            
            // Sale USD
            set_buffer_usd_sale => restrict_to: [super_admin, OWNER];             
            get_nfts_for_usd_sale => restrict_to: [admin, super_admin, OWNER];
            reserve_nfts_for_usd_sale => restrict_to: [admin, super_admin, OWNER];            
            
            // Earnings
            get_xrd => restrict_to: [OWNER];             
            get_all_xrd => restrict_to: [OWNER];             

            // just for the case the SC is broken so we can still get the minted NFTs out and sell them otherwise
            get_nfts_emergency => restrict_to: [OWNER];                                                                 

            // config settings
            set_check_assertions => restrict_to: [admin, super_admin, OWNER];  
            set_max_assign_at_once => restrict_to: [admin, super_admin, OWNER];  

            get_ph_bucket => PUBLIC;
            get_latest_usd_price => PUBLIC;
            get_internal_state => PUBLIC;
            set_do_check_for_same_transaction => PUBLIC;                       
        }
    }

        //struct Pyro<'a> {
        struct Pyro {
            
            // strings
            collection_name: String,     
            ph_nft_description: String,
            ph_nft_filenames: Vec<String>,   

            // status
            status_minting_finished: bool,
            status_sale_paused: bool,
            status_sale_started: bool,              

            // constants
            max_collection_size: u16,
            amount_nfts_for_team: u16, // NEW

            // Price
            price_nft_usd_stage1: Decimal,              // current nft price            
            price_nft_usd_stage2: Decimal,              // current nft price            
            price_nft_usd_stage3: Decimal,              // current nft price      
            latest_usd_price: Decimal,      

            amount_stage1: u16, 
            amount_stage2: u16, 
            amount_stage3: u16, 
            //amount_nfts: u16, 

            // resource managers                 
            rm_badge_owner: ResourceManager, 
            rm_badge_super_admin: ResourceManager, 
            rm_badge_admin: ResourceManager,                         
            rm_pyro_nft: ResourceManager, 
            rm_pyro_ph: ResourceManager,        // resource manager for placeholder nfts   


            // vaults
            vault_nfts : Vault,                 // all minted nfts which were not sold/redeemed yet        
            vault_collected_xrd: Vault,         // XRDs collected from sale which is not withdrawn yet
            vault_phs : Vault,                  // all minted phs which were not sold yet
            // vault_phs_burned: Vault,            // workaround since burning a placeholder results in wallet crash we put it into this vault instead                                    
                        
            
            // Counters and datastructures
            
            // minting
            ct_minted_nfts_total: u16,          // total amount of minted nfts
            ct_minted_nft_sale : u16,           // of which how many are to be sold
            ct_minted_givaways : u16,           // of which how many are giveaways (meaning they are not being sold)
            ct_phs_for_team_sent: u16, // 

            // list of nft ids
            nft_ids: Vec<u16>,                  // list/vector with all minted nft ids                        

            // placeholders
            ct_phs_sold_total: u16,             // total amount of phs "sold" including the ones for team members and USD
            ct_phs_sold_xrd: u16,       // amount of phs sold against xrd or team -> stats only
            ct_phs_sold_usd: u16,       // amount of phs sold against xrd or team -> stats only
            ct_phs_sold_team: u16,       // amount of phs sold against xrd or team -> stats only
            ct_phs_mapped: u16,                 // indicating that ids 1...ct_phs_mapped were assigned a real nft id (stored in mapping)            
            ct_phs_redeemed: u16,               // indicating how many placeholders were already changed into real nfts - stats only
            

            // mapping phs->nfts
            mapping_ph_nft: Vec<u16>,            // mapping PH id -> Pyro Id
            
            // usd
            ct_sold_usd_total: u16,             // NEW - amount of phs and nfts sold against USD (and redeemed in one step!)
            ct_sold_usd_just_reserved: u16,      // NEW - amount of phs and nfts sold against USD (and redeemed in one step!)            

            // capacities for xrd sale and voucher sale
            cap_left_sale: u16,             // CHANGED amount of nfts reserved
            cap_buffer_sale_usd: u16,             // how many voucher tokens can (still) be minted            
            
            // coupons
            coupons: KeyValueStore<String, bool>, // make sure coupon code is not redeemed twice (so method is not called twice for same coupon)                     
            last_random_based_on_trans_hash: u32, 

            max_amount_nfts_per_buy:u16, 
            latest_mint_id:u16, 
            check_assertions:bool, 
            max_assign_at_once: u16, 
            manual_usd_price:Decimal, 
            use_manual_usd_price: bool,
            ct_phs_unassigned:u16, 
            do_check_for_same_transaction:bool
        }
    

    impl Pyro {        
        pub fn instantiate_pyro(price: Decimal, collection_name: String, 
            nft_name:String, nft_desc:String, nft_info_url:String, nft_icon_url:String, 
            ph_name:String, ph_desc:String, ph_info_url:String, ph_icon_url:String, 
            ph_nft_description: String, ph_nft_filenames: Vec<String>, max_coll_size:u16, amount_nfts_for_team: u16, 
            cap_buffer_sale_usd: u16, dapp_definition_address:ComponentAddress, max_amount_nfts_per_buy:u16, 
            max_assign_at_once: u16
            )             
            -> (Global<Pyro>, FungibleBucket, FungibleBucket, FungibleBucket)  { 
                       
            
            // we need component_address to allow the component to mint new badges
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(Pyro::blueprint_id()); 
            
            /*  create badges */

            //owner badge            
            let mut tags: Vec<String> = Vec::new();
            tags.push(String::from("badge"));            
            
            let rm_badge_owner_bucket = ResourceBuilder::new_fungible(OwnerRole::None)                
                .metadata(metadata!(
                    init {
                        "name" => format!("{}{}", collection_name, " - Owner Badge".to_string()), locked;
                        "tags" => tags, locked;   
                        "dapp_definitions" => vec!(GlobalAddress::from(dapp_definition_address)), updatable;                     
                    }
                ))
                .divisibility(DIVISIBILITY_NONE)                
                .mint_roles(mint_roles!(
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(deny_all);
                 ))
                 .mint_initial_supply(1);                
            
            let owner_badge_address = rm_badge_owner_bucket.resource_address();

            //super admin badge
            tags = Vec::new();
            tags.push(String::from("badge"));            
            
            //let rm_badge_super_admin = ResourceBuilder::new_fungible(OwnerRole::None)                
            let rm_badge_super_admin_bucket = ResourceBuilder::new_fungible(OwnerRole::None)                
                .metadata(metadata!(
                    init {
                        "name" => format!("{}{}", collection_name, " - Super Admin Badge".to_string()), locked;
                        "tags" => tags, locked;   
                        "dapp_definitions" => vec!(GlobalAddress::from(dapp_definition_address)), updatable;                     
                    }
                ))
                .divisibility(DIVISIBILITY_NONE)
                .mint_roles(mint_roles!(
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(deny_all);
                 ))         
                 .mint_initial_supply(1);

            let super_admin_badge_address = rm_badge_super_admin_bucket.resource_address();   

            //super admin badge
            tags = Vec::new();
            tags.push(String::from("badge"));            

            let rm_badge_admin_bucket = ResourceBuilder::new_fungible(OwnerRole::None)                
                .metadata(metadata!(
                    init {
                        "name" => format!("{}{}", collection_name, " - Admin Badge".to_string()), locked;
                        "tags" => tags, locked;   
                        "dapp_definitions" => vec!(GlobalAddress::from(dapp_definition_address)), updatable;                     
                    }
                ))
                .divisibility(DIVISIBILITY_NONE)
                .mint_roles(mint_roles!(
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(deny_all);
                 ))          
                 .mint_initial_supply(1);

            let admin_badge_address = rm_badge_admin_bucket.resource_address();                                                                  

            // create NFT resource - mint badge required
            let rm_pyro_nft =
                    ResourceBuilder::new_integer_non_fungible::<PyroNFT>(OwnerRole::None) // 
                    .metadata(metadata!(
                        init {
                            "name" => nft_name, locked;
                            // "symbol" => nft_symbol, locked;
                            "description" => nft_desc, locked;
                            "info_url" => Url::of(nft_info_url), locked; //Url(nft_info_url), locked;
                            "icon_url" => Url::of(nft_icon_url), locked; //Url(nft_icon_url), locked;                            
                            "dapp_definitions" => vec!(GlobalAddress::from(dapp_definition_address)), updatable;                     

                        })					)                                                     
                    .mint_roles(mint_roles!(
                        minter => rule!(require(global_caller(component_address)));
                        minter_updater => rule!(require(global_caller(component_address)));
                     ))
                    .create_with_no_initial_supply(); 


            // create Placeholder NFT resource - mint badge required
            let rm_pyro_ph =
                    ResourceBuilder::new_integer_non_fungible::<PyroPlaceholder>(OwnerRole::None) // 
                    .metadata(metadata!(
                        init {
                            "name" => ph_name, locked;
                            // "symbol" => ph_symbol, locked;
                            "description" => ph_desc, locked;
                            "info_url" => Url::of(ph_info_url), locked;//Url(ph_info_url), locked;
                            "icon_url" => Url::of(ph_icon_url), locked; //Url(ph_icon_url), locked;
                            "dapp_definitions" => vec!(GlobalAddress::from(dapp_definition_address)), updatable;                                         

                        })					)                                
                    .mint_roles(mint_roles!(
                        minter => rule!(require(global_caller(component_address)));
                        minter_updater => rule!(require(global_caller(component_address)));
                    ))            
                    .burn_roles(burn_roles!(
                        burner => rule!(require(global_caller(component_address)));
                        burner_updater => rule!(deny_all);                        
                    ))                                          
                    .create_with_no_initial_supply();    

            // init usd price
            let usd_price = Runtime::get_usd_price();                 
        
            // Instantiate a Pyro component
            let pyro = Self {
                collection_name, 
                rm_pyro_nft,                 
                vault_nfts: Vault::new(rm_pyro_nft.address()),
                ct_minted_nfts_total: 0u16, 
                ct_minted_nft_sale: 0u16, 
                ct_minted_givaways: 0u16, 
                ct_phs_sold_team: 0u16, 
                ct_phs_sold_xrd: 0u16, 
                ct_phs_sold_usd: 0u16, 
                ct_phs_sold_total: 0u16,
                cap_buffer_sale_usd: cap_buffer_sale_usd, 
                cap_left_sale: 0u16, 
                price_nft_usd_stage1: price, 
                price_nft_usd_stage2: price, 
                price_nft_usd_stage3: price, 
                amount_stage1: max_coll_size, 
                amount_stage2: max_coll_size, 
                amount_stage3: max_coll_size,
                vault_collected_xrd: Vault::new(XRD),
                nft_ids: Vec::new(),                 
                mapping_ph_nft: Vec::new(), 
                ct_phs_mapped: 0u16,
                ct_phs_redeemed: 0u16, 
                rm_pyro_ph, 
                vault_phs: Vault::new(rm_pyro_ph.address()), 
                ph_nft_description, 
                ph_nft_filenames: ph_nft_filenames.clone(),                 
                rm_badge_admin: rm_badge_admin_bucket.resource_manager(), 
                rm_badge_super_admin: rm_badge_super_admin_bucket.resource_manager(), 
                rm_badge_owner: rm_badge_owner_bucket.resource_manager(), 
                status_minting_finished: false,
                status_sale_paused: false, 
                status_sale_started: false, 
                coupons: KeyValueStore::new(), 
                max_collection_size: max_coll_size, 
                amount_nfts_for_team: amount_nfts_for_team, 
                ct_phs_for_team_sent: 0u16, 
                ct_sold_usd_just_reserved: 0u16, 
                ct_sold_usd_total: 0u16, 
                last_random_based_on_trans_hash: 0u32, 
                latest_usd_price: usd_price, 
                max_amount_nfts_per_buy: max_amount_nfts_per_buy, 
                latest_mint_id: 0u16, 
                check_assertions: true, 
                max_assign_at_once: max_assign_at_once, 
                use_manual_usd_price:false,
                manual_usd_price: Decimal::ZERO, 
                ct_phs_unassigned: 0u16, 
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
            
            (pyro, rm_badge_owner_bucket, rm_badge_super_admin_bucket, rm_badge_admin_bucket)

        }


        pub fn set_status_minting_finished(&mut self) {

            assert!(self.ct_minted_nfts_total >= self.max_collection_size, "There are only {} NFTs minted yet, but there should be {}.", self.ct_minted_nfts_total, self.max_collection_size);
            
            self.status_minting_finished = true;

            // don't allow minting for Pyro NFTs anymore and never ever again
            self.rm_pyro_nft.set_mintable(rule!(deny_all));
            self.rm_pyro_nft.lock_mintable();

            // don't allow minting for Placeholder NFTs anymore and never ever again
            self.rm_pyro_ph.set_mintable(rule!(deny_all));
            self.rm_pyro_ph.lock_mintable();

            self.check_assertions();

        }

        pub fn set_sale_started(&mut self) {

            assert!(self.ct_phs_for_team_sent == self.amount_nfts_for_team, "Please first get all placeholders for team members. {} left.", self.amount_nfts_for_team -  self.ct_phs_for_team_sent);
            
            self.status_sale_started = true;

            self.assign_placeholders_to_nfts(); // this will lead to the team placeholders being assigned right after start of sale. But it might be not enough and you might have to call it manually again multiple times

        }

        pub fn set_sale_paused(&mut self, paused: bool) {
                        
            assert!(self.status_sale_started, "Sale is not started yet.");              

            self.status_sale_paused = paused;

        }
        
        pub fn get_another_admin_badge(&mut self) -> Bucket {
            
            self.rm_badge_admin.mint(1)

        }

        pub fn get_another_super_admin_badge(&mut self) -> Bucket {
            
            self.rm_badge_super_admin.mint(1)
            
        }

        pub fn get_another_owner_badge(&mut self) -> Bucket {
            
            self.rm_badge_owner.mint(1)
            
        }


        pub fn assign_placeholders_to_nfts(&mut self) {            
            
            self.check_for_same_transaction(); // you cannot do this within same transaction as buying placeholder nfts or getting placeholder nfts for team members

            
            assert!(self.status_sale_started, "Sale is not started yet. That is why assignment from placeholders cannot be done yet.");

            let mut i = self.ct_phs_mapped;

            let mut cnt_mapped = 0u16;            

            while i < self.ct_phs_sold_total && cnt_mapped < self.max_assign_at_once
            {
                let max = self.get_nft_ids_left() as u32;

                let random = self.get_random_nr(max);

                let id = self.get_nft_id_from_list(random as usize); // removes element from nft_ids

                self.mapping_ph_nft.push(id);

                i+=1;
                cnt_mapped+=1;

                self.ct_phs_mapped+=1;
            }

            self.check_assertions();
            self.set_ct_phs_unassigned();
        }

        fn mint_nft_and_return(&mut self, nft_id:u16, 
            pyro_id: u16, pyro_name:String, pyro_desc: String, pyro_filename:String, key_image_hash:String, pyro_traits: Vec<(String, String)>,
            mint_placeholder:bool) -> (Bucket, Bucket)  { 
                                            
            assert!(!self.status_minting_finished, "Minting is already finished.");

            assert!(self.ct_minted_nfts_total < self.max_collection_size, "Max. collection size of {} is reached.", self.max_collection_size);

            assert!(pyro_traits.len() == 11, "11 Traits epected, but {} found", pyro_traits.len());

            let id = nft_id as u64;

            
            /* 1. real pyro nfts                         */

            // create data struct first
            let new_pyro_nft = PyroNFT { 
                name: String::from( pyro_name), // e.g. #00001
                description: String::from (pyro_desc),      
                key_image_url: String::from(pyro_filename),
                key_image_hash: String::from(key_image_hash),
                collection: String::from(&self.collection_name),
                id: pyro_id,
                traits: pyro_traits.clone(), 
                bracers:    (&(&pyro_traits[0]).1).to_string(), 
                ear_ring:   (&(&pyro_traits[1]).1).to_string(), 
                glasses:    (&(&pyro_traits[2]).1).to_string(), 
                head_style: (&(&pyro_traits[3]).1).to_string(), 
                necklace:   (&(&pyro_traits[4]).1).to_string(), 
                nose:       (&(&pyro_traits[5]).1).to_string(), 
                pants:      (&(&pyro_traits[6]).1).to_string(), 
                ring:       (&(&pyro_traits[7]).1).to_string(), 
                shirts:     (&(&pyro_traits[8]).1).to_string(), 
                tattoos:    (&(&pyro_traits[9]).1).to_string(), 
                wall:       (&(&pyro_traits[10]).1).to_string()
            };    
            
            let rm_nft = self.rm_pyro_nft;         

            // mint the NFT
            let real_nft = rm_nft.mint_non_fungible(
                &NonFungibleLocalId::Integer(id.into()),
                new_pyro_nft
            );

            /*  2. dummy placeholder nfts            */

            let rm_ph = self.rm_pyro_ph;         
            let mut ph_nft = Bucket::new(rm_ph.address());

            if mint_placeholder
            {                
                let ph_id: u64 = self.ct_minted_nft_sale as u64 + 1;
                                
                let ph_filename_index = (ph_id as usize) % self.ph_nft_filenames.len();
                let ph_nft_filename = &self.ph_nft_filenames[ph_filename_index];                
                
                // create data struct first 
                let new_ph_nft = PyroPlaceholder {                 
                    id: ph_id as u16,                                
                    description: String::from (&self.ph_nft_description),
                    key_image_url: String::from(ph_nft_filename),
                    collection: String::from(&self.collection_name)
                };    
                            
                // mint the NFT
                let ph_nft_1 = rm_ph.mint_non_fungible(
                    &NonFungibleLocalId::Integer(ph_id.into()),
                    new_ph_nft
                );

                ph_nft.put(ph_nft_1);
            };

            self.check_assertions();

            (real_nft, ph_nft)

        }
        
        fn add_nft_to_list(&mut self, nft: Bucket)
        {
            self.vault_nfts.put(nft);
        }

        fn add_ph_to_list(&mut self, ph: Bucket)
        {
            self.vault_phs.put(ph);
        }
        
        fn get_nft_id_from_list(&mut self, pos:usize) -> u16
        {
            return self.nft_ids.remove(pos);
        }

        fn get_nft_ids_left(&mut self) -> usize
        {
            return self.nft_ids.len();
        }

        fn get_ph_id_from_list(&mut self) -> u16
        {
            return self.ct_phs_sold_total + 1;
        }

        fn get_ph_from_list(&mut self) -> Bucket
        {
            let id = self.get_ph_id_from_list() as u64;

            assert!(id>=1 && id<= (self.ct_minted_nfts_total as u64), "id is {}, but should be between {} and {}", id, 1, self.ct_minted_nfts_total);                        

            // create nf id
            let key = &NonFungibleLocalId::Integer( id.into() );
                    
            // retrieve nft based on key
            let nft:Bucket = self.vault_phs.as_non_fungible().take_non_fungible(key).into();

            nft
        }            

        // Mint a new Pyro NFT and keep it in blueprint for sale            
        pub fn mint_nft(&mut self, nft_id:u16, 
            pyro_id: u16, pyro_name:String, pyro_desc: String, pyro_filename: String, key_image_hash:String, pyro_traits: Vec<(String, String)>)  {   

            assert!(!self.status_minting_finished, "Minting is already finished.");   

            // this results in too many failed transactions: 
            // assert!(self.latest_mint_id + 1 == nft_id, "Mint id {} expected, but {} encountered.", self.latest_mint_id + 1, nft_id);                           

            // mint nft token                                                      
            let (real_nft, ph_nft) = self.mint_nft_and_return(nft_id, pyro_id, pyro_name, pyro_desc, pyro_filename, key_image_hash, pyro_traits, true);

            // add nft_id into list of nft_ids, nfts and ph_ids        
            self.nft_ids.push(nft_id);
            
            // add nft into vector of nfts
            self.add_nft_to_list(real_nft); 

            // add nft into vector of phs
            self.add_ph_to_list(ph_nft);             

            // increase counter of minted nfts (meant to be sold against xrd by default) and decrease left over nfts 
            self.ct_minted_nfts_total +=1;
            self.ct_minted_nft_sale   +=1;
            self.cap_left_sale        +=1;

            self.latest_mint_id = nft_id;

            self.check_assertions();
        }

        // Mint a new Pyro NFT and return it (e.g.) for giveaway
        pub fn mint_individual_nft_and_return_for_giveaway(&mut self, nft_id:u16, 
            pyro_id: u16, pyro_name:String, pyro_desc: String, pyro_filename:String, key_image_hash:String, pyro_traits: Vec<(String, String)>) -> (Bucket, Bucket)  {                            

            // this results in too many failed transactions: 
            // assert!(self.     + 1 == nft_id, "Mint id {} expected, but {} encountered.", self.latest_mint_id + 1, nft_id);
            
            // minting those is explicitly allowed even if official minting is finished

            let (real_nft, ph_nft) = self.mint_nft_and_return(nft_id, pyro_id, pyro_name, pyro_desc, pyro_filename, key_image_hash, pyro_traits, false);
            
            // increase counter of minted nfts, meant to be sold against xrd by default
            self.ct_minted_nfts_total +=1;
            self.ct_minted_givaways   +=1;       

            self.latest_mint_id = nft_id;

            self.check_assertions();

            (real_nft,  ph_nft)
        }

        fn get_nr_based_on_trans_hash() ->u32
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
            
            let random = Self::get_nr_based_on_trans_hash();
                                   
            let number = random % max;

            number            
        }             

        fn get_mapped_nfts(&mut self, mut placeholders:Bucket, amount:u16) -> (NonFungibleBucket, Bucket) {

            let mut bucket = Bucket::new(self.rm_pyro_nft.address()).as_non_fungible();

            let mut i = 1u16;
            while i <= amount {  

                // get id of placeholder
                let ph = placeholders.take(1);
                
                let ph_nf = ph.as_non_fungible();

                let id: NonFungibleLocalId = ph_nf.non_fungible_local_id();

                // convert id to u64
                let ph_id : u64 = match id { 
                    NonFungibleLocalId::Integer(int_id) => { int_id.value().into() } 
                    _ => { panic!("not an integer nft") }
                  };
                                  
                // get the id of the mapped nft                
                let nft_id_option = self.mapping_ph_nft.get((ph_id - 1) as usize);

                assert!(nft_id_option.is_some(), "The placeholder with id = {} was not yet assigned to a real pyro. Please try again later", ph_id);
                                
                let nft_id = *nft_id_option.unwrap();

                let nft_non_fung_id = IntegerNonFungibleLocalId::new(nft_id as u64);

                // get nft and save it in buckets for returning
                let one_nft = self.vault_nfts.as_non_fungible().take_non_fungible(&NonFungibleLocalId::Integer(nft_non_fung_id));                
                bucket.put(one_nft);                

                // burn placeholder
                self.rm_pyro_ph.burn(ph);                
                                
                self.ct_phs_redeemed += 1;

                i+=1;
            };

            (bucket, placeholders)

        }

        fn get_one_placeholder(&mut self) -> Bucket {                        

            let ph: Bucket = self.get_ph_from_list();

            assert!(ph.amount() == Decimal::ONE, "method get_one_placeholder should return ONE nft, but doesn't.");

            return ph; 
        }

        fn get_placeholders(&mut self, amount: u16, reason:ReasonPlaceholder) -> Bucket {

            let mut bucket = Bucket::new(self.rm_pyro_ph.address());

            let mut i = 1u16;

            while i <= amount {                                            
                let bucket_one = self.get_one_placeholder();

                bucket.put(bucket_one);

                i+=1;

                // update counter based on purpose of placeholder
                match reason 
                {
                    ReasonPlaceholder::SaleUsd    => self.ct_phs_sold_usd  +=1, 
                    ReasonPlaceholder::SaleXrd    => self.ct_phs_sold_xrd  +=1, 
                    ReasonPlaceholder::TeamMember => self.ct_phs_sold_team +=1, 
                }                

                self.ct_phs_sold_total +=1;                 
            };

            bucket 
        }  


        fn get_total_price(&mut self, mut amount:u16) -> Decimal {

            let mut amount_sold = self.ct_phs_sold_total + self.ct_sold_usd_just_reserved - self.ct_phs_sold_team; // don't count team pyros, giveaways are not contained in ct_phs_sold_total            
            
            let mut use_stage1 = 0u16;            
            let mut use_stage2 = 0u16;
            let mut use_stage3 = 0u16;

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
                amount_sold += use_stage2;
            }

            // calc usage of stage 3 price
            if amount_sold < self.amount_stage3
            {
                let av_stage3 = self.amount_stage3 - amount_sold;
                use_stage3 = std::cmp::min(amount, av_stage3);

                amount -= use_stage3;
            }

            assert!(amount == 0, "Price cannot be calculated. amount should be zero here, but is {}.", amount);

            // total price in usd
            let total_price_usd =   use_stage1 * self.price_nft_usd_stage1 + 
                                             use_stage2 * self.price_nft_usd_stage2 +
                                             use_stage3 * self.price_nft_usd_stage3;

            let mut usd_price = Runtime::get_usd_price();     
            if self.use_manual_usd_price
            {
                usd_price = self.manual_usd_price;
            }

            self.latest_usd_price = usd_price;

            let total_price_usd_dec = Decimal::from(total_price_usd);            
            let price_xrd = total_price_usd_dec * usd_price;            

            price_xrd
        }
        
        // buy place holders via XRD
        pub fn buy_placeholders (&mut self, mut payment: Bucket, amount: u16) -> (Bucket, Bucket) {  

            assert!(self.status_minting_finished, "Minting is not finished yet.");  

            assert!(self.status_sale_started, "Sale is not started yet.");  

            assert!(!self.status_sale_paused, "Sale is currently paused.");  

            assert!(amount<=self.max_amount_nfts_per_buy, "You can only buy {} NFTs at once.", self.max_amount_nfts_per_buy);

            assert!(payment.resource_address() == XRD, "You can only buy with XRD.");

            assert!(amount > 0, "Amount should be greater than zero, but is {}.", amount);

            // check if amount less or equal than available nfts 
            assert!(self.cap_buffer_sale_usd <= self.cap_left_sale, "There are no more NFTS left for XRD sale due to USD buffer."); //do this check to prevent next check from "overflow"
            assert!(amount<=self.cap_left_sale - self.cap_buffer_sale_usd, "Only {} NFTs left for sale, but {} needed.", self.cap_left_sale - self.cap_buffer_sale_usd, amount);

            // calc price, take it from payment and put it into collected xrd vault
            let total = self.get_total_price(amount);
            self.vault_collected_xrd.put(payment.take(total));                                
            
            // retrieve placeholder nfts which can be changed into real nfts in a 2nd step
            let sold_phs= self.get_placeholders(amount, ReasonPlaceholder::SaleXrd);                                                                                

            // increase sold counter and decrease left counter                         
            self.cap_left_sale -= amount;

            // make sure caller gets requested amount of placeholder NFTs back
            assert!(sold_phs.amount() == Decimal::from(amount), "{} placeholder NFTs should be bought, but only {} would be returned", amount, sold_phs.amount());                                            

            // make sure you cannot assign placeholders in same transaction
            self.set_last_random_based_on_trans_hash(); 
            
            self.check_assertions();

            self.set_ct_phs_unassigned();

            // Return the NFT and change
            (sold_phs, payment)  
        }
        
        // change placeholders into real pyro nfts
        pub fn change_placeholders_into_nfts(&mut self, placeholders: Bucket, amount: u16) -> (NonFungibleBucket, Bucket) {                                     
            
            assert!(amount > 0, "Amount should be greater than zero, but is {}.", amount);
            
            // check if placeholders resource address is the right one.
            assert!(placeholders.resource_address() == self.vault_phs.resource_address(), "Placeholder adress is {} but should be {}", placeholders.resource_address().to_hex(), self.vault_phs.resource_address().to_hex() );

            assert!(self.status_sale_started, "Sale is not started yet. That is why changing placeholders into individual NFTs cannot be done yet.");

            // check if there are enough placeholder nfts
            assert!(placeholders.amount()>= Decimal::from(amount), "Only {} placeholders in bucket, but {} needed.", placeholders.amount(), amount);
                        
            // retrieve "random" nfts which were actually mapped in assign method before
            let (nfts, placeholders) = self.get_mapped_nfts(placeholders, amount);
            
            // make sure caller gets requested amount of NFTs
            assert!(nfts.amount() == Decimal::from(amount), "{} nfts should be bought, but only {} would be returned", amount, nfts.amount());
            
            self.check_assertions();

            // Return the NFT and placeholders
            (nfts, placeholders) 
        }                    

        // get nfts for team        
        pub fn get_placeholders_for_team(&mut self, amount:u16) -> Bucket
        {                                       
            assert!(self.status_minting_finished, "Minting is not finished yet.");

            assert!(self.ct_phs_for_team_sent + amount <= self.amount_nfts_for_team, 
                "There are only {} NFTs for team, but you want to get {} in total", self.amount_nfts_for_team, self.ct_phs_for_team_sent + amount);                                    

            assert!(amount > 0, "Amount should be greater than zero, but is {}.", amount);

            let phs = self.get_placeholders(amount, ReasonPlaceholder::TeamMember);

            self.ct_phs_for_team_sent += amount;

            self.cap_left_sale -= amount; 

            self.set_last_random_based_on_trans_hash(); // make sure you cannot assign placeholders in same transaction
            
            self.check_assertions();
            self.set_ct_phs_unassigned();

            phs

        }
        
        // get nfts for sending them to coupon owners
        pub fn get_nfts_for_usd_sale(&mut self, coupon_code:String, amount: u16) -> (NonFungibleBucket, Bucket)
        {                           
            assert!(self.status_minting_finished, "Minting is not finished yet.");  
            assert!(self.status_sale_started, "Sale is not started yet.");  
            assert!(!self.status_sale_paused, "Sale is currently paused.");  
            assert!(amount > 0, "Amount should be greater than zero, but is {}.", amount);

            // TC 6.27a
            // don't check this if they were reserved before: 
            // assert!(amount<=self.cap_left_sale, "Only {} NFTs left for sale, but {} needed.", self.cap_left_sale, amount);
            // --> move into match:None below
                        
            // check if coupon was already redeemed            
            /*  coupons(Code)=
                a) undefined -> coupon was not yet used at all
                b) false -> coupon was used to reserve, but no NFTs were returned yet
                c) true  -> coupon was used and NFTs were already returned
            */

            let coupon_bool = self.coupons.remove(&coupon_code);            

            let allowed:bool;

            match coupon_bool {
                Some(claimed)=>
                {
                    allowed = !claimed;   // coupon was only used to reserve NFTs, but NFTs were not yet returned

                    self.ct_sold_usd_just_reserved -= amount; // those 'appear' in self.ct_sold_usd_and_redeemed

                }, 
                None => 
                {
                    allowed = true; // coupon was not yet used at all.             

                    // TC 6.27a
                    assert!(amount<=self.cap_left_sale, "Only {} NFTs left for sale, but {} needed.", self.cap_left_sale, amount);
                    
                    self.cap_left_sale     -= amount; // there was no reservation done before                    
                    self.ct_sold_usd_total += amount;
                }
            }

            if allowed
            {
                self.coupons.insert(coupon_code, true);
            }
            else
            {
                assert!(allowed, "Coupon code {} was already redeemed.", coupon_code);
            }                                            

            let phs = self.get_placeholders(amount, ReasonPlaceholder::SaleUsd);

            self.assign_placeholders_to_nfts();

            let (nfts, phs_left) = self.change_placeholders_into_nfts(phs, amount);

            assert!(phs_left.is_empty(), "There should be no placeholders left, but there are {} left.", phs_left.amount());    

            self.check_assertions();        

            (nfts, phs_left)

        }


        pub fn reserve_nfts_for_usd_sale(&mut self, coupon_code:String, amount: u16)
        {                           
            assert!(self.status_minting_finished, "Minting is not finished yet.");  
            assert!(self.status_sale_started, "Sale is not started yet.");  
            assert!(!self.status_sale_paused, "Sale is currently paused.");  
            assert!(amount<=self.cap_left_sale, "Only {} NFTs left for sale, but {} needed for reservation.", self.cap_left_sale, amount);
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

                    self.cap_left_sale             -= amount; // this is the actual reservation
                    self.ct_sold_usd_just_reserved += amount;

                    self.ct_sold_usd_total += amount;
                }
            }

            if allowed
            {
                self.coupons.insert(coupon_code, false); //false = only reseverd, not yet redeemed.
            }
            else
            {
                assert!(allowed, "NFTs for this coupon code {} were already reserved.", coupon_code);
            }         

            self.check_assertions();                                   

        }

        // change nft price
        pub fn set_price(&mut self, price1:Decimal, price2:Decimal, price3:Decimal, amount_stage1:u16, amount_stage2:u16)
        {
            assert!(price1 > Decimal::ZERO, "Price1 should be greater than zero, but is {}.", price1);
            assert!(price2 > Decimal::ZERO, "Price2 should be greater than zero, but is {}.", price2);
            assert!(price3 > Decimal::ZERO, "Price3 should be greater than zero, but is {}.", price3);            
            assert!(amount_stage2 <= self.max_collection_size, "Amount for stage 2 must be less or equal {}, but is {}.", self.max_collection_size, amount_stage2);
            assert!(amount_stage1 <= amount_stage2, "Amount for stage 2 must be greater or equal amount_stage1, but is {} > {}.", amount_stage1, amount_stage2);

            self.price_nft_usd_stage1 = price1;
            self.price_nft_usd_stage2 = price2;
            self.price_nft_usd_stage3 = price3;

            self.amount_stage1 = amount_stage1;
            self.amount_stage2 = amount_stage2;
            self.amount_stage3 = self.max_collection_size;
        }

        // withdraw a certain amount of xrd
        pub fn get_xrd(&mut self, amount:Decimal) -> Bucket 
        {
            assert!(amount > Decimal::ZERO, "Amount should be greater than zero, but is {}.", amount);

            return self.vault_collected_xrd.take(amount);
        }

        // withdraw all collected xrds
        pub fn get_all_xrd(&mut self) -> Bucket 
        {
            return self.vault_collected_xrd.take_all();
        }

        // set capacity for "vouchers" meaning that this amount cannot be sold against xrd
        pub fn set_buffer_usd_sale(&mut self, capacity:u16)
        {                                    
            assert!(capacity <= self.cap_left_sale, "Capacity should be less or equal left capacity ({}), but is {}.", self.cap_left_sale, capacity);

            self.cap_buffer_sale_usd = capacity;            
        }                                         

        pub fn set_check_assertions(&mut self, check:bool)
        {                        
            self.check_assertions = check;            
        }       

        fn set_last_random_based_on_trans_hash(&mut self)
        {
            self.last_random_based_on_trans_hash = Self::get_nr_based_on_trans_hash();
        }

        fn check_for_same_transaction(&mut self)
        {
            let latest_trans = self.last_random_based_on_trans_hash;
            let this_trans = Self::get_nr_based_on_trans_hash();

            if self.do_check_for_same_transaction
            {
                assert!(latest_trans != this_trans, "You cannot buy placeholder/get placeholder for team within the same transaction.");
            }            
        }

        fn check_assertions(&mut self)
        {
            if self.check_assertions
            {
                assert!(!self.status_minting_finished || (self.ct_minted_nfts_total == self.ct_minted_nft_sale + self.ct_minted_givaways), "After minting is finished ct_minted_nfts_total should be ct_minted_nft_sale + ct_minted_givaways.");
                assert!(!self.status_minting_finished || (self.ct_minted_nfts_total == self.max_collection_size), "After minting is finished ct_minted_nfts_total should be max_collection_size.");
                assert!(self.vault_phs.amount() == Decimal::from(self.cap_left_sale + self.ct_sold_usd_just_reserved), "self.vault_phs.amount should be equal to cap_left_sale + ct_sold_usd_just_reserved, but it is {}.", self.vault_phs.amount());                
            }
        }

        pub fn get_nfts_emergency(&mut self, mut amount:Decimal) -> Bucket
        {           
            assert!(amount > Decimal::ZERO, "Amount should be greater than zero, but is {}.", amount);
            assert!(self.status_sale_paused, "Sales must be paused/stopped before you can get NFTs from SC.");

            if amount > self.vault_nfts.amount()
            {
                amount = self.vault_nfts.amount();
            }

            self.vault_nfts.take (amount)
        }

        pub fn set_max_assign_at_once(&mut self, max_assign_at_once:u16)
        {
            assert!(max_assign_at_once > 0, "Max_assign_at_once should be greater than zero, but is {}.", max_assign_at_once);

            self.max_assign_at_once = max_assign_at_once;
        }

        pub fn use_manual_usd_price(&mut self, manual_usd_price: Decimal) {
                        
            assert!(manual_usd_price > Decimal::ZERO, "Price must be greater than zero.");              

            self.use_manual_usd_price = true;
            self.manual_usd_price = manual_usd_price;
            self.latest_usd_price = manual_usd_price;
        }

        pub fn use_runtime_usd_price(&mut self) {                                                

            self.use_manual_usd_price = false;            

            self.latest_usd_price = Runtime::get_usd_price();

        }

        fn set_ct_phs_unassigned(&mut self)
        {
            self.ct_phs_unassigned = self.ct_phs_sold_total - self.ct_phs_mapped;
        }

        pub fn get_ph_bucket(&self) -> Bucket
        {
            Bucket::new(self.rm_pyro_ph.address())
        }

        pub fn get_latest_usd_price(&self) -> Decimal 
        {
            self.latest_usd_price
        }

        pub fn get_internal_state(&self) -> (Decimal, u16, u16, Decimal, u16, u16, u16, Decimal)
        {
            let a = self.vault_phs.amount();

            let b = self.ct_phs_sold_total;

            let c = self.mapping_ph_nft.len() as u16;

            let d = self.vault_nfts.amount();

            let e = self.nft_ids.len() as u16;

            let f = self.ct_sold_usd_just_reserved;

            let g = self.cap_buffer_sale_usd;

            let h = self.vault_collected_xrd.amount();

            (a, b, c, d, e, f, g, h)

        }

        pub fn set_do_check_for_same_transaction(&mut self, do_check:bool)
        {
            self.do_check_for_same_transaction = do_check;
        }
    }
}