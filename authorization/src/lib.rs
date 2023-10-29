use scrypto::prelude::*;

#[blueprint]
mod pyroauth {

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
        }
    }

        struct PyroAuthorization {                        
            // resource managers                 
            rm_badge_owner: ResourceManager, 
            rm_badge_super_admin: ResourceManager, 
            rm_badge_admin: ResourceManager
        }
    

    impl PyroAuthorization {        
        pub fn instantiate(collection_name:String, dapp_definition_address:ComponentAddress)             
            -> (Global<PyroAuthorization>, Bucket, Bucket, Bucket)  { 
                
            // we need component_address to allow the component to mint new badges
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(PyroAuthorization::blueprint_id()); 
            
            // create badges

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

            //admin badge
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
        
            // Instantiate a Pyro component
            let pyro_authorization = Self {                                
                rm_badge_admin: rm_badge_admin_bucket.resource_manager(), 
                rm_badge_super_admin: rm_badge_super_admin_bucket.resource_manager(), 
                rm_badge_owner: rm_badge_owner_bucket.resource_manager(),                 
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
            
            (pyro_authorization, rm_badge_owner_bucket.into(), rm_badge_super_admin_bucket.into(), rm_badge_admin_bucket.into())

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
    }
}