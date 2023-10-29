//use pretty_assertions::assert_eq;
use sale::test_bindings::*;
use minting::test_bindings::*;
use authorization::test_bindings::*;
use dummy_account::test_bindings::*;
use scrypto::this_package;
use scrypto_test::prelude::*;

pub struct MigrationHelper {
    pub env: TestEnvironment,
    pub dapp_definition: ComponentAddress,
    pub sale_package_address: PackageAddress,
    pub auth_package_address: PackageAddress,
    pub mint_package_address: PackageAddress,
    pub xrd_token: Bucket,
    pub non_xrd_token: Bucket,        
    pub xrd_address: ResourceAddress,
    pub non_xrd_address: ResourceAddress,        
    pub pyro_sale: Option<PyroSale>,
    pub pyro_minting: Option<PyroMinting>,    
    pub pyro_auth: Option<PyroAuthorization>,    
    pub phs_bucket: Option<Bucket>, 
    pub pyros_bucket: Option<Bucket>, 
    pub latest_usd_price: Decimal
}


impl MigrationHelper {

    pub fn new() -> Result<Self, RuntimeError> {
        
        let mut env = TestEnvironment::new();   

        env.disable_auth_module();
        env.disable_limits_module();

        env.enable_costing_module();        
        env.call_method_typed::<_, _, ()>(FAUCET, "lock_fee", &(dec!("123456"),))
        .unwrap();
        env.disable_costing_module();     

        let sale_package_address = Package::compile_and_publish(this_package!(), &mut env)?;            
        
        let auth_package_address = Package::compile_and_publish(  
            "./authorization", 
            &mut env
        )?;
        
        let mint_package_address = Package::compile_and_publish( 
            "./minting", 
            &mut env
        )?;
    
        let dummy_account_package_address = Package::compile_and_publish(
            "./dummy_account",
            &mut env
        )?;
        
        let account = DummyAccount::instantiate(dummy_account_package_address, &mut env)?;
        let dapp_definition = account.address(&mut env)?;

        let non_xrd_token = ResourceBuilder::new_fungible(OwnerRole::None)
            .divisibility(18)
            .mint_initial_supply(100000, &mut env)?;        

        let xrd_token = BucketFactory::create_fungible_bucket(
            XRD,
            100000.into(),
            Mock,
            &mut env
        )?;


        let non_xrd_address = non_xrd_token.resource_address(&mut env)?;
        let xrd_address = xrd_token.resource_address(&mut env)?;
    
        Ok(Self {
            env,
            dapp_definition,
            sale_package_address,
            auth_package_address, 
            mint_package_address, 
            xrd_token,
            non_xrd_token,
            xrd_address,
            non_xrd_address,            
            pyro_sale: None,   
            pyro_minting: None,  
            pyro_auth: None,         
            phs_bucket: None, 
            pyros_bucket: None,            
            latest_usd_price: Decimal::ZERO
        })

    
    }

    pub fn instantiate(
        &mut self,      
        max_collection_size:u16,         
        price:Decimal
    ) -> Result<(), RuntimeError> {      
                
        let (badge_owner_bucket, badge_super_admin_bucket, badge_admin_bucket) = self.instantiate_authorization()?;
        
        let owner_badge_address = badge_owner_bucket.resource_address(&mut self.env)?;
        let super_admin_badge_address = badge_super_admin_bucket.resource_address(&mut self.env)?;
        let admin_badge_address = badge_admin_bucket.resource_address(&mut self.env)?;
        
        let (pyro_address, placeholder_address) = self.instantiate_minting( 
            owner_badge_address,     
            max_collection_size
        )?;                        

        self.instantiate_sale(
            owner_badge_address, 
            super_admin_badge_address, 
            admin_badge_address, 
            pyro_address, 
            placeholder_address, 
            price                         
        )?;        

        Ok(())

    }

