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