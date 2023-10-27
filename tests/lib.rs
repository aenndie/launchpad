mod helper;
use scrypto_test::prelude::*;
use helper::*;

/*
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

#[test]
fn tc_1_3_1_buy_max_amount_case_max() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
    
    helper.mint_till_start_sale(100, 10);
        
    // max amount per buy is 50    
    let amount_token = 50*price * helper.latest_usd_price;                
    helper.buy_placeholders(true,  50u16, amount_token).unwrap();
}

#[test]
#[should_panic]
fn tc_1_3_2_buy_max_amount_case_max_plus_1() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
    
    helper.mint_till_start_sale(100, 10);

    // max amount per buy is 50    
    let amount_token = 51*price * helper.latest_usd_price;                
    helper.buy_placeholders(true,  51u16, amount_token).unwrap();
}

#[test]
#[should_panic]
fn tc_1_4_1_buy_amount_positive_case_zero() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
    
    helper.mint_till_start_sale(100, 10);

    // max amount per buy is 50    
    let amount_token = price * helper.latest_usd_price;                
    helper.buy_placeholders(true,  0u16, amount_token).unwrap();
}

// tc_1_4_2 is not necessary since parameter amount:u16 is unsigned

#[test]
fn tc_1_4_3_buy_amount_positive_case_one() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
    
    helper.mint_till_start_sale(100, 10);

    // max amount per buy is 50    
    let amount_token = price * helper.latest_usd_price;                
    helper.buy_placeholders(true,  1u16, amount_token).unwrap();
}

#[test]
fn tc_1_5_1_payment_enough_case_exact() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
    
    helper.mint_till_start_sale(100, 10);

    // max amount per buy is 50    
    let amount_token = 5*price * helper.latest_usd_price;                
    helper.buy_placeholders(true,  5u16, amount_token).unwrap();
}

#[test]
#[should_panic]
fn tc_1_5_2_payment_enough_case_too_little() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
    
    helper.mint_till_start_sale(100, 10);

    // max amount per buy is 50    
    let amount_token = 5*price * helper.latest_usd_price - dec!("0.0001");                
    helper.buy_placeholders(true,  5u16, amount_token).unwrap();
}

// tc 1_6 was tested before since we check the amount of nfts we get back already within helper.buy_placeholders_check via
// assert_eq!( phs.amount(&mut self.env)?, Decimal::from(amount_placeholders) );

#[test]
fn tc_1_7_1_change_is_correct_case_zero() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
    
    helper.mint_till_start_sale(100, 10);

    // max amount per buy is 50    
    let amount_token = 5*price * helper.latest_usd_price;                
    helper.buy_placeholders_check(true,  5u16, amount_token, true, dec!("0.0") ).unwrap();
}

#[test]
fn tc_1_7_2_change_is_correct_case_pos() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
    
    helper.mint_till_start_sale(100, 10);

    // max amount per buy is 50    
    let amount_token = 5*price * helper.latest_usd_price + dec!("1.2345");                
    helper.buy_placeholders_check(true,  5u16, amount_token, true, dec!("1.2345") ).unwrap();
}

#[test]
#[should_panic]
fn tc_1_7_3_change_is_correct_case_doublecheck() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
    
    helper.mint_till_start_sale(100, 10);

    // max amount per buy is 50    
    let amount_token = 5*price * helper.latest_usd_price + dec!("1.2345");                
    helper.buy_placeholders_check(true,  5u16, amount_token, true, dec!("1.2346") ).unwrap();
}

#[test]
fn tc_1_8_1_availability_case_max() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
    
    helper.mint_till_start_sale(100, 10);

    // max amount available is 65 = 100 - 10 (team) - 25 (buffer) = 50 + 15
    let amount_token = 50*price * helper.latest_usd_price ;                
    helper.buy_placeholders(true,  50u16, amount_token).unwrap();

    let amount_token = 15*price * helper.latest_usd_price ;                
    helper.buy_placeholders(true,  15u16, amount_token).unwrap();
}

#[test]
#[should_panic]
fn tc_1_8_2_availability_case_max_plus_1() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
    
    helper.mint_till_start_sale(100, 10);

    // max amount available is 65 = 100 - 10 (team) - 25 (buffer) = 50 + 15
    let amount_token = 50*price * helper.latest_usd_price ;                
    helper.buy_placeholders(true,  50u16, amount_token).unwrap();

    let amount_token = 16*price * helper.latest_usd_price ;                
    helper.buy_placeholders(true,  16u16, amount_token).unwrap();
}
 
#[test]
fn tc_1_9_1_price_correct_case_range1() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
    
    helper.mint_till_start_sale(100, 10);

    // set price stages here: 15-20-25 USD for 10-20-100
    helper.set_price(dec!("15.0"), dec!("20.0"), dec!("25.0"),  10u16, 20u16).unwrap();

    // buy 5 placeholder: should cost 5x15=75
    let amount_token = 75* helper.latest_usd_price;
    helper.buy_placeholders_check(true,  5u16, amount_token, true, dec!("0.0")).unwrap();    
}

#[test]
fn tc_1_9_2_price_correct_case_range1_and_range_2() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
    
    helper.mint_till_start_sale(100, 10);

    // set price stages here: 15-20-25 USD for 10-20-100
    helper.set_price(dec!("15.0"), dec!("20.0"), dec!("25.0"),  10u16, 20u16).unwrap();

    // buy 12 placeholder: should cost 10x15 + 2x20 = 150 + 40 = 190
    let amount_token = 190 * helper.latest_usd_price;
    helper.buy_placeholders_check(true,  12u16, amount_token, true, dec!("0.0")).unwrap();    
}

#[test]
fn tc_1_9_3_price_correct_case_range1_and_range2_and_range3() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
    
    helper.mint_till_start_sale(100, 10);

    // set price stages here: 15-20-25 USD for 10-20-100
    helper.set_price(dec!("15.0"), dec!("20.0"), dec!("25.0"),  10u16, 20u16).unwrap();

    // buy 23 placeholder: should cost 10x15 + 10x20 + 3*25 = 150 + 200 + 75 = 425
    let amount_token = 425 * helper.latest_usd_price;
    helper.buy_placeholders_check(true,  23u16, amount_token, true, dec!("0.0")).unwrap();    
}

#[test]
fn tc_1_9_4_price_correct_case_range2() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
    
    helper.mint_till_start_sale(100, 10);

    // set price stages here: 15-20-25 USD for 10-20-100
    helper.set_price(dec!("15.0"), dec!("20.0"), dec!("25.0"),  10u16, 20u16).unwrap();

    // at first 12 placeholders -> 190
    let amount_token = 190* helper.latest_usd_price;
    helper.buy_placeholders(true,  12u16, amount_token).unwrap();
    
    // buy 5 placeholder: should cost 5x20=100
    let amount_token = 100* helper.latest_usd_price;
    helper.buy_placeholders_check(true,  5u16, amount_token, true, dec!("0.0")).unwrap();    
}

#[test]
fn tc_1_9_5_price_correct_case_range2_and_range3() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
    
    helper.mint_till_start_sale(100, 10);

    // set price stages here: 15-20-25 USD for 10-20-100
    helper.set_price(dec!("15.0"), dec!("20.0"), dec!("25.0"),  10u16, 20u16).unwrap();

    // at first 12 placeholders -> 10*15 + 2*20 = 190
    let amount_token = 190* helper.latest_usd_price;
    helper.buy_placeholders(true,  12u16, amount_token).unwrap();
    
    // buy 10 placeholder: should cost 8x20 + 2x25 = 160 + 50 = 210
    let amount_token = 210* helper.latest_usd_price;
    helper.buy_placeholders_check(true,  10u16, amount_token, true, dec!("0.0")).unwrap();    
}

#[test]
fn tc_1_9_6_price_correct_case_range3() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
    
    helper.mint_till_start_sale(100, 10);

    // set price stages here: 15-20-25 USD for 10-20-100
    helper.set_price(dec!("15.0"), dec!("20.0"), dec!("25.0"),  10u16, 20u16).unwrap();

    // at first 20 placeholders -> 10*15 + 10*20 = 150 + 200 = 350
    let amount_token = 350 * helper.latest_usd_price;
    helper.buy_placeholders(true,  20u16, amount_token).unwrap();
    
    // buy 45 placeholder: should cost 45*25= 1.125
    let amount_token = 1125 * helper.latest_usd_price;
    helper.buy_placeholders_check(true,  45u16, amount_token, true, dec!("0.0")).unwrap();    
}


#[test]
#[should_panic]
fn tc_2_1_1_set_price_case_no_proof() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();

    helper.env.enable_auth_module();     

    helper.set_price(dec!("15"), dec!("20"), dec!("25"), 10, 20 ).unwrap();        
}


#[test]
fn tc_2_1_2_set_price_case_auth_disabled() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();

    helper.env.disable_auth_module();     

    helper.set_price(dec!("15"), dec!("20"), dec!("25"), 10, 20 ).unwrap();        
}

//todo 2_1_3 how to create proof?

// 2_2_1, 2_3_1 and 2_4_1 ->

#[test]
#[should_panic]
fn tc_2_2_2_price_zero_case_price1_zero() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();

    helper.env.disable_auth_module();     

    helper.set_price(dec!("0"), dec!("20"), dec!("25"), 10, 20 ).unwrap();        
}

#[test]
#[should_panic]
fn tc_2_3_2_price_zero_case_price2_zero() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    

    helper.set_price(dec!("15"), dec!("0"), dec!("25"), 10, 20 ).unwrap();        
}

#[test]
#[should_panic]
fn tc_2_4_2_price_zero_case_price3_zero() {

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();

    helper.set_price(dec!("15"), dec!("20"), dec!("0"), 10, 20 ).unwrap();        
}

#[test]
#[should_panic]
fn tc_2_2_3_price_zero_case_price1_neg() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();

    helper.env.disable_auth_module();     

    helper.set_price(dec!("-1"), dec!("20"), dec!("25"), 10, 20 ).unwrap();        
}

#[test]
#[should_panic]
fn tc_2_3_3_price_zero_case_price2_neg() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    

    helper.set_price(dec!("15"), dec!("-1"), dec!("25"), 10, 20 ).unwrap();        
}

#[test]
#[should_panic]
fn tc_2_4_3_price_zero_case_price3_neg() {

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();

    helper.set_price(dec!("15"), dec!("20"), dec!("-1"), 10, 20 ).unwrap();        
}

#[test]
// is allowed
fn tc_2_5_2_price_stage_1_case_zero() {

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();

    helper.set_price(dec!("15"), dec!("20"), dec!("25"), 0, 20).unwrap();        
}

#[test]
fn tc_2_6_2_price_stage_2_case_eq_stage1() {

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();

    helper.set_price(dec!("15"), dec!("20"), dec!("25"), 10, 10).unwrap();        
}

#[test]
#[should_panic]
fn tc_2_6_3_price_stage_2_case_lt_stage1() {

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();

    helper.set_price(dec!("15"), dec!("20"), dec!("25"), 10, 9).unwrap();        
}

#[test]
fn tc_2_6_4_price_stage_2_case_eq_coll_size() {

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();

    helper.set_price(dec!("15"), dec!("20"), dec!("25"), 10, 100).unwrap();
}

#[test]
#[should_panic]
fn tc_2_6_5_price_stage_2_case_gr_coll_size() {

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();

    helper.set_price(dec!("15"), dec!("20"), dec!("25"), 10, 101).unwrap();
}



 #[test]
 #[should_panic]
fn tc_3_1_1_assign_placeholder_case_no_proof() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
    
    helper.mint_till_start_sale(100, 10);
    
    // buy 5 phs
    let amount_token = 5 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  5, amount_token).unwrap();
    
    // assign phs
    helper.env.enable_auth_module();
    
    helper.assign_placeholders_to_nfts().unwrap();
    
    helper.env.disable_auth_module();
}

#[test]
fn tc_3_1_2_assign_placeholder_case_auth_mod_disabled() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
    
    helper.mint_till_start_sale(100, 10);
    
    // buy 5 phs
    let amount_token = 5 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  5, amount_token).unwrap();
    
    // assign phs        
    helper.assign_placeholders_to_nfts().unwrap();    
}


#[test]
#[should_panic]
fn tc_3_2_1_same_transaction_case_buy_ph() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
    
    helper.mint_till_start_sale(100, 10);
    
    // buy 5 phs
    let amount_token = 5 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  5, amount_token).unwrap();
    
    // assign phs        
    helper.set_do_check_for_same_transaction(true);
    helper.assign_placeholders_to_nfts().unwrap();    
    helper.set_do_check_for_same_transaction(false);
}

#[test]
#[should_panic]
fn tc_3_2_2_same_transaction_case_team() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();    
    
    helper.set_do_check_for_same_transaction(true);    
    
    // get_phs_for_team is called within this method
    helper.mint_till_start_sale(100, 10);

    helper.assign_placeholders_to_nfts().unwrap();    
    
    helper.set_do_check_for_same_transaction(false);
}

#[test]
#[should_panic]
fn tc_3_3_0_amount_mapped_case_doublecheck() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();            
    
    // get_phs_for_team is called within this method
    helper.mint_till_start_sale(100, 10);

    // buy 5 phs
    let amount_token = 5 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  5, amount_token).unwrap();

    // expect 5 phs to be assignd: 10 team pyros were assigned while starting sale
    // so 4 should fail
    helper.assign_placeholders_to_nfts_check(true, 4).unwrap();            
}


// max amount for mapping at once is set to 20

#[test]
fn tc_3_3_1_amount_mapped_case_lt_max() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();            
    
    // get_phs_for_team is called within this method
    helper.mint_till_start_sale(100, 10);

    // buy 5 phs
    let amount_token = 19 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  19, amount_token).unwrap();

    // expect 19 phs to be assignd: 10 team pyros were assigned while starting sale
    helper.assign_placeholders_to_nfts_check(true, 19).unwrap();            
}

#[test]
fn tc_3_3_1_amount_mapped_case_eq_max() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();            
    
    // get_phs_for_team is called within this method
    helper.mint_till_start_sale(100, 10);

    // buy 20 phs
    let amount_token = 20 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  20, amount_token).unwrap();

    // expect 20 phs to be assignd: 10 team pyros were assigned while starting sale
    helper.assign_placeholders_to_nfts_check(true, 20).unwrap();            
}

#[test]
fn tc_3_3_1_amount_mapped_case_gt_max() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();            
    
    // get_phs_for_team is called within this method
    helper.mint_till_start_sale(100, 10);

    // buy 21 phs
    let amount_token = 21 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  21, amount_token).unwrap();

    // expect 20 phs to be assignd: 10 team pyros were assigned while starting sale
    helper.assign_placeholders_to_nfts_check(true, 20).unwrap();            
}

#[test]
fn check_if_storing_phs_works() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();            
    
    // get_phs_for_team is called within this method
    helper.mint_till_start_sale(100, 10);

    helper.expect_phs_in_bucket(dec!("10")).unwrap();

    // buy 21 phs
    let amount_token = 21 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  21, amount_token).unwrap();

    helper.expect_phs_in_bucket(dec!("31")).unwrap();
}

#[test]
#[should_panic]
fn check_if_storing_phs_double_check1() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();            
    
    // get_phs_for_team is called within this method
    helper.mint_till_start_sale(100, 10);

    helper.expect_phs_in_bucket(dec!("11")).unwrap(); // should fail

    // buy 21 phs
    let amount_token = 21 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  21, amount_token).unwrap();

    helper.expect_phs_in_bucket(dec!("31")).unwrap();
}

#[test]
#[should_panic]
fn check_if_storing_phs_double_check2() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();            
    
    // get_phs_for_team is called within this method
    helper.mint_till_start_sale(100, 10);

    helper.expect_phs_in_bucket(dec!("10")).unwrap();

    // buy 21 phs
    let amount_token = 21 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  21, amount_token).unwrap();

    helper.expect_phs_in_bucket(dec!("32")).unwrap(); // should fail
}

#[test]
#[should_panic]
fn tc_4_1_1_change_phs_not_all_phs_mapped_case_not_assigned_at_all() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();            
        
    helper.mint_till_start_sale(100, 10);    

    // buy 10 phs
    let amount_token = 10 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  10, amount_token).unwrap();

    helper.change_placeholders_into_nfts(1).unwrap(); // should fail since assign was not called in between
}

#[test]
#[should_panic]
fn tc_4_1_2_change_phs_not_all_phs_mapped_case_only_once_assigned_but_two_needed() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();            
        
    helper.mint_till_start_sale(100, 10);    

    // buy 60 phs
    let amount_token = 30 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  30, amount_token).unwrap();
    helper.buy_placeholders(true,  30, amount_token).unwrap();
    
    // assign once
    helper.assign_placeholders_to_nfts().unwrap();    

    // change 1
    helper.change_placeholders_into_nfts(1).unwrap(); // should fail since previous assign could not assign all
}

#[test]
fn tc_4_1_3_change_phs_all_phs_mapped_case_call_twice() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();            
        
    helper.mint_till_start_sale(100, 10);    

    // buy 60 phs
    let amount_token = 30 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  30, amount_token).unwrap();
    helper.buy_placeholders(true,  30, amount_token).unwrap();
    
    // assign phs: 3x needed since we only assign 20 at once for testing
    helper.assign_placeholders_to_nfts().unwrap();
    helper.assign_placeholders_to_nfts().unwrap();
    helper.assign_placeholders_to_nfts().unwrap();

    // change 1
    helper.change_placeholders_into_nfts(1).unwrap(); // should succeed now
}


#[test]
//#[should_panic]
fn tc_4_2_1_change_phs_case_amount_zero() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();            
        
    helper.mint_till_start_sale(100, 10);    

    // buy 30 phs
    let amount_token = 10 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  10, amount_token).unwrap();    
    
    // assign once
    helper.assign_placeholders_to_nfts().unwrap();    

    // change 0
    helper.change_placeholders_into_nfts(0).unwrap(); // should fail since 0 is not allowed
}


#[test]
fn tc_4_3_1_change_phs_max_amount_case_eq_max() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();            
        
    helper.mint_till_start_sale(100, 10);    

    // buy 2x30 phs
    let amount_token = 30 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  30, amount_token).unwrap();    
    helper.buy_placeholders(true,  30, amount_token).unwrap();    
    
    // assign
    helper.assign_placeholders_to_nfts().unwrap();    
    helper.assign_placeholders_to_nfts().unwrap();    
    helper.assign_placeholders_to_nfts().unwrap();    

    // change 0
    helper.change_placeholders_into_nfts(50).unwrap(); // should succeed
}


#[test]
#[should_panic]
fn tc_4_3_2_change_phs_max_amount_case_eq_max_plus1() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();            
        
    helper.mint_till_start_sale(100, 10);    

    // buy 2x30 phs
    let amount_token = 30 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  30, amount_token).unwrap();    
    helper.buy_placeholders(true,  30, amount_token).unwrap();    
    
    // assign
    helper.assign_placeholders_to_nfts().unwrap();    
    helper.assign_placeholders_to_nfts().unwrap();    
    helper.assign_placeholders_to_nfts().unwrap();    

    // change 0
    helper.change_placeholders_into_nfts(51).unwrap(); // should fail 
}

*/

#[test]
fn tc_4_4_2_change_phs_case_bucket_larger_than_needed() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();            
        
    helper.mint_till_start_sale(100, 10);    

    // buy 10 phs
    let amount_token = 10 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  10, amount_token).unwrap();        
    
    // assign
    helper.assign_placeholders_to_nfts().unwrap();        

    // change 0
    helper.expect_phs_in_bucket(dec!("20")).unwrap();
    helper.change_placeholders_into_nfts_check(dec!("10"), 5).unwrap(); // should succeed and we get 5 phs back
    helper.expect_phs_in_bucket(dec!("15")).unwrap(); // from team and 5 back from this call
    helper.expect_pyros_in_bucket(dec!("5")).unwrap();
}

#[test]
#[should_panic]
fn tc_4_4_3_change_phs_case_bucket_smaller_than_needed() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size, team_amount, price).unwrap();            
        
    helper.mint_till_start_sale(100, 10);    

    // buy 10 phs
    let amount_token = 10 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  10, amount_token).unwrap();        
    
    // assign
    helper.assign_placeholders_to_nfts().unwrap();        

    // change 0    
    helper.change_placeholders_into_nfts_check(dec!("5"), 10).unwrap(); // should fail
}