    pub fn instantiate_authorization(&mut self ) -> Result<(Bucket, Bucket, Bucket), RuntimeError> {        

        let (pyro_authorization, rm_badge_owner_bucket, rm_badge_super_admin_bucket, rm_badge_admin_bucket) = 
            PyroAuthorization::instantiate(
                "Collection name".to_owned(), 
                
                self.dapp_definition, 
                self.auth_package_address, 
                &mut self.env)?;

        self.pyro_auth = Some (pyro_authorization);

        Ok((rm_badge_owner_bucket, rm_badge_super_admin_bucket, rm_badge_admin_bucket))
    }

    pub fn instantiate_minting(&mut self, owner_badge_address:ResourceAddress, max_collection_size:u16, ) -> Result<(ResourceAddress, ResourceAddress), RuntimeError> {

        let mut ph_filenames: Vec<String> = Vec::new();
                ph_filenames.push("https://test.pyros-world.com/img/Blanko_1.jpg".to_owned());                                
                ph_filenames.push("https://test.pyros-world.com/img/Blanko_2.jpg".to_owned());
                ph_filenames.push("https://test.pyros-world.com/img/Blanko_3.jpg".to_owned());
                ph_filenames.push("https://test.pyros-world.com/img/Blanko_4.jpg".to_owned());
                ph_filenames.push("https://test.pyros-world.com/img/Blanko_5.jpg".to_owned());
                ph_filenames.push("https://test.pyros-world.com/img/Blanko_6.jpg".to_owned());
                ph_filenames.push("https://test.pyros-world.com/img/Blanko_7.jpg".to_owned());
                ph_filenames.push("https://test.pyros-world.com/img/Blanko_8.jpg".to_owned());       

        let (pyro_minting, pyro_nft_address, placeholder_nft_address) = PyroMinting::instantiate(
            owner_badge_address, 
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
        11, 
        self.mint_package_address, 
        &mut self.env)?;

        self.pyro_minting = Some (pyro_minting);

        Ok((pyro_nft_address, placeholder_nft_address))
    }

    pub fn instantiate_sale(
        &mut self,      
        owner_badge_address:ResourceAddress, super_admin_badge_address:ResourceAddress, admin_badge_address:ResourceAddress,
        pyro_nfts_address: ResourceAddress, placeholder_nfts_address:ResourceAddress,         
        price:Decimal
    ) -> Result<(), RuntimeError> {                                 
        
        self.env.enable_costing_module();
        let mut pyro_sale = 
                PyroSale::instantiate_pyro(
                    owner_badge_address, 
                    super_admin_badge_address,
                    admin_badge_address, 
                    pyro_nfts_address, 
                    placeholder_nfts_address, 
                price,                                                 
                self.dapp_definition,
                50u16,                
                self.sale_package_address,                
                &mut self.env)?;
        
        self.env.disable_costing_module();

        self.pyro_sale = Some (pyro_sale);  

        pyro_sale.use_manual_usd_price(Decimal::ONE, &mut self.env)?;
        
        let b = pyro_sale.get_placeholder_bucket(&mut self.env)?;
        self.phs_bucket = Some(b);

        self.pyros_bucket = Some ( pyro_sale.get_pyro_bucket(&mut self.env)? );

        self.latest_usd_price = pyro_sale.get_latest_usd_price(&mut self.env)?;

        self.set_do_check_for_same_transaction(false).unwrap();

        Ok(())
    }


    pub fn buy_placeholders_check(&mut self, use_xrd:bool, amount_placeholders:u16, amount_token:Decimal, check_change:bool, expected_change:Decimal) -> Result<(), RuntimeError>
    {           
        // self.env.enable_costing_module();

        let mut pyro = self.pyro_sale.unwrap();

        let (a1, b1, _, _, _, _, _) = pyro.get_internal_state(&mut self.env).unwrap();

        let mut payment = self.xrd_token.take(amount_token, &mut self.env)?;
        
        if !use_xrd
        {
            payment = self.non_xrd_token.take(amount_token, &mut self.env)?;
        }                                
        
        //let (phs, change) = pyro.buy_placeholders(payment, amount_placeholders, &mut self.env)?;  
        // self.env.enable_costing_module();          
        let x = pyro.buy_placeholders(payment, amount_placeholders, &mut self.env);
        
        let (phs, change) = x.unwrap();
                
        // check return
        assert_eq!( phs.amount(&mut self.env)?, Decimal::from(amount_placeholders) );

        if check_change
        {
            assert_eq!( change.amount(&mut self.env)?, expected_change );
        }    

        assert_eq!( phs.amount(&mut self.env)?, Decimal::from(amount_placeholders) );
        
        // check internal state
        let (a2, b2, _, _, _, _, _) = pyro.get_internal_state(&mut self.env).unwrap();

        assert_eq!(a2, a1 - amount_placeholders);
        assert_eq!(b2, b1 + amount_placeholders);

        // store placeholders in bucket for later usage
        self.phs_bucket.as_mut().unwrap().put(phs, &mut self.env)?;

        Ok(())
    }

