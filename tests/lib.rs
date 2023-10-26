/*#[cfg(test)]
mod pyro {
    use scrypto::*;
    //use scrypto_test::prelude::*;
    use super::*;
    use helper::*;

    #[test]
    fn test_instantiate_new_amount_equal_old_supply() {
        let mut helper = MigrationHelper::new().unwrap();        
        helper.instantiate(100u16).unwrap();
    }    
}*/


mod helper;
use scrypto_test::prelude::*;
use helper::*;


#[test]
fn tc_0_0_just_instantiate() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
}

#[test]
#[should_panic]
fn tc_1_1_1_sale_must_be_started_case_is_not_started() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    

    helper.mint_dummy_nfts(100);
    helper.set_status_minting_finished().unwrap();    
    helper.get_placeholders_for_team(10).unwrap();                

    let amount_token = price * helper.latest_usd_price;                
    helper.buy_placeholders(true,  1u16, amount_token).unwrap();
}

#[test]
fn tc_1_1_2_sale_must_be_started_case_is_started() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    

    helper.mint_dummy_nfts(100);
    helper.set_status_minting_finished().unwrap();    
    helper.get_placeholders_for_team(10).unwrap();                
    helper.start_sale().unwrap();

    let amount_token = price * helper.latest_usd_price;                
    helper.buy_placeholders(true,  1u16, amount_token).unwrap();
}

#[test]
#[should_panic]
fn tc_1_1_3_sale_must_be_started_case_is_paused() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    

    helper.mint_dummy_nfts(100);
    helper.set_status_minting_finished().unwrap();    
    helper.get_placeholders_for_team(10).unwrap();                
    helper.start_sale().unwrap();
    helper.pause_sale().unwrap();

    let amount_token = price * helper.latest_usd_price;                
    helper.buy_placeholders(true,  1u16, amount_token).unwrap();
}


#[test]
#[should_panic]
fn tc_1_2_1_payment_must_be_xrd_case_neq_xrd() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
    
    helper.mint_till_start_sale(100, 10);

    let amount_token = price * helper.latest_usd_price;                
    helper.buy_placeholders(false,  1u16, amount_token).unwrap();

}


#[test]
fn tc_1_2_2_payment_must_be_xrd_case_eq_xrd() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
    
    helper.mint_till_start_sale(100, 10);
        
    let amount_token = price * helper.latest_usd_price;                
    helper.buy_placeholders(true,  1u16, amount_token).unwrap();
}
