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

#[blueprint]
mod pyrominting {

    enable_method_auth! { 
        roles {
            super_admin => updatable_by: [];
            admin => updatable_by: [super_admin];
        },       

        methods {             
            // Minting
            mint_pyro_nft => restrict_to: [admin, super_admin, OWNER];  // hot wallet
            mint_placeholder_nft => restrict_to: [admin, super_admin, OWNER]; //             
            finish_minting => restrict_to: [admin, super_admin, OWNER]; //                         
        }
    }

        //struct Pyro<'a> {
        struct PyroMinting {            
            // strings
            collection_name: String,     
            placeholder_nft_description: String,
            placeholder_nft_filenames: Vec<String>,   

            // status            
            status_minting_finished: bool, 
            // status_sale_started: bool,              

            // constants
            max_collection_size: u16,            

            // resource managers                             
            pyro_nft_manager: ResourceManager, 
            placeholder_nft_manager: ResourceManager,        // resource manager for placeholder nfts   
            
            // Counters and datastructures
            
            // minting
            minted_pyro_nfts: u16,       
            minted_placeholder_nfts: u16,
            expected_nft_traits_amount: u16
        }    

    impl PyroMinting {        
        pub fn instantiate(
            owner_badge_address:ResourceAddress, 
            collection_name: String, 
            nft_name:String, pyro_nft_description:String, nft_info_url:String, nft_icon_url:String, 
            placeholder_name:String, placeholder_description:String, placeholder_info_url:String, placeholder_icon_url:String, placeholder_nft_description: String, 
            placeholder_nft_filenames: Vec<String>, 
            max_collection_size:u16, expected_nft_traits_amount:u16
            )             
            -> (Global<PyroMinting>, ResourceAddress, ResourceAddress)  { 
                       
            
            // we need component_address to allow the component to mint new badges
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(PyroMinting::blueprint_id());                                                                                        

            // create NFT resource - mint badge required
            let pyro_nft_manager =
                    ResourceBuilder::new_integer_non_fungible::<PyroNFT>(OwnerRole::None) // 
                    .metadata(metadata!(
                        init {
                            "name" => nft_name, locked;
                            // "symbol" => nft_symbol, locked;
                            "description" => pyro_nft_description, locked;
                            "info_url" => Url::of(nft_info_url), locked; //Url(nft_info_url), locked;
                            "icon_url" => Url::of(nft_icon_url), locked; //Url(nft_icon_url), locked;                                                        

                        })					)                                                     
                    .mint_roles(mint_roles!(
                        minter => rule!(require(global_caller(component_address)));
                        minter_updater => rule!(require(global_caller(component_address)));
                     ))
                    .create_with_no_initial_supply(); 


            // create Placeholder NFT resource - mint badge required
            let placeholder_nft_manager =
                    ResourceBuilder::new_integer_non_fungible::<PyroPlaceholder>(OwnerRole::None) // 
                    .metadata(metadata!(
                        init {
                            "name" => placeholder_name, locked;
                            // "symbol" => placeholder_symbol, locked;
                            "description" => placeholder_description, locked;
                            "info_url" => Url::of(placeholder_info_url), locked;//Url(placeholder_info_url), locked;
                            "icon_url" => Url::of(placeholder_icon_url), locked; //Url(placeholder_icon_url), locked;                            

                        })					)                                
                    .mint_roles(mint_roles!(
                        minter => rule!(require(global_caller(component_address)));
                        minter_updater => rule!(require(global_caller(component_address)));
                    ))            
                    .burn_roles(burn_roles!(
                        //burner => rule!(require(global_caller(component_address)));
                        burner => rule!(allow_all); // allow "anybody" to burn them, especially our sale smart contract
                        burner_updater => rule!(deny_all);                        
                    ))                                          
                    .create_with_no_initial_supply();            
        
            // Instantiate a Pyro component
            let pyro = Self {                
                placeholder_nft_description, 
                placeholder_nft_filenames: placeholder_nft_filenames.clone(),                                 
                pyro_nft_manager,             
                placeholder_nft_manager,                 
                collection_name,    
                max_collection_size,                 
                expected_nft_traits_amount,
                minted_pyro_nfts: 0u16, 
                minted_placeholder_nfts: 0u16,                                                 
                status_minting_finished: false,                                 
            }
            .instantiate()            
            .prepare_to_globalize(OwnerRole::Fixed(
                rule!(require(owner_badge_address))))
            /*.roles( 
                roles!(
                    super_admin => rule!(require_amount(dec!(1), super_admin_badge_address)); 
                    admin => rule!(require(admin_badge_address));
                )
            ) */
            .with_address(address_reservation)                             
            .globalize();
            
            (pyro, pyro_nft_manager.address(), placeholder_nft_manager.address())

        }        

        pub fn mint_pyro_nft(&mut self, nft_id:u16, 
            pyro_id: u16, pyro_name:String, pyro_desc: String, pyro_filename:String, key_image_hash:String, pyro_traits: Vec<(String, String)>) -> Bucket  {                                                                                 

            let id = nft_id as u64;
                                
            assert!(self.minted_pyro_nfts < self.max_collection_size, "Max. collection size of {} is reached.", self.max_collection_size);
            assert!(pyro_traits.len() as u16 == self.expected_nft_traits_amount, "{} Traits epected, but {} found", self.expected_nft_traits_amount, pyro_traits.len());

            // create data struct first
            let nft_data = PyroNFT { 
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
            
            let rm_nft = self.pyro_nft_manager;         

            // mint the NFT
            let nft = rm_nft.mint_non_fungible(
                &NonFungibleLocalId::Integer(id.into()),
                nft_data
            );

            self.minted_pyro_nfts +=1;

            nft                                 
        }                

        pub fn mint_placeholder_nft(&mut self) -> Bucket  {                                                                                 

            assert!(self.minted_placeholder_nfts < self.max_collection_size, "Max. collection size of {} is reached.", self.max_collection_size);

            let rm_ph = self.placeholder_nft_manager;
            
            let placeholder_id: u64 = self.minted_placeholder_nfts as u64 + 1;
                            
            let placeholder_filename_index = (placeholder_id as usize) % self.placeholder_nft_filenames.len();
            let placeholder_nft_filename = &self.placeholder_nft_filenames[placeholder_filename_index];                
            
            // create data struct first 
            let nft_data = PyroPlaceholder {                 
                id: placeholder_id as u16,                                
                description: String::from (&self.placeholder_nft_description),
                key_image_url: String::from(placeholder_nft_filename),
                collection: String::from(&self.collection_name)
            };    
                        
            // mint the NFT
            let nft = rm_ph.mint_non_fungible(
                &NonFungibleLocalId::Integer(placeholder_id.into()),
                nft_data
            );

            self.minted_placeholder_nfts +=1;

            nft
        }

        pub fn finish_minting(&mut self) {

            assert!(self.minted_pyro_nfts >= self.max_collection_size, "There are only {} NFTs minted yet, but there should be {}.", self.minted_pyro_nfts, self.max_collection_size);
            
            self.status_minting_finished = true;

            // don't allow minting for Pyro NFTs anymore and never ever again
            self.pyro_nft_manager.set_mintable(rule!(deny_all));
            self.pyro_nft_manager.lock_mintable();

            // don't allow minting for Placeholder NFTs anymore and never ever again
            self.placeholder_nft_manager.set_mintable(rule!(deny_all));
            self.placeholder_nft_manager.lock_mintable();
        }
    }
}