    pub fn buy_placeholders(&mut self, use_xrd:bool, amount_placeholders:u16, amount_token:Decimal) -> Result<(), RuntimeError>
    {                   
        self.buy_placeholders_check(use_xrd, amount_placeholders, amount_token, false, Decimal::ZERO)
    }

    fn mint_dummy_nft(&mut self, nft_id: u16, put_placeholder_in_sale_contract:bool) -> Result<(), RuntimeError>
    {
        let mut pyro_sale = self.pyro_sale.unwrap();
        
        let mut pyro_mint = self.pyro_minting.unwrap();

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
        
        let pyro_nft = pyro_mint.mint_pyro_nft(nft_id, pyro_id, pyro_name, pyro_desc, pyro_filename, key_image_hash, pyro_traits, &mut self.env).unwrap();
        
        // self.env.disable_costing_module();
        let placeholder_nft = pyro_mint.mint_placeholder_nft(&mut self.env).unwrap();

        let placeholder_bucket = pyro_sale.get_placeholder_bucket(&mut self.env)?;

        if put_placeholder_in_sale_contract
        {
            placeholder_bucket.put(placeholder_nft, &mut self.env)?;
        }
        else
        {
            // save in global bucket
            self.phs_bucket.as_mut().unwrap().put(placeholder_nft, &mut self.env)?;
        }

        pyro_sale.add_nfts_for_sale(nft_id, pyro_nft, placeholder_bucket, &mut self.env).unwrap();

        Ok(())
        

    }

    pub fn mint_dummy_nfts(&mut self, amount_total:u16, amount_team:u16) -> Result<(), RuntimeError>
    {
        //self.env.disable_costing_module();
                
        let mut nft_id = 1;
        
        while nft_id <= amount_total { 

            // the FIRST nft_ids from 1 .. amount_team are kept outside the SC
            let put_placeholder_in_sale_contract = nft_id > amount_team;

            self.mint_dummy_nft(nft_id, put_placeholder_in_sale_contract)?;
            nft_id+=1;
        };

        Ok(())
    }

    pub fn start_sale(&mut self) -> Result<(), RuntimeError>
    {
        let mut pyro = self.pyro_sale.unwrap();

        pyro.start_sale(&mut self.env)        
    }

    pub fn pause_sale(&mut self) -> Result<(), RuntimeError>
    {
        let mut pyro = self.pyro_sale.unwrap();

        pyro.pause_sale(&mut self.env)        
    }
    
    /*pub fn get_placeholders_for_team(&mut self, amount:u16) -> Result<(), RuntimeError>
    {
        let mut pyro = self.pyro_sale.unwrap();

        let phs = pyro.get_placeholders_for_team(amount, &mut self.env).unwrap();        
                
        let phs_amount = phs.amount(&mut self.env)?;        

        // check return
        assert_eq!(phs_amount, Decimal::from(amount));

        // store phs in bucket for later usage
        //let mut b = 
        self.phs_bucket.as_mut().unwrap().put(phs, &mut self.env)?;

        /*
        let amount_before = b.amount(&mut self.env)?;

        b.put(phs, &mut self.env)?;

        // check if changes on b affected self.phs_bucket
        b = self.phs_bucket.as_mut().unwrap();

        let amount_after = b.amount(&mut self.env)?;

        assert_eq!(amount_before + Decimal::from(amount), amount_after);
        */

