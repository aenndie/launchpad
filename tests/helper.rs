//use pretty_assertions::assert_eq;
use pyro19::test_bindings::*;
use dummy_account::test_bindings::*;
use scrypto::this_package;
use scrypto_test::prelude::*;
// use scrypto::{prelude::Vault, resource::ScryptoVault, this_package};

pub struct MigrationHelper {
    pub env: TestEnvironment,
    pub dapp_definition: ComponentAddress,
    pub package_address: PackageAddress,
    pub xrd_token: Bucket,
    pub non_xrd_token: Bucket,        
    pub xrd_address: ResourceAddress,
    pub non_xrd_address: ResourceAddress,        
    pub pyro19: Option<Pyro>,
    //pub phs_vault: Vault,
    pub phs_bucket: Option<Bucket>, 
    pub latest_usd_price: Decimal
}


impl MigrationHelper {

    pub fn new() -> Result<Self, RuntimeError> {
        
        let mut env = TestEnvironment::new();   

        env.disable_auth_module();
        env.disable_limits_module();

        env.enable_costing_module();
        //env.disable_costing_module();
        env.call_method_typed::<_, _, ()>(FAUCET, "lock_fee", &(dec!("1000"),))
        .unwrap();
     

        let package_address = Package::compile_and_publish(this_package!(), &mut env)?;
    
        let dummy_account_package_address = Package::compile_and_publish(
            "./dummy_account",
            &mut env
        )?;
        
        let account = DummyAccount::instantiate(dummy_account_package_address, &mut env)?;
        let dapp_definition = account.address(&mut env)?;

        let non_xrd_token = ResourceBuilder::new_fungible(OwnerRole::None)
            .divisibility(18)
            .mint_initial_supply(1000, &mut env)?;        

        let xrd_token = BucketFactory::create_fungible_bucket(
            XRD,
            1000.into(),
            Mock,
            &mut env
        )?;


        let non_xrd_address = non_xrd_token.resource_address(&mut env)?;
        let xrd_address = xrd_token.resource_address(&mut env)?;
    
        Ok(Self {
            env,
            dapp_definition,
            package_address,
            xrd_token,
            non_xrd_token,
            xrd_address,
            non_xrd_address,            
            pyro19: None,
            // phs_vault: Vault::new( XRD ) // WORKAROUND: should be empty
            phs_bucket: None, 
            latest_usd_price: Decimal::ZERO
        })

    
    }


    pub fn instantiate(
        &mut self,      
        max_collection_size:u16, 
        amount_nfts_for_team:u16, 
        price:Decimal
    ) -> Result<(), RuntimeError> {        
        
        let mut ph_filenames: Vec<String> = Vec::new();
                ph_filenames.push("https://test.pyros-world.com/img/Blanko_1.jpg".to_owned());                                
                ph_filenames.push("https://test.pyros-world.com/img/Blanko_2.jpg".to_owned());
                ph_filenames.push("https://test.pyros-world.com/img/Blanko_3.jpg".to_owned());
                ph_filenames.push("https://test.pyros-world.com/img/Blanko_4.jpg".to_owned());
                ph_filenames.push("https://test.pyros-world.com/img/Blanko_5.jpg".to_owned());
                ph_filenames.push("https://test.pyros-world.com/img/Blanko_6.jpg".to_owned());
                ph_filenames.push("https://test.pyros-world.com/img/Blanko_7.jpg".to_owned());
                ph_filenames.push("https://test.pyros-world.com/img/Blanko_8.jpg".to_owned());
    
        
        //self.env.enable_costing_module();

        //let mut env = &self.env;
        // let mut env = TestEnvironment::new();

        // let mut env = self.env;

        
        let x = 
        //self.env.with_costing_module_enabled(
        //    |env|
            //{                                          
                //let mut hello = Hello::instantiate_hello(package_address, &mut env)?;    

                // let (pyro19, _, _, _)  = Pyro::instantiate_pyro(
                Pyro::instantiate_pyro(
                price, 
                "1st collection".to_owned(),
                "Pyros World - Test 4.10.23".to_owned(), 
                "Fully 3D and metaverse-ready NFTs with genuine utility.".to_owned(),
                "https://www.pyrosworld.com".to_owned(),
                "https://test.pyros-world.com/img/favicon.png".to_owned(),
                "Pyros World - Placeholders Test 4.10.23".to_owned(),
                "These are anonymous placeholder NFTs needed for a real random sale. Please exchange them into real individual Pyro NFTs on our website.".to_owned(),
                "https://www.pyrosworld.com".to_owned(),
                "https://test.pyros-world.com/img/favicon.png".to_owned(),
                "This NFT is a placeholder/coupon for a real NFT. Please exchange it.".to_owned(), 
                ph_filenames, 
                max_collection_size,
                amount_nfts_for_team,
                25u16,
                self.dapp_definition,
                50u16,
                20u16, 
                false, 
                self.package_address,                
                &mut self.env);
            //}
        //);        
        
        let (pyro19, _, _, _) = x?;
        self.pyro19 = Some (pyro19);  
        
        let b = pyro19.get_ph_bucket(&mut self.env)?;
        self.phs_bucket = Some(b);

        self.latest_usd_price = pyro19.get_latest_usd_price(&mut self.env)?;

        Ok(())
    }


