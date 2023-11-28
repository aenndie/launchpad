//use pretty_assertions::assert_eq;
use sale::test_bindings::*;
use minting::{test_bindings::*, PyroNFT};
use authorization::test_bindings::*;
use dummy_account::test_bindings::*;
use dummy_oracle::test_bindings::*;
use scrypto::this_package; // resource::ScryptoBucket
use scrypto_test::prelude::*;

#[derive(ScryptoSbor, PartialEq)]
pub enum Action {
    AddNftsForSale,
    StartSale, 
    PauseSale, 
    ContinueSale, 
    UseManualUsdPrice, 
    UseOracleUsdPrice, 
    WithdrawXRD,
    CollectNftsInEmergency
}

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
    pub pyros_address: Option<ResourceAddress>, 
    pub placeholders_address: Option<ResourceAddress>, 
    //pub phs_bucket: Bucket, 
    //pub pyros_bucket: Bucket, 
    pub latest_usd_price: Decimal, 
    pub owner_badge_address: Option<ResourceAddress>, 
    pub super_admin_badge_address: Option<ResourceAddress>, 
    pub admin_badge_address: Option<ResourceAddress>,     
    pub dummy_oracle_address:ComponentAddress, 
    pub dummy_oracle:DummyOracle
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
        
        let dummy_account = DummyAccount::instantiate(dummy_account_package_address, &mut env)?;

        let dummy_oracle_package_address = Package::compile_and_publish(
            "./dummy_oracle",
            &mut env
        )?;
        
        let dummy_oracle = DummyOracle::instantiate(dec!("2"), dummy_oracle_package_address, &mut env)?;

        let dapp_definition = dummy_account.address(&mut env)?;

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

        let dummy_oracle_address = dummy_oracle.address(&mut env).unwrap();
    
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
            latest_usd_price: Decimal::ZERO, 
            owner_badge_address: None, 
            super_admin_badge_address: None, 
            admin_badge_address: None, 
            pyros_address: None, 
            placeholders_address: None, 
            dummy_oracle_address, 
            dummy_oracle
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
            admin_badge_address,     
            max_collection_size
        )?;                 

        self.pyros_address = Some (pyro_address);       
        self.placeholders_address = Some (placeholder_address);       

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

        self.owner_badge_address = Some (rm_badge_owner_bucket.resource_address(&mut self.env)?);
        self.super_admin_badge_address = Some (rm_badge_super_admin_bucket.resource_address(&mut self.env)?);
        self.admin_badge_address = Some (rm_badge_admin_bucket.resource_address(&mut self.env)?);

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
        "https://www.test.pyros-world.com/Home/License".to_owned(),
        "https://bafybeigloxb64wepe6rapw3lqv2456jgfj37ncq2zsf7hcch3ik24rvg7a.ipfs.nftstorage.link/".to_owned(),
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
                    self.dummy_oracle_address,           
                    self.sale_package_address,     
                    &mut self.env)?;
        
        self.env.disable_costing_module();

        self.pyro_sale = Some (pyro_sale);  

        //
        let xrd_price = dec!("0.05");
        let usd_price = dec!("20.0");
        pyro_sale.use_manual_xrd_price(xrd_price, &mut self.env)?;                
        pyro_sale.set_do_check_for_same_transaction(false, &mut self.env)?;
        
        self.phs_bucket         = Some ( self.get_placeholder_bucket() );
        self.pyros_bucket       = Some ( self.get_pyro_bucket() );
        
        self.latest_usd_price   = usd_price; // self.get_latest_usd_price();

        Ok(())
    }


    pub fn buy_placeholders_check(&mut self, use_xrd:bool, amount_placeholders:u16, amount_token:Decimal, check_change:bool, expected_change:Decimal) -> Result<(), RuntimeError>
    {           
        // self.env.enable_costing_module();

        let mut pyro = self.pyro_sale.unwrap();

        let (a1, b1, _, _, _, _, _) = self.get_internal_state();

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
        let (a2, b2, _, _, _, _, _) = self.get_internal_state();

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

    fn mint_dummy_pyro_nft(&mut self, nft_id:u16) -> Bucket
    {
        let mut pyro_mint = self.pyro_minting.unwrap();

        let pyro_name = "Name ".to_owned() + &nft_id.to_string();
        let pyro_desc = "Description ".to_owned() + &nft_id.to_string();
        let pyro_filename = "Filename ".to_owned() + &nft_id.to_string();
        let key_image_hash = "Hash ".to_owned() + &nft_id.to_string();

        let nft_storage = "nft_storage ".to_owned() + &nft_id.to_string();        
        
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
        
        let pyro_nft = pyro_mint.mint_pyro_nft(nft_id, pyro_id, pyro_name, pyro_desc, pyro_filename, key_image_hash,nft_storage, pyro_traits, &mut self.env).unwrap();

        pyro_nft

    }
    
    fn mint_dummy_pyro_and_placeholder_nft(&mut self, nft_id: u16, put_placeholder_in_sale_contract:bool) -> Result<(), RuntimeError>
    {
        let mut pyro_sale = self.pyro_sale.unwrap();
        let mut pyro_mint = self.pyro_minting.unwrap();
        
        let pyro_nft = self.mint_dummy_pyro_nft(nft_id);
        
        // self.env.disable_costing_module();
        let placeholder_nft = pyro_mint.mint_placeholder_nft(&mut self.env).unwrap();

        let placeholder_bucket = self.get_placeholder_bucket();

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

            self.mint_dummy_pyro_and_placeholder_nft(nft_id, put_placeholder_in_sale_contract)?;
            nft_id+=1;
        };

        Ok(())
    }

    pub fn start_sale(&mut self) -> Result<(), RuntimeError>
    {
        let mut pyro = self.pyro_sale.unwrap();

        pyro.start_sale(&mut self.env)        
    }

    pub fn continue_sale(&mut self) -> Result<(), RuntimeError>
    {
        let mut pyro = self.pyro_sale.unwrap();

        pyro.continue_sale(&mut self.env)        
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
        self.mint_till_start_sale_or_without(amount_nfts, amount_team, true).unwrap();

        Ok(())
    }
    
    pub fn mint_till_start_sale_or_without(&mut self, amount_nfts:u16, amount_team:u16, start_sale:bool) -> Result<(), RuntimeError>
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

        if start_sale 
        {
            self.start_sale().unwrap();
        }        

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

        let (_, _, c1, _, e1, _, _) = self.get_internal_state();

        pyro.assign_placeholders_to_nfts( 20u16,&mut self.env).unwrap();

        let (_, _, c2, _, e2, _, _) = self.get_internal_state();

        
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

        pyro.set_do_check_for_same_transaction(do_check, &mut self.env).unwrap();

        Ok(())
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


    pub fn swap_placeholders(&mut self, amount: u16) -> Result<(), RuntimeError>
    {
        self.swap_placeholders_check(Decimal::from(amount), amount).unwrap();

        Ok(())
    }
    
    pub fn swap_placeholders_check(&mut self, amount_bucket: Decimal, amount: u16) -> Result<(), RuntimeError>
    {
        // let pyro = self.pyro_sale.unwrap();

        let (_, _, _, d1, _, _, _) = self.get_internal_state();

        // get amount_bucket phs from internal helper bucket for changing into pyro nfts
        let mut pyro = self.pyro_sale.unwrap();  
        let bucket = self.phs_bucket.as_mut().unwrap().take(amount_bucket, &mut self.env)?;
        
        let (pyros, phs) = pyro.swap_placeholders( bucket, amount, &mut self.env).unwrap();

        // check internal
        let (_, _, _, d2, _, _, _) = self.get_internal_state();
        
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

        let (a1, b1, c1, d1, e1, f1, g1) = self.get_internal_state();

        pyro.reserve_nfts_for_usd_sale(coupon_code, amount, &mut self.env).unwrap();


        // check internal state
        let (a2, b2, c2, d2, e2, f2, g2) = self.get_internal_state();

        assert_eq!(a1, a2);
        assert_eq!(b1, b2);
        assert_eq!(c1, c2);
        assert_eq!(d1, d2);
        assert_eq!(e1, e2);        
        assert_eq!(f1 + amount, f2); 
        assert_eq!(g1, g2);
        
        Ok(())
    }


    pub fn claim_nfts_for_usd_sale(&mut self, coupon_code:String, amount: u16, was_reserved_before:bool) -> Result<(), RuntimeError>
    {
        let mut pyro = self.pyro_sale.unwrap();

        let (a1, b1, _c1, d1, _e1, f1, g1) = self.get_internal_state();

        let pyro_nfts = pyro.claim_nfts_for_usd_sale(coupon_code, amount, 50, &mut self.env).unwrap();
        
        // check return
        assert_eq!(pyro_nfts.amount(&mut self.env)?, Decimal::from(amount));            

        // check internal state
        let (a2, b2, _c2, d2, _e2, f2, g2) = self.get_internal_state();

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
        self.claim_nfts_for_usd_sale("CP02".to_owned(), 7, true).unwrap();

        // get 9
        self.claim_nfts_for_usd_sale("CP03".to_owned(), 9, false).unwrap();

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

    pub fn complex_testcase(&mut self, step:u16) -> Result<(), RuntimeError>
    {        
        let mut pyro_mint = self.pyro_minting.unwrap();
        let mut pyro_sale = self.pyro_sale.unwrap();        
        
        if step==1 
        {
            let placeholder_nft = pyro_mint.mint_placeholder_nft(&mut self.env).unwrap();                                
            let empty_pyro_bucket = self.get_pyro_bucket();
            pyro_sale.add_nfts_for_sale(1, empty_pyro_bucket, placeholder_nft, &mut self.env).unwrap();
            return Ok(())
        }
        else if step==2
        {
            let placeholder_nft = pyro_mint.mint_placeholder_nft(&mut self.env).unwrap();                                
            let pyro_nfts_2 = self.mint_dummy_pyro_nft(1);
            pyro_nfts_2.put(self.mint_dummy_pyro_nft(2), &mut self.env).unwrap();
            pyro_sale.add_nfts_for_sale(1, pyro_nfts_2, placeholder_nft, &mut self.env).unwrap();
            return Ok(())
        }       
        else if step==3
        {
            let pyro_nft_1 = self.mint_dummy_pyro_nft(2);        
            let placeholder_nfts_2 = pyro_mint.mint_placeholder_nft(&mut self.env).unwrap();
            placeholder_nfts_2.put( pyro_mint.mint_placeholder_nft(&mut self.env).unwrap(), &mut self.env ).unwrap();
            pyro_sale.add_nfts_for_sale(1, pyro_nft_1, placeholder_nfts_2, &mut self.env).unwrap();            
            return Ok(())
        }
        else 
        {
            // mint without starting sale
            self.mint_till_start_sale_or_without(99, 10, false).unwrap();                        
        }

        if step==4
        {
            pyro_sale.pause_sale(&mut self.env).unwrap();
            return Ok(())
        }
        else if step == 5 
        {
            pyro_sale.continue_sale(&mut self.env).unwrap();
            return Ok(())
        }
        else if step == 6
        {
            self.buy_placeholders(true, 1, self.latest_usd_price).unwrap();
            return Ok(())
        }
        else if step == 7
        {
            self.assign_placeholders_to_nfts().unwrap();
            return Ok(())
        }
        else if step == 8
        {
            self.swap_placeholders(1).unwrap();
            return Ok(())
        }
        else if step == 9
        {
            self.reserve_nfts_for_usd_sale("CP001".to_owned(), 1).unwrap();
            return Ok(())
        }
        else if step == 10
        {
            self.claim_nfts_for_usd_sale("CP001".to_owned(), 1, false).unwrap();
            return Ok(())
        }
        else
        {
            // start sale
            self.start_sale().unwrap();            
        }

        if step == 11
        {
            self.start_sale().unwrap();
            return Ok(())
        }        
        else if step == 12
        {
            let pyro_nft = self.mint_dummy_pyro_nft(100);        
            let placeholder_nft = self.get_placeholder_bucket();

            pyro_sale.add_nfts_for_sale(100, pyro_nft, placeholder_nft, &mut self.env).unwrap();
            return Ok(())
        }
        else 
        {
            // pause sale
            self.pause_sale().unwrap();            
        }

        if step==13
        {
            pyro_sale.pause_sale(&mut self.env).unwrap(); // should fail since already paused
            return Ok(())
        }
        else if step == 14
        {
            pyro_sale.continue_sale(&mut self.env).unwrap(); // should succeed
            return Ok(())
        }
        else if step == 15
        {
            self.buy_placeholders(true, 1, self.latest_usd_price).unwrap(); // should fail since sale paused
            return Ok(())
        }
        else if step == 16
        {
            self.assign_placeholders_to_nfts().unwrap(); // should succeed
            return Ok(())
        }
        else if step == 17
        {
            self.assign_placeholders_to_nfts().unwrap();
            self.swap_placeholders(1).unwrap(); // should succeed
            return Ok(())
        }
        else if step == 18
        {
            self.reserve_nfts_for_usd_sale("CP001".to_owned(), 1).unwrap(); // should fail since sale paused
            return Ok(())
        }
        else if step == 19
        {
            self.claim_nfts_for_usd_sale("CP001".to_owned(), 1, false).unwrap(); // should fail since sale paused
            return Ok(())
        }
        else 
        {
            // continue sale
            self.continue_sale().unwrap();            
        }

        if step==20
        {
            self.continue_sale().unwrap();
            return Ok(())
        }
        else if step==21
        {
            // should all succeed
            self.buy_placeholders(true, 1, self.latest_usd_price).unwrap(); // should fail since sale paused                        
            self.assign_placeholders_to_nfts().unwrap();            
            self.swap_placeholders(1).unwrap(); 
            self.reserve_nfts_for_usd_sale("CP001".to_owned(), 1).unwrap(); // should fail since sale paused            
            self.claim_nfts_for_usd_sale("CP001".to_owned(), 1, true).unwrap(); // should fail since sale paused
            self.claim_nfts_for_usd_sale("CP002".to_owned(), 1, false).unwrap(); // should fail since sale paused            
            
            self.buy_placeholders(true, 2, 2*self.latest_usd_price).unwrap(); // should fail since sale paused                        
            
            // get xrd: should be 3 = 2+1 times latest_usd_price
            let xrd_bucket = pyro_sale.withdraw_xrd(&mut self.env).unwrap();    
            assert_eq!(xrd_bucket.amount(&mut self.env).unwrap(), 3*self.latest_usd_price);
            self.xrd_token.put(xrd_bucket, &mut self.env).unwrap();

            // collect remaining nfts in emergency situation: should be 99 - 3 = 96 left
            self.pause_sale().unwrap();
            let pyro_nfts = pyro_sale.collect_nfts_in_emergency_situation(dec!("96.0"), &mut self.env).unwrap();
            assert_eq!(pyro_nfts.amount(&mut self.env).unwrap(), dec!("96.0"));
            self.pyros_bucket.as_mut().unwrap().put(pyro_nfts, &mut self.env).unwrap();

            return Ok(())
        }
        else
        {
            let usd_price = dec!("4.0");
            pyro_sale.use_manual_xrd_price( 1 / usd_price, &mut self.env).unwrap();

            // set price of an NFT in USD to 20 USD
            let price_in_usd = dec!("20.0");
            pyro_sale.set_price(price_in_usd, price_in_usd, price_in_usd, 0, 0, &mut self.env).unwrap();
        
            self.buy_placeholders_check(true, 1, usd_price*price_in_usd, true, dec!("0.0")).unwrap();            
        }

        if step==22
        {
            return Ok(())
        }
        else if step==23
        {
            // let prev_usd_price = self.get_latest_usd_price();

            //self.env.enable_costing_module();
            // pyro_sale.use_runtime_usd_price( &mut self.env).unwrap();
            self.dummy_oracle.set_price( dec!("0.08"), &mut self.env ).unwrap();
            pyro_sale.use_oracle_xrd_price(&mut self.env).unwrap();

            // let current_usd_price = self.get_latest_usd_price();

            // assert_ne!(prev_usd_price, current_usd_price, "Price should have changed by using runtime_price");

            self.buy_placeholders_check(true, 1, dec!("20.0") / dec!("0.08"), true, dec!("0.0")).unwrap();            

            //self.env.disable_costing_module();
        }
        
        Ok(())

    }

    pub fn auth_testcase(&mut self, proof_address:ResourceAddress, action:Action) -> Result<(), RuntimeError>
    {        
        let mut pyro_mint = self.pyro_minting.unwrap();
        let mut pyro_sale = self.pyro_sale.unwrap();

        pyro_sale.use_manual_xrd_price(dec!("1"), &mut self.env).unwrap();

        
        // prepare state depending on action
        if action==Action::AddNftsForSale || action==Action::StartSale
        {
            self.mint_till_start_sale_or_without(99, 10, false).unwrap();                        
        }
        else 
        {
            self.mint_till_start_sale(100, 10).unwrap(); 

            self.buy_placeholders(true, 1, dec!("1.0")).unwrap();                       
        }

        if action==Action::ContinueSale || action==Action::CollectNftsInEmergency
        {
            self.pause_sale().unwrap();
        }
        
        // create proof and push it on auth zone
        let proof = ProofFactory::create_fungible_proof(proof_address, Decimal::ONE, CreationStrategy::Mock, &mut self.env).unwrap();        
        LocalAuthZone::push(proof, &mut self.env).unwrap();        
        
        // enable auth module
        self.env.enable_auth_module();

        // do according action
        match action {
            Action::AddNftsForSale => 
            {
                self.env.disable_auth_module(); // we don't want to check minting
                let placeholder_nft = pyro_mint.mint_placeholder_nft(&mut self.env).unwrap();                                                
                let pyro_nft = self.mint_dummy_pyro_nft(100);
                self.env.enable_auth_module(); // we don't want to check minting

                pyro_sale.add_nfts_for_sale(100, pyro_nft, placeholder_nft, &mut self.env).unwrap();        
            }, 
            Action::CollectNftsInEmergency =>
            {
                let pyro_nfts = pyro_sale.collect_nfts_in_emergency_situation(dec!("1.0"), &mut self.env).unwrap();
                self.pyros_bucket.as_mut().unwrap().put( pyro_nfts, &mut self.env).unwrap();
            }, 
            Action::ContinueSale =>
            {
                self.continue_sale().unwrap();
                
            },  
            Action::PauseSale =>
            {
                self.pause_sale().unwrap();
                
            }, 
            Action::StartSale =>
            {
                self.start_sale().unwrap();   
            }, 
            Action::UseManualUsdPrice =>
            {                
                pyro_sale.use_manual_xrd_price(dec!("1.0"), &mut self.env).unwrap();                                
            }, 
            Action::UseOracleUsdPrice =>
            {
                self.env.enable_costing_module();
                pyro_sale.use_oracle_xrd_price(&mut self.env).unwrap();
                self.env.disable_costing_module();
            }, 
            Action::WithdrawXRD =>
            {
                let xrd = pyro_sale.withdraw_xrd(&mut self.env).unwrap();
                self.xrd_token.put( xrd, &mut self.env).unwrap();
            }, 
        } 

        self.env.disable_auth_module();                        

        Ok(())
    }

    fn get_placeholder_bucket(&mut self) -> Bucket
    {
        /*let pyro_sale = self.pyro_sale.unwrap();
        let state = self.env
        .read_component_state::<PyroSaleState, _>(pyro_sale)
        .unwrap();    */

        let empty_it = std::iter::empty::<(NonFungibleLocalId,PyroNFT)>();

        BucketFactory::create_non_fungible_bucket(
            self.placeholders_address.unwrap(),
            empty_it, 
            Mock, 
            &mut self.env).unwrap()        
    }

    /*fn dummy(&mut self) -> Result<(), RuntimeError>
    {
        let pyro_sale = self.pyro_sale.unwrap();
        let _state = self.env
            .read_component_state::<PyroSaleState, _>(pyro_sale)
            .unwrap();        

        Ok(()) 
    }*/

    fn get_pyro_bucket(&mut self) -> Bucket
    {
        /*let pyro_sale = self.pyro_sale.unwrap();
        let state = self.env
        .read_component_state::<PyroSaleState, _>(pyro_sale)
        .unwrap();
    */

        let empty_it = std::iter::empty::<(NonFungibleLocalId,PyroNFT)>();

        BucketFactory::create_non_fungible_bucket(
            self.pyros_address.unwrap(), 
            empty_it, 
            Mock, 
            &mut self.env).unwrap()
    }

    /*fn get_latest_usd_price(&mut self) -> Decimal
    {
        let pyro_sale = self.pyro_sale.unwrap();
        /*;
        let state = self.env
        .read_component_state::<PyroSaleState, _>(pyro_sale)
        .unwrap();

        state.latest_usd_price*/
        
        pyro_sale.get_latest_usd_price(&mut self.env).unwrap()
    }*/

    fn get_internal_state(&mut self) -> (Decimal, u16, u16, Decimal, u16, u16, Decimal)
    {
        let pyro_sale = self.pyro_sale.unwrap();
        /*let state = self.env
        .read_component_state::<PyroSaleState, _>(pyro_sale)
        .unwrap();

        let a = state.placeholder_nfts_vault.amount(&mut self.env).unwrap();

        let b = state.placeholders_sold_or_used_up_total;

        let c = state.mapping_placeholder_nft.len() as u16;

        let d = state.pyro_nfts_vault.amount(&mut self.env).unwrap();

        let e = state.nft_ids.len() as u16;

        let f = state.sold_usd_just_reserved;            

        let g = state.collected_xrd_vault.amount(&mut self.env).unwrap();

        (a, b, c, d, e, f, g)
        */

        pyro_sale.get_internal_state(&mut self.env).unwrap()
    }

    pub fn use_oracle_xrd_price(&mut self, price:Decimal)
    {
        self.pyro_sale.unwrap().use_oracle_xrd_price(&mut self.env).unwrap();
        
        self.dummy_oracle.set_price(price, &mut self.env).unwrap();

        let new_price = self.dummy_oracle.get_price(&mut self.env).unwrap();

        assert_eq!(price, new_price.price);        

        self.latest_usd_price = 1 / price;
    }      

    pub fn use_new_oracle(&mut self) 
    {
        let dummy_oracle_package_address = Package::compile_and_publish(
            "./dummy_oracle",
            &mut self.env
        ).unwrap();
        
        let dummy_oracle = DummyOracle::instantiate(dec!("2"), dummy_oracle_package_address, &mut self.env).unwrap();

        let dummy_oracle_address = dummy_oracle.address(&mut self.env).unwrap();

        self.dummy_oracle = dummy_oracle;
        self.dummy_oracle_address = dummy_oracle_address;

        self.pyro_sale.unwrap().set_oracle_component_address(dummy_oracle_address, &mut self.env).unwrap();
    }
}