        Ok(())
    }
    */

    /*pub fn set_status_minting_finished(&mut self) -> Result<(), RuntimeError>
    {
        let mut pyro = self.pyro_sale.unwrap();

        pyro.set_status_minting_finished(&mut self.env)
    }*/

    pub fn mint_till_start_sale(&mut self, amount_nfts:u16, amount_team:u16) -> Result<(), RuntimeError>
    {
        self.mint_dummy_nfts(amount_nfts, amount_team)?;
    
        // self.set_status_minting_finished().unwrap();

        /*
        let team_first = amount_team/3;
        let team_second = amount_team/4;
        let team_third = amount_team - team_first - team_second;
        
        self.get_placeholders_for_team(team_first).unwrap();
        self.get_placeholders_for_team(team_second).unwrap();
        self.get_placeholders_for_team(team_third).unwrap();        
        */
            
        self.start_sale().unwrap();

        Ok(())
    }

    pub fn set_price(&mut self, price1:Decimal, price2:Decimal, price3:Decimal, amount_stage1:u16, amount_stage2:u16) -> Result<(), RuntimeError>
    {
        let mut pyro = self.pyro_sale.unwrap();

        pyro.set_price(price1, price2, price3, amount_stage1, amount_stage2, &mut self.env)
    }

    pub fn assign_placeholders_to_nfts(&mut self) -> Result<(), RuntimeError>
    {
       self.assign_placeholders_to_nfts_check(false, 0)
    }
    
    pub fn assign_placeholders_to_nfts_check(&mut self, do_check:bool, amount_expected:u16) -> Result<(), RuntimeError>
    {
        let mut pyro = self.pyro_sale.unwrap();

        let (_, _, c1, _, e1, _, _) = pyro.get_internal_state(&mut self.env).unwrap();

        pyro.assign_placeholders_to_nfts( 20u16,&mut self.env).unwrap();

        let (_, _, c2, _, e2, _, _) = pyro.get_internal_state(&mut self.env).unwrap();

        
        // check internal state
        assert_eq!(c2-c1, e1-e2);
        
        if do_check
        {
            let amount_mapped = c2 - c1;

            assert_eq!(amount_mapped, amount_expected);
        }

        Ok(())
    }
    

    pub fn set_do_check_for_same_transaction(&mut self, do_check:bool) -> Result<(), RuntimeError>
    {
        let mut pyro = self.pyro_sale.unwrap();

        pyro.set_do_check_for_same_transaction(do_check, &mut self.env)
    }

    pub fn expect_phs_in_bucket(&mut self, amount_expected:Decimal) -> Result<(), RuntimeError>
    {
        let amount_phs = self.phs_bucket.as_mut().unwrap().amount(&mut self.env)?;

        assert_eq!(amount_phs, amount_expected);

        Ok(())
    }

    pub fn expect_pyros_in_bucket(&mut self, amount_expected:Decimal) -> Result<(), RuntimeError>
    {
        let amount_pyros = self.pyros_bucket.as_mut().unwrap().amount(&mut self.env)?;

        assert_eq!(amount_pyros, amount_expected);

        Ok(())
    }


    pub fn change_placeholders_into_nfts(&mut self, amount: u16) -> Result<(), RuntimeError>
    {
        self.change_placeholders_into_nfts_check(Decimal::from(amount), amount).unwrap();

        Ok(())
    }
    
    pub fn change_placeholders_into_nfts_check(&mut self, amount_bucket: Decimal, amount: u16) -> Result<(), RuntimeError>
    {
        let pyro = self.pyro_sale.unwrap();

        let (_, _, _, d1, _, _, _) = pyro.get_internal_state(&mut self.env)?;

        // get amount_bucket phs from internal helper bucket for changing into pyro nfts
        let mut pyro = self.pyro_sale.unwrap();  
        let bucket = self.phs_bucket.as_mut().unwrap().take(amount_bucket, &mut self.env)?;
        
        let (pyros, phs) = pyro.change_placeholders_into_nfts( bucket, amount, &mut self.env).unwrap();

        // check internal
        let (_, _, _, d2, _, _, _) = pyro.get_internal_state(&mut self.env)?;
        
        assert_eq!(d2, d1 - amount);        

        // check return
        let pyros_bucket: Bucket = pyros.into();

        assert_eq!(pyros_bucket.amount(&mut self.env)?, Decimal::from(amount) );

        let amount_phs_expected = Decimal::from(amount_bucket) - amount;

        assert_eq!(phs.amount(&mut self.env)?, amount_phs_expected);

        // store results in bucket of helper
        self.pyros_bucket.as_mut().unwrap().put( pyros_bucket, &mut self.env ).unwrap();
        self.phs_bucket.as_mut().unwrap().put( phs, &mut self.env ).unwrap();


        Ok(())
    }


    pub fn reserve_nfts_for_usd_sale(&mut self, coupon_code:String, amount: u16) -> Result<(), RuntimeError>
    {
        let mut pyro = self.pyro_sale.unwrap();

        let (a1, b1, c1, d1, e1, f1, g1) = pyro.get_internal_state(&mut self.env)?;

        pyro.reserve_nfts_for_usd_sale(coupon_code, amount, &mut self.env).unwrap();


        // check internal state
        let (a2, b2, c2, d2, e2, f2, g2) = pyro.get_internal_state(&mut self.env)?;

        assert_eq!(a1, a2);
        assert_eq!(b1, b2);
        assert_eq!(c1, c2);
        assert_eq!(d1, d2);
        assert_eq!(e1, e2);        
        assert_eq!(f1 + amount, f2); 
        assert_eq!(g1, g2);
        
        Ok(())
    }


    pub fn get_nfts_for_usd_sale(&mut self, coupon_code:String, amount: u16, was_reserved_before:bool) -> Result<(), RuntimeError>
    {
        let mut pyro = self.pyro_sale.unwrap();

        let (a1, b1, _c1, d1, _e1, f1, g1) = pyro.get_internal_state(&mut self.env)?;

        let pyro_nfts = pyro.get_nfts_for_usd_sale(coupon_code, amount, 50, &mut self.env).unwrap();
        
        // check return
        assert_eq!(pyro_nfts.amount(&mut self.env)?, Decimal::from(amount));            

        // check internal state
        let (a2, b2, _c2, d2, _e2, f2, g2) = pyro.get_internal_state(&mut self.env)?;

        assert_eq!(a1 - amount, a2);
        assert_eq!(b1 + amount, b2);
        // assert_eq!(c1 + amount, c2); we don't know how many phs were mapped 
        assert_eq!(d1 - amount, d2);
        // assert_eq!(e1 - amount, e2); ; we don't know how many phs were mapped                 
        assert_eq!(g1, g2);
        
        if was_reserved_before
        {            
            assert_eq!(f1 - amount, f2); 
        }  
        else {
            assert_eq!(f1, f2); 
        }      

        // store pyro_nfts in helper bucket
        self.pyros_bucket.as_mut().unwrap().put( pyro_nfts, &mut self.env).unwrap();
        
        Ok(())
    }

    pub fn buy_41_with_all_methods(&mut self) -> Result<(), RuntimeError>
    {
        let price = dec!("20");
        let amount_token = 5 * price * self.latest_usd_price;                

        // buy 5 ph 
        self.buy_placeholders(true, 5, amount_token).unwrap();
        self.assign_placeholders_to_nfts().unwrap();

        // reserve 3
        self.reserve_nfts_for_usd_sale("CP01".to_owned(), 3).unwrap();
        
        // reserve and get 7
        self.reserve_nfts_for_usd_sale("CP02".to_owned(), 7).unwrap();
        self.get_nfts_for_usd_sale("CP02".to_owned(), 7, true).unwrap();

        // get 9
        self.get_nfts_for_usd_sale("CP03".to_owned(), 9, false).unwrap();

        // buy 5 ph 
        self.buy_placeholders(true, 5, amount_token).unwrap();
        self.assign_placeholders_to_nfts().unwrap();   

        // reserve 12
        self.reserve_nfts_for_usd_sale("CP04".to_owned(), 12).unwrap();

        // 5+3+7+9+5+12=41 sold/reserved in SC

        // 7+9 =16 Pyro NFTs

        // 5+5 = 10 Placeholder

        // 3+12=15 reserved

        Ok(())

    }

}