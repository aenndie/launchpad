//use pretty_assertions::assert_eq;
use pyro19::test_bindings::*;
use dummy_account::test_bindings::*;
use scrypto::*;
use scrypto_test::prelude::*;

pub struct MigrationHelper {
    pub env: TestEnvironment,
    pub dapp_definition: ComponentAddress,
    pub package_address: PackageAddress,
    pub xrd_token: Bucket,
    pub non_xrd_token: Bucket,        
    pub xrd_address: ResourceAddress,
    pub non_xrd_address: ResourceAddress,        
    pub pyro19: Option<Pyro>,
}


impl MigrationHelper {

    pub fn new() -> Result<Self, RuntimeError> {

        let mut env = TestEnvironment::new();

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
        })

    
    }


    pub fn instantiate(
        &mut self,      
        max_collection_size:u16
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

        //let mut hello = Hello::instantiate_hello(package_address, &mut env)?;    

        let (pyro19, _, _, _)  = Pyro::instantiate_pyro(
        dec!("20"), 
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
        10u16,
        25u16,
        self.dapp_definition,
        50u16,
        20u16, 
        self.package_address,
        &mut self.env)?;

        self.pyro19 = Some (pyro19);

        Ok(())
    }



}