    fn buy_placeholders_check(&mut self, use_xrd:bool, amount_placeholders:u16, amount_token:Decimal, check_change:bool, expected_change:Decimal) -> Result<(), RuntimeError>
    {           
        let mut payment = self.xrd_token.take(amount_token, &mut self.env)?;
        
        if !use_xrd
        {
            payment = self.non_xrd_token.take(amount_token, &mut self.env)?;
        }

        let mut pyro = self.pyro19.unwrap();

        let (phs, change) = pyro.buy_placeholders(payment, amount_placeholders, &mut self.env)?;

        assert_eq!( phs.amount(&mut self.env)?, Decimal::from(amount_placeholders) );

        if check_change
        {
            assert_eq!( change.amount(&mut self.env)?, expected_change );
        }

        Ok(())
    }

    pub fn buy_placeholders(&mut self, use_xrd:bool, amount_placeholders:u16, amount_token:Decimal) -> Result<(), RuntimeError>
    {           
        self.env.enable_costing_module();
        self.buy_placeholders_check(use_xrd, amount_placeholders, amount_token, false, Decimal::ZERO)
    }

    fn mint_dummy_nft(&mut self, nft_id: u16)
    {
        let mut pyro = self.pyro19.unwrap();

        let pyro_name = "Name ".to_owned() + &nft_id.to_string();
        let pyro_desc = "Description ".to_owned() + &nft_id.to_string();
        let pyro_filename = "Filename ".to_owned() + &nft_id.to_string();
        let key_image_hash = "Hash ".to_owned() + &nft_id.to_string();
        
        let pyro_traits: Vec<(String, String)> = vec![
            ("Bracer".to_owned(),       "Bracers ".to_owned()       + &nft_id.to_string()), 
            ("Ear Ring".to_owned(),     "Ear Ring ".to_owned()      + &nft_id.to_string()), 
            ("Glasses".to_owned(),      "Glasses ".to_owned()       + &nft_id.to_string()), 
            ("Head Style".to_owned(),   "Head Style ".to_owned()    + &nft_id.to_string()), 
            ("Necklace".to_owned(),     "Necklace ".to_owned()      + &nft_id.to_string()), 
            ("Nose".to_owned(),         "Nose ".to_owned()          + &nft_id.to_string()), 
            ("Pants".to_owned(),        "Pants ".to_owned()         + &nft_id.to_string()), 
            ("Ring".to_owned(),         "Ring ".to_owned()          + &nft_id.to_string()), 
            ("Shirt".to_owned(),        "Shirt ".to_owned()         + &nft_id.to_string()), 
            ("Tattoo".to_owned(),       "Tattoo ".to_owned()        + &nft_id.to_string()), 
            ("Wall".to_owned(),         "Wall ".to_owned()          + &nft_id.to_string())
        ];        

        let pyro_id = nft_id;
        pyro.mint_nft(nft_id, pyro_id, pyro_name, pyro_desc, pyro_filename, key_image_hash, pyro_traits, &mut self.env).unwrap();
    }

    pub fn mint_dummy_nfts(&mut self, amount:u16)
    {
        self.env.disable_costing_module();
                
        let mut nft_id = 1;
        
        while nft_id <= amount { 
            self.mint_dummy_nft(nft_id);
            nft_id+=1;
        }
    }

    pub fn start_sale(&mut self) -> Result<(), RuntimeError>
    {
        let mut pyro = self.pyro19.unwrap();

        pyro.set_sale_started(&mut self.env)        
    }

    pub fn pause_sale(&mut self) -> Result<(), RuntimeError>
    {
        let mut pyro = self.pyro19.unwrap();

        pyro.set_sale_paused(true, &mut self.env)        
    }
    
    pub fn get_placeholders_for_team(&mut self, amount:u16) -> Result<(), RuntimeError>
    {
        let mut pyro = self.pyro19.unwrap();

        let phs = pyro.get_placeholders_for_team(amount, &mut self.env).unwrap();        
                
        let phs_amount = phs.amount(&mut self.env)?;        

        //self.phs_bucket.unwrap().put(phs, &mut self.env);
        let b = self.phs_bucket.as_mut().unwrap();
        b.put(phs, &mut self.env)?;

        assert_eq!(phs_amount, Decimal::from(amount));

        Ok(())
    }

    pub fn set_status_minting_finished(&mut self) -> Result<(), RuntimeError>
    {
        let mut pyro = self.pyro19.unwrap();

        pyro.set_status_minting_finished(&mut self.env)
    }

    pub fn mint_till_start_sale(&mut self, amount_nfts:u16, amount_team:u16)
    {
        self.mint_dummy_nfts(amount_nfts);
    
        self.set_status_minting_finished().unwrap();

        let team_first = amount_team/3;
        let team_second = amount_team/4;
        let team_third = amount_team - team_first - team_second;

        self.get_placeholders_for_team(team_first).unwrap();
        self.get_placeholders_for_team(team_second).unwrap();
        self.get_placeholders_for_team(team_third).unwrap();
            
        self.start_sale().unwrap();
    }


}