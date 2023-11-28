mod helper;
use scrypto_test::prelude::*;
use helper::*;

const POS_DIFF_1:Decimal = dec!("1");
const POS_DIFF_ATO:Decimal = dec!("0.000000000000000001"); // 10^-18


#[test]
fn tc_0_0_just_instantiate() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();   // should succeed 
}

#[test]
#[should_panic]
fn tc_1_1_1_sale_must_be_started_case_is_not_started() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_dummy_nfts(collection_size, team_amount).unwrap();    

    let amount_token = price * helper.latest_usd_price;                
    helper.buy_placeholders(true,  1u16, amount_token).unwrap(); // should fail since sale not started
}

#[test]
fn tc_1_1_2_sale_must_be_started_case_is_started() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    

    helper.mint_dummy_nfts(collection_size, team_amount).unwrap();    

    helper.start_sale().unwrap();

    let amount_token = price * helper.latest_usd_price;                
    helper.buy_placeholders(true,  1u16, amount_token).unwrap(); // should succeed now
}

#[test]
#[should_panic]
fn tc_1_1_3_sale_must_be_started_case_is_paused() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    

    helper.mint_dummy_nfts(collection_size, team_amount).unwrap();    

    helper.start_sale().unwrap();
    helper.pause_sale().unwrap();

    let amount_token = price * helper.latest_usd_price;                
    helper.buy_placeholders(true,  1u16, amount_token).unwrap(); // should fail since sale paused
}

#[test]
#[should_panic]
fn tc_1_2_1_payment_must_be_xrd_case_neq_xrd() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

    let amount_token = price * helper.latest_usd_price;                
    helper.buy_placeholders(false,  1u16, amount_token).unwrap(); // should fail

}


#[test]
fn tc_1_2_2_payment_must_be_xrd_case_eq_xrd() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();
        
    let amount_token = price * helper.latest_usd_price;                
    helper.buy_placeholders(true,  1u16, amount_token).unwrap(); // should succeed now since we pay with XRD
}

#[test]
fn tc_1_3_1_buy_max_amount_case_max() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();
        
    // max amount per buy is 50    
    let amount_token = 50*price * helper.latest_usd_price;                
    helper.buy_placeholders(true,  50u16, amount_token).unwrap(); // should succeed 
}

#[test]
#[should_panic]
fn tc_1_3_2_buy_max_amount_case_max_plus_1() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // max amount per buy is 50    
    let amount_token = 51*price * helper.latest_usd_price;                
    helper.buy_placeholders(true,  51u16, amount_token).unwrap(); // should fail
}

#[test]
#[should_panic]
fn tc_1_4_1_buy_amount_positive_case_zero() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // max amount per buy is 50    
    let amount_token = price * helper.latest_usd_price;                
    helper.buy_placeholders(true,  0u16, amount_token).unwrap(); // should fail
}

// tc_1_4_2 is not necessary since parameter amount:u16 is unsigned

#[test]
fn tc_1_4_3_buy_amount_positive_case_one() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // max amount per buy is 50    
    let amount_token = price * helper.latest_usd_price;                
    helper.buy_placeholders(true,  1u16, amount_token).unwrap(); // should succeed
}

#[test]
fn tc_1_5_1_payment_enough_case_exact() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // max amount per buy is 50    
    let amount_token = 5*price * helper.latest_usd_price;                
    helper.buy_placeholders(true,  5u16, amount_token).unwrap();
}


#[test]
#[should_panic]
fn tc_1_5_2_payment_enough_case_too_little_1() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // max amount per buy is 50    
    let amount_token = 5*price * helper.latest_usd_price - POS_DIFF_1;                
    helper.buy_placeholders(true,  5u16, amount_token).unwrap();
}

#[test]
#[should_panic]
fn tc_1_5_3_payment_enough_case_too_little_ato() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // max amount per buy is 50    
    let amount_token = 5*price * helper.latest_usd_price - POS_DIFF_ATO;                
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
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

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
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // max amount per buy is 50    
    let amount_token = 5*price * helper.latest_usd_price + dec!("1.2345");                
    helper.buy_placeholders_check(true,  5u16, amount_token, true, dec!("1.2345") ).unwrap();
}

#[test]
fn tc_1_7_3_change_is_correct_case_pos_ato() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // max amount per buy is 50    
    let amount_token = 5*price * helper.latest_usd_price + POS_DIFF_ATO;                
    helper.buy_placeholders_check(true,  5u16, amount_token, true, POS_DIFF_ATO ).unwrap();
}

#[test]
#[should_panic]
fn tc_1_7_4_change_is_correct_case_doublecheck() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

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
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // max amount available is 100 - 10 (team) - 0 (buffer) = 90
    let amount_token = 50*price * helper.latest_usd_price ;                
    helper.buy_placeholders(true,  50u16, amount_token).unwrap();

    let amount_token = 40*price * helper.latest_usd_price ;                
    helper.buy_placeholders(true,  40u16, amount_token).unwrap(); //should succeed
}

#[test]
#[should_panic]
fn tc_1_8_2_availability_case_max_plus_1() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // max amount available is 90 = 100 - 10 (team) - 0 (buffer) = 90
    let amount_token = 50*price * helper.latest_usd_price ;                
    helper.buy_placeholders(true,  50u16, amount_token).unwrap();

    let amount_token = 41*price * helper.latest_usd_price ;                
    helper.buy_placeholders(true,  41u16, amount_token).unwrap(); // should fail since 91 > 90
}
 
#[test]
fn tc_1_9_1_price_correct_case_range1() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

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
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

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
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

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
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

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
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

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
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

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
fn tc_1_10_1_price_correct_oracle_case_range1() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // set price stages here: 15-20-25 USD for 10-20-100
    helper.set_price(dec!("15.0"), dec!("20.0"), dec!("25.0"),  10u16, 20u16).unwrap();

    let xrd_price = dec!("0.08");
    helper.use_oracle_xrd_price(xrd_price);

    // buy 5 placeholder: should cost 5x15=75
    let amount_token = 75* helper.latest_usd_price;
    helper.buy_placeholders_check(true,  5u16, amount_token, true, dec!("0.0")).unwrap();    
}

#[test]
fn tc_1_10_1_1_price_correct_oracle_case_range1_new_oracle() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // set price stages here: 15-20-25 USD for 10-20-100
    helper.set_price(dec!("15.0"), dec!("20.0"), dec!("25.0"),  10u16, 20u16).unwrap();

    let xrd_price = dec!("0.08");
    helper.use_oracle_xrd_price(xrd_price);

    helper.use_new_oracle();
    let xrd_price = dec!("0.05");
    helper.use_oracle_xrd_price(xrd_price);

    // buy 5 placeholder: should cost 5x15=75
    let amount_token = 75 / dec!("0.05");
    helper.buy_placeholders_check(true,  5u16, amount_token, true, dec!("0.0")).unwrap();    
}

#[test]
fn tc_1_10_2_price_correct_oracle_case_range1_and_range_2() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // set price stages here: 15-20-25 USD for 10-20-100
    helper.set_price(dec!("15.0"), dec!("20.0"), dec!("25.0"),  10u16, 20u16).unwrap();

    let xrd_price = dec!("0.08");
    helper.use_oracle_xrd_price(xrd_price);

    // buy 12 placeholder: should cost 10x15 + 2x20 = 150 + 40 = 190
    let amount_token = 190  / dec!("0.08");
    helper.buy_placeholders_check(true,  12u16, amount_token, true, dec!("0.0")).unwrap();    
}

#[test]
fn tc_1_10_3_price_correct_oracle_case_range1_and_range2_and_range3() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // set price stages here: 15-20-25 USD for 10-20-100
    helper.set_price(dec!("15.0"), dec!("20.0"), dec!("25.0"),  10u16, 20u16).unwrap();

    let xrd_price = dec!("0.08");
    helper.use_oracle_xrd_price(xrd_price);

    // buy 23 placeholder: should cost 10x15 + 10x20 + 3*25 = 150 + 200 + 75 = 425
    let amount_token = 425 / dec!("0.08");
    helper.buy_placeholders_check(true,  23u16, amount_token, true, dec!("0.0")).unwrap();    
}

#[test]
fn tc_1_10_4_price_correct_oracle_case_range2() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // set price stages here: 15-20-25 USD for 10-20-100
    helper.set_price(dec!("15.0"), dec!("20.0"), dec!("25.0"),  10u16, 20u16).unwrap();

    let xrd_price = dec!("0.08");
    helper.use_oracle_xrd_price(xrd_price);

    // at first 12 placeholders -> 190
    let amount_token = 190 / dec!("0.08");
    helper.buy_placeholders(true,  12u16, amount_token).unwrap();
    
    // buy 5 placeholder: should cost 5x20=100
    let amount_token = 100 / dec!("0.08");
    helper.buy_placeholders_check(true,  5u16, amount_token, true, dec!("0.0")).unwrap();    
}

#[test]
fn tc_1_10_5_price_correct_oracle_case_range2_and_range3() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // set price stages here: 15-20-25 USD for 10-20-100
    helper.set_price(dec!("15.0"), dec!("20.0"), dec!("25.0"),  10u16, 20u16).unwrap();

    let xrd_price = dec!("0.08");
    helper.use_oracle_xrd_price(xrd_price);

    // at first 12 placeholders -> 10*15 + 2*20 = 190
    let amount_token = 190 / dec!("0.08");
    helper.buy_placeholders(true,  12u16, amount_token).unwrap();
    
    // buy 10 placeholder: should cost 8x20 + 2x25 = 160 + 50 = 210
    let amount_token = 210 / dec!("0.08");
    helper.buy_placeholders_check(true,  10u16, amount_token, true, dec!("0.0")).unwrap();    
}

#[test]
fn tc_1_10_6_price_correct_oracle_case_range3() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // set price stages here: 15-20-25 USD for 10-20-100
    helper.set_price(dec!("15.0"), dec!("20.0"), dec!("25.0"),  10u16, 20u16).unwrap();
    
    let xrd_price = dec!("0.08");
    helper.use_oracle_xrd_price(xrd_price);

    // at first 20 placeholders -> 10*15 + 10*20 = 150 + 200 = 350
    let amount_token = 350 / dec!("0.08");
    helper.buy_placeholders(true,  20u16, amount_token).unwrap();
    
    // buy 45 placeholder: should cost 45*25= 1.125
    let amount_token = 1125 / dec!("0.08");
    helper.buy_placeholders_check(true,  45u16, amount_token, true, dec!("0.0")).unwrap();    
}

#[test]
fn tc_1_11_1_availability_case_max() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // max amount available is 100 - 10 (team) - 0 (buffer) = 90
    let amount_token = 50*price * helper.latest_usd_price ;                
    helper.buy_placeholders(true,  50u16, amount_token).unwrap();

    helper.reserve_nfts_for_usd_sale("CP001".to_owned(), 40).unwrap(); //should succeed
}

#[test]
#[should_panic]
fn tc_1_11_2_availability_case_max_plus_1() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // max amount available is 90 = 100 - 10 (team) - 0 (buffer) = 90
    let amount_token = 50*price * helper.latest_usd_price ;                
    helper.buy_placeholders(true,  50u16, amount_token).unwrap();

    helper.reserve_nfts_for_usd_sale("CP001".to_owned(), 41).unwrap(); //should fail
}

#[test]
fn tc_1_12_1_price_correct_stage1_eq_stage_2_case_range1() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // set price stages here: 15-20-25 USD for 10-20-100
    helper.set_price(dec!("15.0"), dec!("20.0"), dec!("25.0"),  10u16, 10u16).unwrap();

    // buy 5 placeholder: should cost 5x15=75
    let amount_token = 75* helper.latest_usd_price;
    helper.buy_placeholders_check(true,  5u16, amount_token, true, dec!("0.0")).unwrap();    
}

#[test]
fn tc_1_12_2_price_correct_stage1_eq_stage_2_case_range1_and_range_3() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // set price stages here: 15-20-25 USD for 10-20-100
    helper.set_price(dec!("15.0"), dec!("20.0"), dec!("25.0"),  10u16, 10u16).unwrap();

    // buy 15 placeholder: should cost 10x15 + 5*25 =150 + 125 = 275
    let amount_token = 275* helper.latest_usd_price;
    helper.buy_placeholders_check(true,  15u16, amount_token, true, dec!("0.0")).unwrap();    
}

#[test]
fn tc_1_12_3_price_correct_stage1_eq_stage_2_case_range_3() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // set price stages here: 15-20-25 USD for 10-20-100
    helper.set_price(dec!("15.0"), dec!("20.0"), dec!("25.0"),  10u16, 10u16).unwrap();

    // at first 12 placeholders -> 10*15 + 2*25 = 150 + 50 = 200
    let amount_token = 200* helper.latest_usd_price;
    helper.buy_placeholders(true,  12u16, amount_token).unwrap();

    // buy 10 placeholder: should cost 10x25 = 250
    let amount_token = 250* helper.latest_usd_price;
    helper.buy_placeholders_check(true,  10u16, amount_token, true, dec!("0.0")).unwrap();    
}

#[test]
#[should_panic]
fn tc_2_1_1_set_price_case_no_proof() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;         
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();

    helper.env.enable_auth_module();     

    helper.set_price(dec!("15"), dec!("20"), dec!("25"), 10, 20 ).unwrap(); // should fail       
}


#[test]
fn tc_2_1_2_set_price_case_auth_disabled() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();

    helper.env.disable_auth_module();     

    helper.set_price(dec!("15"), dec!("20"), dec!("25"), 10, 20 ).unwrap(); // should succeed now       
}


#[test]
#[should_panic]
fn tc_2_1_3_set_price_case_admin_proof() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();
    
    let _proof = ProofFactory::create_fungible_proof(helper.admin_badge_address.unwrap(), Decimal::ONE, CreationStrategy::Mock, &mut helper.env).unwrap();        

    LocalAuthZone::push(_proof, &mut helper.env).unwrap();
    
    helper.env.enable_auth_module();

    helper.set_price(dec!("15"), dec!("20"), dec!("25"), 10, 20 ).unwrap(); // should succeed now           
}

#[test]
fn tc_2_1_4_set_price_case_super_admin_proof() -> Result<(), RuntimeError> {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();
    
    let _proof = ProofFactory::create_fungible_proof(helper.super_admin_badge_address.unwrap(), Decimal::ONE, CreationStrategy::Mock, &mut helper.env).unwrap();        

    LocalAuthZone::push(_proof, &mut helper.env)?;
    
    helper.env.enable_auth_module();

    helper.set_price(dec!("15"), dec!("20"), dec!("25"), 10, 20 ).unwrap(); // should succeed now       

    Ok(())
}

#[test]
fn tc_2_1_5_set_price_case_owner_proof() -> Result<(), RuntimeError> {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();
    
    let _proof = ProofFactory::create_fungible_proof(helper.owner_badge_address.unwrap(), Decimal::ONE, CreationStrategy::Mock, &mut helper.env).unwrap();        

    LocalAuthZone::push(_proof, &mut helper.env)?;
    
    helper.env.enable_auth_module();

    helper.set_price(dec!("15"), dec!("20"), dec!("25"), 10, 20 ).unwrap(); // should succeed now       

    Ok(())
}

#[test]
#[should_panic]
fn tc_2_1_6_set_price_case_wrong_proof() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();
    
    let _proof = ProofFactory::create_fungible_proof(helper.non_xrd_address, Decimal::ONE, CreationStrategy::Mock, &mut helper.env).unwrap();        

    LocalAuthZone::push(_proof, &mut helper.env).unwrap();
    
    helper.env.enable_auth_module();

    helper.set_price(dec!("15"), dec!("20"), dec!("25"), 10, 20 ).unwrap(); // should succeed now       
}

//todo 2_1_3 how to create proof?

// 2_2_1, 2_3_1 and 2_4_1 ->

#[test]
#[should_panic]
fn tc_2_2_2_price_zero_case_price1_zero() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();

    helper.env.disable_auth_module();     

    helper.set_price(dec!("0"), dec!("20"), dec!("25"), 10, 20 ).unwrap();    // should fail    
}

#[test]
#[should_panic]
fn tc_2_3_2_price_zero_case_price2_zero() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    

    helper.set_price(dec!("15"), dec!("0"), dec!("25"), 10, 20 ).unwrap();     // should fail
}

#[test]
#[should_panic]
fn tc_2_4_2_price_zero_case_price3_zero() {

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();

    helper.set_price(dec!("15"), dec!("20"), dec!("0"), 10, 20 ).unwrap();   // should fail     
}

#[test]
#[should_panic]
fn tc_2_2_3_price_zero_case_price1_neg() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();

    helper.env.disable_auth_module();     

    helper.set_price(dec!("-1"), dec!("20"), dec!("25"), 10, 20 ).unwrap();      // should fail  
}

#[test]
#[should_panic]
fn tc_2_3_3_price_zero_case_price2_neg() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    

    helper.set_price(dec!("15"), dec!("-1"), dec!("25"), 10, 20 ).unwrap();      // should fail  
}

#[test]
#[should_panic]
fn tc_2_4_3_price_zero_case_price3_neg() {

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();

    helper.set_price(dec!("15"), dec!("20"), dec!("-1"), 10, 20 ).unwrap();       // should fail 
}

#[test]
// is allowed
fn tc_2_5_2_price_stage_1_case_zero() {

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();

    helper.set_price(dec!("15"), dec!("20"), dec!("25"), 0, 20).unwrap();        // should succeed
}

#[test]
fn tc_2_6_2_price_stage_2_case_eq_stage1() {

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();

    helper.set_price(dec!("15"), dec!("20"), dec!("25"), 10, 10).unwrap();       // should succeed - is allowed 
}

#[test]
#[should_panic]
fn tc_2_6_3_price_stage_2_case_lt_stage1() {

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();

    helper.set_price(dec!("15"), dec!("20"), dec!("25"), 10, 9).unwrap();        // should fail
}

#[test]
fn tc_2_6_4_price_stage_2_case_eq_coll_size() {

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();

    helper.set_price(dec!("15"), dec!("20"), dec!("25"), 10, 100).unwrap();   // should succeed
}



#[test]
// #[should_panic] --> we allow this, it just means to skip price stage 3 -> but we need to test price range for this
fn tc_2_6_5_price_stage_2_case_gr_coll_size() {

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();

    helper.set_price(dec!("15"), dec!("20"), dec!("25"), 10, 101).unwrap(); // should succeed as well
}

 #[test]
 #[should_panic]
fn tc_3_1_1_assign_placeholder_case_no_proof() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();
    
    // buy 5 phs
    let amount_token = 5 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  5, amount_token).unwrap();
    
    // assign phs
    helper.env.enable_auth_module();
    
    helper.assign_placeholders_to_nfts().unwrap(); // should fail
    
    helper.env.disable_auth_module();
}

#[test]
fn tc_3_1_2_assign_placeholder_case_auth_mod_disabled() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();
    
    // buy 5 phs
    let amount_token = 5 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  5, amount_token).unwrap(); 
    
    // assign phs        
    helper.assign_placeholders_to_nfts().unwrap();    // should succeed
}

#[test]
fn tc_3_1_3_assign_placeholder_case_admin_proof() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();
    
    // buy 5 phs
    let amount_token = 5 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  5, amount_token).unwrap();

    let _proof = ProofFactory::create_fungible_proof(helper.admin_badge_address.unwrap(), Decimal::ONE, CreationStrategy::Mock, &mut helper.env).unwrap();        
    LocalAuthZone::push(_proof, &mut helper.env).unwrap();
    
    // assign phs
    helper.env.enable_auth_module();
    
    helper.assign_placeholders_to_nfts().unwrap(); // should fail
    
    helper.env.disable_auth_module();
}

#[test]
fn tc_3_1_4_assign_placeholder_case_super_admin_proof() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();
    
    // buy 5 phs
    let amount_token = 5 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  5, amount_token).unwrap();

    let _proof = ProofFactory::create_fungible_proof(helper.super_admin_badge_address.unwrap(), Decimal::ONE, CreationStrategy::Mock, &mut helper.env).unwrap();        
    LocalAuthZone::push(_proof, &mut helper.env).unwrap();
    
    // assign phs
    helper.env.enable_auth_module();
    
    helper.assign_placeholders_to_nfts().unwrap(); // should fail
    
    helper.env.disable_auth_module();
}

#[test]
fn tc_3_1_5_assign_placeholder_case_owner_proof() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();
    
    // buy 5 phs
    let amount_token = 5 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  5, amount_token).unwrap();

    let _proof = ProofFactory::create_fungible_proof(helper.owner_badge_address.unwrap(), Decimal::ONE, CreationStrategy::Mock, &mut helper.env).unwrap();        
    LocalAuthZone::push(_proof, &mut helper.env).unwrap();
    
    // assign phs
    helper.env.enable_auth_module();
    
    helper.assign_placeholders_to_nfts().unwrap(); // should fail
    
    helper.env.disable_auth_module();
}

#[test]
 #[should_panic]
fn tc_3_1_6_assign_placeholder_case_wrong_proof() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();
    
    // buy 5 phs
    let amount_token = 5 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  5, amount_token).unwrap();

    let _proof = ProofFactory::create_fungible_proof(helper.non_xrd_address, Decimal::ONE, CreationStrategy::Mock, &mut helper.env).unwrap();        
    LocalAuthZone::push(_proof, &mut helper.env).unwrap();
    
    // assign phs
    helper.env.enable_auth_module();
    
    helper.assign_placeholders_to_nfts().unwrap(); // should fail
    
    helper.env.disable_auth_module();
}

#[test]
#[should_panic]
fn tc_3_2_1_same_transaction_case_buy_ph() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.mint_till_start_sale(100, team_amount).unwrap();
    
    // buy 5 phs
    let amount_token = 5 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  5, amount_token).unwrap();
    
    // assign phs        
    helper.set_do_check_for_same_transaction(true).unwrap();
    helper.assign_placeholders_to_nfts().unwrap();    // should fail
    helper.set_do_check_for_same_transaction(false).unwrap();
}

#[test]
#[should_panic]
fn tc_3_2_2_same_transaction_case_team() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.set_do_check_for_same_transaction(true).unwrap();    
    
    // placeholders for team were not put into SC
    helper.mint_till_start_sale(100, team_amount).unwrap();

    helper.assign_placeholders_to_nfts().unwrap();    
    
    helper.set_do_check_for_same_transaction(false).unwrap();
}

#[test]
#[should_panic]
fn tc_3_3_0_amount_mapped_case_doublecheck() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
    
    // placeholders for team were not put into SC
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // buy 5 phs
    let amount_token = 5 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  5, amount_token).unwrap();

    // expect 5 phs to be assignd: 10 team pyros + 5 new ones
    // so 14 should fail
    helper.assign_placeholders_to_nfts_check(true, 14).unwrap();            
}



// max amount for mapping at once is set to 20

#[test]
fn tc_3_3_1_amount_mapped_case_lt_max() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
    
    // placeholders for team were not put into SC
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // assign (10) placeholders kept outside
    helper.assign_placeholders_to_nfts().unwrap();

    // buy 19 phs
    let amount_token = 19 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  19, amount_token).unwrap();

    // expect 19 phs to be assignd: 10 team pyros were assigned while starting sale
    helper.assign_placeholders_to_nfts_check(true, 19).unwrap();            
}

#[test]
fn tc_3_3_2_amount_mapped_case_eq_max() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
    
    // placeholders for team were not put into SC
    helper.mint_till_start_sale(100, team_amount).unwrap();

    // buy 20 phs
    let amount_token = 20 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  20, amount_token).unwrap();

    // expect 20 phs to be assignd: 10 team pyros were assigned while starting sale
    helper.assign_placeholders_to_nfts_check(true, 20).unwrap();            
}

#[test]
fn tc_3_3_3_amount_mapped_case_gt_max() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
    
    // placeholders for team were not put into SC
    helper.mint_till_start_sale(100, team_amount).unwrap();

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
        
    helper.instantiate(collection_size,  price).unwrap();            
    
    // placeholders for team were not put into SC
    helper.mint_till_start_sale(100, team_amount).unwrap();

    helper.expect_phs_in_bucket(dec!("10")).unwrap();

    // buy 21 phs
    let amount_token = 21 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  21, amount_token).unwrap();

    // we expect 21 + 10 (team)
    helper.expect_phs_in_bucket(dec!("31")).unwrap();
}

#[test]
#[should_panic]
fn check_if_storing_phs_works_double_check1() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
    
    // placeholders for team were not put into SC
    helper.mint_till_start_sale(100, team_amount).unwrap();

    helper.expect_phs_in_bucket(dec!("11")).unwrap(); // should fail    
}

#[test]
#[should_panic]
fn check_if_storing_phs_works_double_check2() {    

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
    
    // placeholders for team were not put into SC
    helper.mint_till_start_sale(100, team_amount).unwrap();

    helper.expect_phs_in_bucket(dec!("10")).unwrap();

    // buy 21 phs
    let amount_token = 21 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  21, amount_token).unwrap();

    helper.expect_phs_in_bucket(dec!("32")).unwrap(); // should fail
}

#[test]
#[should_panic]
fn tc_4_1_1_swap_phs_not_all_phs_mapped_case_not_assigned_at_all() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // buy 10 phs
    let amount_token = 10 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  10, amount_token).unwrap();

    helper.swap_placeholders(1).unwrap(); // should fail since assign was not called in between
}

#[test]
#[should_panic]
fn tc_4_1_2_swap_phs_not_all_phs_mapped_case_only_once_assigned_but_two_needed() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            

    // this will leave 10 phs unassigned    
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // buy 60 phs
    let amount_token = 30 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  30, amount_token).unwrap();
    helper.buy_placeholders(true,  30, amount_token).unwrap();
    
    // 10 for team kept outside = 70 in total
    
    // assign phs: 4x needed since we only assign 20 at once for testing
    helper.assign_placeholders_to_nfts().unwrap();
    helper.assign_placeholders_to_nfts().unwrap();
    helper.assign_placeholders_to_nfts().unwrap();

    // change 1
    helper.swap_placeholders(1).unwrap(); // should fail since previous assign could not assign all
}

#[test]
fn tc_4_1_3_swap_phs_all_phs_mapped_case_call_twice() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // buy 60 phs
    let amount_token = 30 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  30, amount_token).unwrap();
    helper.buy_placeholders(true,  30, amount_token).unwrap();

    // 10 for team kept outside = 70 in total
    
    // assign phs: 4x needed since we only assign 20 at once for testing
    helper.assign_placeholders_to_nfts().unwrap();
    helper.assign_placeholders_to_nfts().unwrap();
    helper.assign_placeholders_to_nfts().unwrap();
    helper.assign_placeholders_to_nfts().unwrap();

    // change 1
    helper.swap_placeholders(1).unwrap(); // should succeed now
}


#[test]
#[should_panic]
fn tc_4_2_1_swap_phs_case_amount_zero() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // buy 30 phs
    let amount_token = 10 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  10, amount_token).unwrap();    
    
    // assign once
    helper.assign_placeholders_to_nfts().unwrap();    

    // change 0
    helper.swap_placeholders(0).unwrap(); // should fail since 0 is not allowed
}


#[test]
fn tc_4_3_1_swap_phs_max_amount_case_eq_max() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // buy 2x30 phs
    let amount_token = 30 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  30, amount_token).unwrap();    
    helper.buy_placeholders(true,  30, amount_token).unwrap();    
    
    // assign
    helper.assign_placeholders_to_nfts().unwrap();    
    helper.assign_placeholders_to_nfts().unwrap();    
    helper.assign_placeholders_to_nfts().unwrap();    
    helper.assign_placeholders_to_nfts().unwrap();    

    // change 50
    helper.swap_placeholders(50).unwrap(); // should succeed
}


#[test]
#[should_panic]
fn tc_4_3_2_swap_phs_max_amount_case_eq_max_plus1() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // buy 2x30 phs
    let amount_token = 30 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  30, amount_token).unwrap();    
    helper.buy_placeholders(true,  30, amount_token).unwrap();    
    
    // assign
    helper.assign_placeholders_to_nfts().unwrap();    
    helper.assign_placeholders_to_nfts().unwrap();    
    helper.assign_placeholders_to_nfts().unwrap();    
    helper.assign_placeholders_to_nfts().unwrap();    

    // change 51
    helper.swap_placeholders(51).unwrap(); // should fail 
}

#[test]
fn tc_4_4_2_swap_phs_case_bucket_larger_than_needed() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // buy 10 phs
    let amount_token = 10 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  10, amount_token).unwrap();        
    
    // assign
    helper.assign_placeholders_to_nfts().unwrap();        

    // change 5, but provide 10 placeholders -> get 5 back
    helper.expect_phs_in_bucket(dec!("20")).unwrap();
    helper.swap_placeholders_check(dec!("10"), 5).unwrap(); // should succeed and we get 5 phs back
    helper.expect_phs_in_bucket(dec!("15")).unwrap(); // from team and 5 back from this call
    helper.expect_pyros_in_bucket(dec!("5")).unwrap();
}

#[test]
#[should_panic]
fn tc_4_4_3_swap_phs_case_bucket_smaller_than_needed() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // buy 10 phs
    let amount_token = 10 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true,  10, amount_token).unwrap();        
    
    // assign
    helper.assign_placeholders_to_nfts().unwrap();        

    // change 10, but only provide 5 placeholders    
    helper.swap_placeholders_check(dec!("5"), 10).unwrap(); // should fail
}


#[test]
#[should_panic]
fn tc_5_1_1_reserve_nfts_case_no_proof() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // reserve 5 pyros
    helper.env.enable_auth_module();
    helper.reserve_nfts_for_usd_sale("CP0012345".to_owned(), 5).unwrap(); // should fail
    
}

#[test]
fn tc_5_1_2_reserve_nfts_case_auth_disabled() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // reserve 5 pyros
    helper.env.disable_auth_module();
    helper.reserve_nfts_for_usd_sale("CP0012345".to_owned(), 5).unwrap(); // should succeed    
}

#[test]
fn tc_5_1_3_reserve_nfts_case_admin_proof() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    let _proof = ProofFactory::create_fungible_proof(helper.admin_badge_address.unwrap(), Decimal::ONE, CreationStrategy::Mock, &mut helper.env).unwrap();        
    LocalAuthZone::push(_proof, &mut helper.env).unwrap();

    // reserve 5 pyros
    helper.env.enable_auth_module();
    helper.reserve_nfts_for_usd_sale("CP0012345".to_owned(), 5).unwrap(); // should fail
    
}

#[test]
fn tc_5_1_4_reserve_nfts_case_super_admin_proof() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    let _proof = ProofFactory::create_fungible_proof(helper.super_admin_badge_address.unwrap(), Decimal::ONE, CreationStrategy::Mock, &mut helper.env).unwrap();        
    LocalAuthZone::push(_proof, &mut helper.env).unwrap();

    // reserve 5 pyros
    helper.env.enable_auth_module();
    helper.reserve_nfts_for_usd_sale("CP0012345".to_owned(), 5).unwrap(); // should fail
    
}

#[test]
fn tc_5_1_5_reserve_nfts_case_owner_proof() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    let _proof = ProofFactory::create_fungible_proof(helper.owner_badge_address.unwrap(), Decimal::ONE, CreationStrategy::Mock, &mut helper.env).unwrap();        
    LocalAuthZone::push(_proof, &mut helper.env).unwrap();

    // reserve 5 pyros
    helper.env.enable_auth_module();
    helper.reserve_nfts_for_usd_sale("CP0012345".to_owned(), 5).unwrap(); // should fail
    
}

#[test]
#[should_panic]
fn tc_5_1_6_reserve_nfts_case_wrong_proof() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    let _proof = ProofFactory::create_fungible_proof(helper.non_xrd_address, Decimal::ONE, CreationStrategy::Mock, &mut helper.env).unwrap();        
    LocalAuthZone::push(_proof, &mut helper.env).unwrap();

    // reserve 5 pyros
    helper.env.enable_auth_module();
    helper.reserve_nfts_for_usd_sale("CP0012345".to_owned(), 5).unwrap(); // should fail
    
}

#[test]
#[should_panic]
fn tc_5_2_1_reserve_nfts_case_call_twice_same_code() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // reserve 5 pyros    
    helper.reserve_nfts_for_usd_sale("CP0012345".to_owned(), 5).unwrap(); 
    helper.reserve_nfts_for_usd_sale("CP0012345".to_owned(), 5).unwrap(); // 2nd call should fail
}

#[test]
fn tc_5_2_2_reserve_nfts_case_call_twice_different_code() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // reserve 5 pyros    
    helper.reserve_nfts_for_usd_sale("CP0012345".to_owned(), 5).unwrap(); 
    helper.reserve_nfts_for_usd_sale("CP0099999".to_owned(), 5).unwrap(); // 2nd call should succeed since different coupon
}

#[test]
fn tc_5_3_1_reserve_nfts_case_eq_max_minus_1() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // 90 available

    // reserve 5 pyros    
    helper.reserve_nfts_for_usd_sale("CP0012345".to_owned(), 30).unwrap(); 
    helper.reserve_nfts_for_usd_sale("CP0022222".to_owned(), 30).unwrap(); 
    helper.reserve_nfts_for_usd_sale("CP0099999".to_owned(), 29).unwrap(); // should succeed
}


#[test]
fn tc_5_3_2_reserve_nfts_case_eq_max() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // 90 available

    // reserve 5 pyros    
    helper.reserve_nfts_for_usd_sale("CP0012345".to_owned(), 30).unwrap(); 
    helper.reserve_nfts_for_usd_sale("CP0022222".to_owned(), 30).unwrap(); 
    helper.reserve_nfts_for_usd_sale("CP0099999".to_owned(), 30).unwrap(); // should succeed
}

#[test]
#[should_panic]
fn tc_5_3_3_reserve_nfts_case_eq_max_plus_1() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // 90 available

    // reserve 5 pyros    
    helper.reserve_nfts_for_usd_sale("CP0012345".to_owned(), 30).unwrap(); 
    helper.reserve_nfts_for_usd_sale("CP0022222".to_owned(), 30).unwrap(); 
    helper.reserve_nfts_for_usd_sale("CP0099999".to_owned(), 31).unwrap(); // should fail
}


#[test]
fn tc_5_5_1_reserve_nfts_before_buy_placeholder_case_eq_max_minus_1() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // 90 available

    // reserve 5 pyros    
    helper.reserve_nfts_for_usd_sale("CP0012345".to_owned(), 30).unwrap(); 
    helper.reserve_nfts_for_usd_sale("CP0022222".to_owned(), 30).unwrap(); 

    let amount_token = 29 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true, 29, amount_token).unwrap();
}

#[test]
fn tc_5_5_2_reserve_nfts_before_buy_placeholder_case_eq_max() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // 90 available

    // reserve 5 pyros    
    helper.reserve_nfts_for_usd_sale("CP0012345".to_owned(), 30).unwrap(); 
    helper.reserve_nfts_for_usd_sale("CP0022222".to_owned(), 30).unwrap(); 

    let amount_token = 30 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true, 30, amount_token).unwrap();
}

#[test]
#[should_panic]
fn tc_5_5_3_reserve_nfts_before_buy_placeholder_case_eq_max_plus_1() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // 90 available

    // reserve 5 pyros    
    helper.reserve_nfts_for_usd_sale("CP0012345".to_owned(), 30).unwrap(); 
    helper.reserve_nfts_for_usd_sale("CP0022222".to_owned(), 30).unwrap(); 

    let amount_token = 31 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true, 31, amount_token).unwrap();
}

#[test]
#[should_panic]
fn tc_6_1_1_get_nfts_case_no_proof() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // reserve 5 pyros
    helper.env.enable_auth_module();
    helper.claim_nfts_for_usd_sale("CP0012345".to_owned(), 5, false).unwrap(); // should fail
    
}

#[test]
fn tc_6_1_2_get_nfts_case_auth_disabled() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // get 5 pyros
    helper.env.disable_auth_module();
    helper.claim_nfts_for_usd_sale("CP0012345".to_owned(), 5, false).unwrap(); // should succeed    
}

#[test]
fn tc_6_1_3_get_nfts_case_admin_proof() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    let _proof = ProofFactory::create_fungible_proof(helper.admin_badge_address.unwrap(), Decimal::ONE, CreationStrategy::Mock, &mut helper.env).unwrap();        
    LocalAuthZone::push(_proof, &mut helper.env).unwrap();

    // reserve 5 pyros
    helper.env.enable_auth_module();
    helper.claim_nfts_for_usd_sale("CP0012345".to_owned(), 5, false).unwrap(); // should fail
    
}

#[test]
fn tc_6_1_4_get_nfts_case_super_admin_proof() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    let _proof = ProofFactory::create_fungible_proof(helper.super_admin_badge_address.unwrap(), Decimal::ONE, CreationStrategy::Mock, &mut helper.env).unwrap();        
    LocalAuthZone::push(_proof, &mut helper.env).unwrap();

    // reserve 5 pyros
    helper.env.enable_auth_module();
    helper.claim_nfts_for_usd_sale("CP0012345".to_owned(), 5, false).unwrap(); // should fail
    
}

#[test]
fn tc_6_1_5_get_nfts_case_owner_proof() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    let _proof = ProofFactory::create_fungible_proof(helper.owner_badge_address.unwrap(), Decimal::ONE, CreationStrategy::Mock, &mut helper.env).unwrap();        
    LocalAuthZone::push(_proof, &mut helper.env).unwrap();

    // reserve 5 pyros
    helper.env.enable_auth_module();
    helper.claim_nfts_for_usd_sale("CP0012345".to_owned(), 5, false).unwrap(); // should fail
    
}

#[test]
#[should_panic]
fn tc_6_1_6_get_nfts_case_wrong_proof() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    let _proof = ProofFactory::create_fungible_proof(helper.non_xrd_address, Decimal::ONE, CreationStrategy::Mock, &mut helper.env).unwrap();        
    LocalAuthZone::push(_proof, &mut helper.env).unwrap();

    // reserve 5 pyros
    helper.env.enable_auth_module();
    helper.claim_nfts_for_usd_sale("CP0012345".to_owned(), 5, false).unwrap(); // should fail
    
}

#[test]
#[should_panic]
fn tc_6_2_1_get_nfts_case_call_twice_same_code() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // get 2x5 pyros    
    helper.claim_nfts_for_usd_sale("CP0012345".to_owned(), 5, false).unwrap(); 
    helper.claim_nfts_for_usd_sale("CP0012345".to_owned(), 5, false).unwrap(); // 2nd call should fail
}

#[test]
fn tc_6_2_2_get_nfts_case_call_twice_different_code() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // get 2x 5 pyros    
    helper.claim_nfts_for_usd_sale("CP0012345".to_owned(), 5, false).unwrap(); 
    helper.claim_nfts_for_usd_sale("CP0099999".to_owned(), 5, false).unwrap(); // 2nd call should succeed since different coupon
}

#[test]
fn tc_6_3_1_get_nfts_case_eq_max_minus_1() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // 90 available

    // get 89 pyros
    helper.claim_nfts_for_usd_sale("CP0012345".to_owned(), 30, false).unwrap(); 
    helper.claim_nfts_for_usd_sale("CP0022222".to_owned(), 30, false).unwrap(); 
    helper.claim_nfts_for_usd_sale("CP0099999".to_owned(), 29, false).unwrap(); // should succeed
}


#[test]
fn tc_6_3_2_get_nfts_case_eq_max() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // 90 available

    // get 90 pyros    
    helper.claim_nfts_for_usd_sale("CP0012345".to_owned(), 30, false).unwrap(); 
    helper.claim_nfts_for_usd_sale("CP0022222".to_owned(), 30, false).unwrap(); 
    helper.claim_nfts_for_usd_sale("CP0099999".to_owned(), 30, false).unwrap(); // should succeed
}

#[test]
#[should_panic]
fn tc_6_3_3_get_nfts_case_eq_max_plus_1() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // 90 available

    // get 91 pyros    
    helper.claim_nfts_for_usd_sale("CP0012345".to_owned(), 30, false).unwrap(); 
    helper.claim_nfts_for_usd_sale("CP0022222".to_owned(), 30, false).unwrap(); 
    helper.claim_nfts_for_usd_sale("CP0099999".to_owned(), 31, false).unwrap(); // should fail
}


#[test]
fn tc_6_5_1_get_nfts_before_buy_placeholder_case_eq_max_minus_1() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // 90 available

    // get 60 + buy 29 = 89 pyros in total   
    helper.claim_nfts_for_usd_sale("CP0012345".to_owned(), 30, false).unwrap(); 
    helper.claim_nfts_for_usd_sale("CP0022222".to_owned(), 30, false).unwrap(); 

    let amount_token = 29 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true, 29, amount_token).unwrap();
}

#[test]
fn tc_6_5_2_get_nfts_before_buy_placeholder_case_eq_max() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // 90 available

    // get 60 + buy 30 = 30 pyros in total   
    helper.claim_nfts_for_usd_sale("CP0012345".to_owned(), 30, false).unwrap(); 
    helper.claim_nfts_for_usd_sale("CP0022222".to_owned(), 30, false).unwrap(); 

    let amount_token = 30 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true, 30, amount_token).unwrap();
}

#[test]
#[should_panic]
fn tc_6_5_3_get_nfts_before_buy_placeholder_case_eq_max_plus_1() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // 90 available

    // get 60 + buy 31 = 91 pyros in total   
    helper.claim_nfts_for_usd_sale("CP0012345".to_owned(), 30, false).unwrap(); 
    helper.claim_nfts_for_usd_sale("CP0022222".to_owned(), 30, false).unwrap(); 

    let amount_token = 31 * 20 * helper.latest_usd_price;
    helper.buy_placeholders(true, 31, amount_token).unwrap();
}

#[test]
fn tc_6_6_1_get_nfts_before_reserve_case_eq_max_minus_1() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // 90 available

    // get 89 pyros
    helper.claim_nfts_for_usd_sale("CP0012345".to_owned(), 30, false).unwrap(); 
    helper.claim_nfts_for_usd_sale("CP0022222".to_owned(), 30, false).unwrap(); 
    helper.reserve_nfts_for_usd_sale("CP0099999".to_owned(), 29).unwrap(); // should succeed
}


#[test]
fn tc_6_6_2_get_nfts_before_reserve_case_eq_max() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // 90 available

    // get 90 pyros    
    helper.claim_nfts_for_usd_sale("CP0012345".to_owned(), 30, false).unwrap(); 
    helper.claim_nfts_for_usd_sale("CP0022222".to_owned(), 30, false).unwrap(); 
    helper.reserve_nfts_for_usd_sale("CP0099999".to_owned(), 30).unwrap(); // should succeed
}

#[test]
#[should_panic]
fn tc_6_6_3_get_nfts_before_reserve_case_eq_max_plus_1() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // 90 available

    // get 91 pyros    
    helper.claim_nfts_for_usd_sale("CP0012345".to_owned(), 30, false).unwrap(); 
    helper.claim_nfts_for_usd_sale("CP0022222".to_owned(), 30, false).unwrap(); 
    helper.reserve_nfts_for_usd_sale("CP0099999".to_owned(), 31).unwrap(); // should fail
}

#[test]
fn tc_6_7_1_get_nfts_case_reserve_get_same_coupon() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // 90 available

    // get 61 pyros    
    helper.claim_nfts_for_usd_sale("CP0012345".to_owned(), 30, false).unwrap();     
    helper.reserve_nfts_for_usd_sale("CP0099999".to_owned(), 31).unwrap(); 
    helper.claim_nfts_for_usd_sale("CP0099999".to_owned(), 31, true).unwrap(); // should succeed
}

#[test]
#[should_panic]
fn tc_6_8_1_get_nfts_case_reserve_get_same_coupon_wrong_amount() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    // assign
    helper.assign_placeholders_to_nfts().unwrap(); 

    // 90 available

    // get 61 pyros    
    helper.claim_nfts_for_usd_sale("CP0012345".to_owned(), 30, false).unwrap();     
    helper.reserve_nfts_for_usd_sale("CP0099999".to_owned(), 31).unwrap(); 
    helper.claim_nfts_for_usd_sale("CP0099999".to_owned(), 32, true).unwrap(); // should fail since 32 <> 31
}

#[test]
fn tc_7_1_1_all_buy_methods_case_last_buy_enough() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");    
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    helper.buy_41_with_all_methods().unwrap(); // should leave 49 left - 16 NFTs - 10 Placeholder

    // 10 (team) + 10 (bought) placeholder
    helper.expect_phs_in_bucket(dec!("20")).unwrap();
    
    // 16 NFTs
    helper.expect_pyros_in_bucket(dec!("16")).unwrap();
    
    // buy 49 placeholder    
    let amount_token = 49 * price * helper.latest_usd_price;                
    helper.buy_placeholders(true, 49, amount_token).unwrap();

    // assign
    helper.assign_placeholders_to_nfts().unwrap();
    helper.assign_placeholders_to_nfts().unwrap();
    helper.assign_placeholders_to_nfts().unwrap();
    helper.assign_placeholders_to_nfts().unwrap();
    helper.assign_placeholders_to_nfts().unwrap();

    
    // 20 + 49 = 69 placeholder
    helper.expect_phs_in_bucket(dec!("69")).unwrap();
    
    // 16 NFTs
    helper.expect_pyros_in_bucket(dec!("16")).unwrap();

    // change 25 placeholder
    helper.swap_placeholders(25).unwrap();

    // 69 - 25 = 44 placeholder
    helper.expect_phs_in_bucket(dec!("44")).unwrap();
    
    // 16 + 25 = 41 NFTs
    helper.expect_pyros_in_bucket(dec!("41")).unwrap();     

    // change remaining 44 placeholder
    helper.swap_placeholders(44).unwrap();

    // 0 placeholder
    helper.expect_phs_in_bucket(dec!("0")).unwrap();
    
    // 41 + 44 = 85 NFTs
    helper.expect_pyros_in_bucket(dec!("85")).unwrap();     

    // remaining 15 are reserved
    helper.claim_nfts_for_usd_sale("CP01".to_owned(), 3, true).unwrap();
    helper.claim_nfts_for_usd_sale("CP04".to_owned(), 12, true).unwrap();

    // no phs left, 100 pyro nfts
    helper.expect_phs_in_bucket(dec!("0")).unwrap();
    helper.expect_pyros_in_bucket(dec!("100")).unwrap();     
}



#[test]
#[should_panic]
fn tc_7_1_2_all_buy_methods_case_last_buy_not_enough() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");    
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    helper.buy_41_with_all_methods().unwrap(); // should leave 49 left - 16 NFTs - 10 Placeholder

    // buy 50 placeholder    
    let amount_token = 50 * price * helper.latest_usd_price;                
    helper.buy_placeholders(true, 50, amount_token).unwrap(); // should fail since only 49 left    
}

#[test]
fn tc_7_2_1_all_buy_methods_case_last_reserve_enough() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");    
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    helper.buy_41_with_all_methods().unwrap(); // should leave 49 left - 16 NFTs - 10 Placeholder

    // reserve 49 placeholder        
    helper.reserve_nfts_for_usd_sale("CP001".to_owned(), 49).unwrap(); // should succeed

    helper.claim_nfts_for_usd_sale("CP001".to_owned(), 49, true).unwrap(); // should succeed as well
}

#[test]
#[should_panic]
fn tc_7_2_2_all_buy_methods_case_last_reserve_not_enough() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");    
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    helper.buy_41_with_all_methods().unwrap(); // should leave 49 left - 16 NFTs - 10 Placeholder

    // reserve 50 placeholder        
    helper.reserve_nfts_for_usd_sale("CP001".to_owned(), 50).unwrap(); // should succeed
}



#[test]
fn tc_7_3_1_all_buy_methods_case_last_get_enough() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");    
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    helper.buy_41_with_all_methods().unwrap(); // should leave 49 left - 16 NFTs - 10 Placeholder

    // get 49 placeholder        
    helper.claim_nfts_for_usd_sale("CP001".to_owned(), 49, false).unwrap(); // should succeed
}

#[test]
#[should_panic]
fn tc_7_3_2_all_buy_methods_case_last_get_not_enough() {

    let mut helper = MigrationHelper::new().unwrap();        
        
    let collection_size = 100u16;     
    let team_amount = 10u16;
    let price = dec!("20");    
        
    helper.instantiate(collection_size,  price).unwrap();            
        
    helper.mint_till_start_sale(100, team_amount).unwrap();    

    helper.buy_41_with_all_methods().unwrap(); // should leave 49 left - 16 NFTs - 10 Placeholder

    // get 50 placeholder        
    helper.claim_nfts_for_usd_sale("CP001".to_owned(), 50, false).unwrap(); // should fail
}

#[test]
#[should_panic]
fn tc_8_1_1_pause_case_no_proof() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;         
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();

    helper.mint_till_start_sale(100, 10).unwrap();

    helper.env.enable_auth_module();
    helper.pause_sale().unwrap() // should fail 
}


#[test]
fn tc_8_1_2_pause_auth_disabled() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;         
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();

    helper.mint_till_start_sale(100, 10).unwrap();
    
    helper.pause_sale().unwrap(); // should succeed since auth module disabled    
}

#[test]
#[should_panic]
fn tc_8_1_3_pause_case_admin_proof() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();

    helper.mint_till_start_sale(100, 10).unwrap();
    
    let _proof = ProofFactory::create_fungible_proof(helper.admin_badge_address.unwrap(), Decimal::ONE, CreationStrategy::Mock, &mut helper.env).unwrap();        

    LocalAuthZone::push(_proof, &mut helper.env).unwrap();
    
    helper.env.enable_auth_module();

    helper.pause_sale().unwrap();    
}

#[test]
fn tc_8_1_4_pause_case_super_admin_proof() -> Result<(), RuntimeError> {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();

    helper.mint_till_start_sale(100, 10).unwrap();
    
    let _proof = ProofFactory::create_fungible_proof(helper.super_admin_badge_address.unwrap(), Decimal::ONE, CreationStrategy::Mock, &mut helper.env).unwrap();        

    LocalAuthZone::push(_proof, &mut helper.env)?;
    
    helper.env.enable_auth_module();

    helper.pause_sale().unwrap();

    Ok(())
}

#[test]
fn tc_8_1_5_pause_case_owner_proof() -> Result<(), RuntimeError> {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();

    helper.mint_till_start_sale(100, 10).unwrap();
    
    let _proof = ProofFactory::create_fungible_proof(helper.owner_badge_address.unwrap(), Decimal::ONE, CreationStrategy::Mock, &mut helper.env).unwrap();        

    LocalAuthZone::push(_proof, &mut helper.env)?;
    
    helper.env.enable_auth_module();

    helper.pause_sale().unwrap();

    Ok(())
}

#[test]
#[should_panic]
fn tc_8_1_6_pause_case_wrong_proof() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;     
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();

    helper.mint_till_start_sale(100, 10).unwrap();
    
    let _proof = ProofFactory::create_fungible_proof(helper.non_xrd_address, Decimal::ONE, CreationStrategy::Mock, &mut helper.env).unwrap();        

    LocalAuthZone::push(_proof, &mut helper.env).unwrap();
    
    helper.env.enable_auth_module();

    helper.pause_sale().unwrap();
}

#[test]
#[should_panic]
fn tc_8_2_1_pause_case_sale_not_started() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;         
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();    
    
    helper.pause_sale().unwrap() // should fail
}


#[test]
#[should_panic]
fn tc_8_3_1_pause_case_buy_placheholder_afterwards() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;         
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();

    helper.mint_till_start_sale(100, 10).unwrap();
    
    helper.pause_sale().unwrap(); // should succeed since auth module disabled    

    let amount_token = price * helper.latest_usd_price;                
    helper.buy_placeholders(true,  1u16, amount_token).unwrap(); // should fail since sale paused
}


#[test]
#[should_panic]
fn tc_8_3_2_pause_case_reserve_afterwards() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;         
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();

    helper.mint_till_start_sale(100, 10).unwrap();
    
    helper.pause_sale().unwrap(); // should succeed since auth module disabled    

    helper.reserve_nfts_for_usd_sale("CP001".to_owned(),  1u16).unwrap(); // should fail since sale paused
}

#[test]
#[should_panic]
fn tc_8_3_3_pause_case_get_nfts_for_usd_afterwards_without_reservation() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;         
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();

    helper.mint_till_start_sale(100, 10).unwrap();
    
    helper.pause_sale().unwrap(); // should succeed since auth module disabled    
    
    helper.claim_nfts_for_usd_sale("CP001".to_owned(),  1u16, false).unwrap(); // should fail since sale paused
}

#[test]
fn tc_8_3_4_pause_case_get_nfts_for_usd_afterwards_with_reservation() {    

    let mut helper = MigrationHelper::new().unwrap();   

    let collection_size = 100u16;         
    let price = dec!("20");
        
    helper.instantiate(collection_size,  price).unwrap();

    helper.mint_till_start_sale(100, 10).unwrap();
    
    helper.reserve_nfts_for_usd_sale("CP001".to_owned(),  1u16).unwrap(); 

    helper.pause_sale().unwrap(); // should succeed since auth module disabled    
    
    helper.claim_nfts_for_usd_sale("CP001".to_owned(),  1u16, true).unwrap(); // this is explictely allowed for users having paid already
}


#[test]
#[should_panic]
fn test_case_complex_1()
{
    let mut helper = MigrationHelper::new().unwrap();                                
    helper.instantiate(100u16,  dec!("20")).unwrap();          

    helper.complex_testcase(1).unwrap();
}

#[test]
#[should_panic]
fn test_case_complex_2()
{
    let mut helper = MigrationHelper::new().unwrap();                                
    helper.instantiate(100u16,  dec!("20")).unwrap();          

    helper.complex_testcase(2).unwrap();
}

#[test]
#[should_panic]
fn test_case_complex_3()
{
    let mut helper = MigrationHelper::new().unwrap();                                
    helper.instantiate(100u16,  dec!("20")).unwrap();          

    helper.complex_testcase(3).unwrap();
}



#[test]
#[should_panic]
fn test_case_complex_4()
{
    let mut helper = MigrationHelper::new().unwrap();                                
    helper.instantiate(100u16,  dec!("20")).unwrap();          

    helper.complex_testcase(4).unwrap();
}

#[test]
#[should_panic]
fn test_case_complex_5()
{
    let mut helper = MigrationHelper::new().unwrap();                                
    helper.instantiate(100u16,  dec!("20")).unwrap();          

    helper.complex_testcase(5).unwrap();
}

#[test]
#[should_panic]
fn test_case_complex_6()
{
    let mut helper = MigrationHelper::new().unwrap();                                
    helper.instantiate(100u16,  dec!("20")).unwrap();          

    helper.complex_testcase(6).unwrap();
}

#[test]
#[should_panic]
fn test_case_complex_7()
{
    let mut helper = MigrationHelper::new().unwrap();                                
    helper.instantiate(100u16,  dec!("20")).unwrap();          

    helper.complex_testcase(7).unwrap();
}

#[test]
#[should_panic]
fn test_case_complex_8()
{
    let mut helper = MigrationHelper::new().unwrap();                                
    helper.instantiate(100u16,  dec!("20")).unwrap();          

    helper.complex_testcase(8).unwrap();
}

#[test]
#[should_panic]
fn test_case_complex_9()
{
    let mut helper = MigrationHelper::new().unwrap();                                
    helper.instantiate(100u16,  dec!("20")).unwrap();          

    helper.complex_testcase(9).unwrap();
}

#[test]
#[should_panic]
fn test_case_complex_10()
{
    let mut helper = MigrationHelper::new().unwrap();                                
    helper.instantiate(100u16,  dec!("20")).unwrap();          

    helper.complex_testcase(10).unwrap();
}

#[test]
#[should_panic]
fn test_case_complex_11()
{
    let mut helper = MigrationHelper::new().unwrap();                                
    helper.instantiate(100u16,  dec!("20")).unwrap();          

    helper.complex_testcase(11).unwrap();
}

#[test]
#[should_panic]
fn test_case_complex_12()
{
    let mut helper = MigrationHelper::new().unwrap();                                
    helper.instantiate(100u16,  dec!("20")).unwrap();          

    helper.complex_testcase(12).unwrap();
}

#[test]
#[should_panic]
fn test_case_complex_13()
{
    let mut helper = MigrationHelper::new().unwrap();                                
    helper.instantiate(100u16,  dec!("20")).unwrap();          

    helper.complex_testcase(13).unwrap();
}

#[test]
fn test_case_complex_14()
{
    let mut helper = MigrationHelper::new().unwrap();                                
    helper.instantiate(100u16,  dec!("20")).unwrap();          

    helper.complex_testcase(14).unwrap();
}

#[test]
#[should_panic]
fn test_case_complex_15()
{
    let mut helper = MigrationHelper::new().unwrap();                                
    helper.instantiate(100u16,  dec!("20")).unwrap();          

    helper.complex_testcase(15).unwrap();
}

#[test]
fn test_case_complex_16()
{
    let mut helper = MigrationHelper::new().unwrap();                                
    helper.instantiate(100u16,  dec!("20")).unwrap();          

    helper.complex_testcase(16).unwrap();
}

#[test]
fn test_case_complex_17()
{
    let mut helper = MigrationHelper::new().unwrap();                                
    helper.instantiate(100u16,  dec!("20")).unwrap();          

    helper.complex_testcase(17).unwrap();
}

#[test]
#[should_panic]
fn test_case_complex_18()
{
    let mut helper = MigrationHelper::new().unwrap();                                
    helper.instantiate(100u16,  dec!("20")).unwrap();          

    helper.complex_testcase(18).unwrap();
}

#[test]
#[should_panic]
fn test_case_complex_19()
{
    let mut helper = MigrationHelper::new().unwrap();                                
    helper.instantiate(100u16,  dec!("20")).unwrap();          

    helper.complex_testcase(19).unwrap();
}

#[test]
#[should_panic]
fn test_case_complex_20()
{
    let mut helper = MigrationHelper::new().unwrap();                                
    helper.instantiate(100u16,  dec!("20")).unwrap();          

    helper.complex_testcase(20).unwrap();
}

#[test]
fn test_case_complex_21()
{
    let mut helper = MigrationHelper::new().unwrap();                                
    helper.instantiate(100u16,  dec!("1")).unwrap();          

    helper.complex_testcase(21).unwrap();
}

#[test]
fn test_case_complex_22()
{
    let mut helper = MigrationHelper::new().unwrap();                                
    helper.instantiate(100u16,  dec!("1")).unwrap();          

    helper.complex_testcase(22).unwrap();
}

#[test]
fn test_case_complex_23()
{
    let mut helper = MigrationHelper::new().unwrap();                                
    helper.instantiate(100u16,  dec!("1")).unwrap();          

    helper.complex_testcase(23).unwrap();
}

// all auth checks for owner

#[test]
fn test_case_auth_case_add_nfts_owner()
{
    let mut helper = MigrationHelper::new().unwrap();
    helper.instantiate(100u16,  dec!("1")).unwrap();                                    

    let action = Action::AddNftsForSale;
    let proof_address = helper.owner_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}

#[test]
fn test_case_auth_case_start_sale_nfts_owner()
{
    let mut helper = MigrationHelper::new().unwrap();      
    helper.instantiate(100u16,  dec!("1")).unwrap();                              

    let action = Action::StartSale;
    let proof_address = helper.owner_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}

#[test]
fn test_case_auth_case_pause_sale_owner()
{
    let mut helper = MigrationHelper::new().unwrap();      
    helper.instantiate(100u16,  dec!("1")).unwrap();                              

    let action = Action::PauseSale;
    let proof_address = helper.owner_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}

#[test]
fn test_case_auth_case_continue_sale_owner()
{
    let mut helper = MigrationHelper::new().unwrap();  
    helper.instantiate(100u16,  dec!("1")).unwrap();                                  

    let action = Action::ContinueSale;
    let proof_address = helper.owner_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}

#[test]
fn test_case_auth_case_use_manual_price_owner()
{
    let mut helper = MigrationHelper::new().unwrap();   
    helper.instantiate(100u16,  dec!("1")).unwrap();                                 

    let action = Action::UseManualUsdPrice;
    let proof_address = helper.owner_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}

#[test]
fn test_case_auth_case_use_oracle_price_owner()
{
    let mut helper = MigrationHelper::new().unwrap();                                    
    helper.instantiate(100u16,  dec!("1")).unwrap();

    let action = Action::UseOracleUsdPrice;
    let proof_address = helper.owner_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}

#[test]
fn test_case_auth_case_withdraw_xrd_owner()
{
    let mut helper = MigrationHelper::new().unwrap();     
    helper.instantiate(100u16,  dec!("1")).unwrap();                               

    let action = Action::WithdrawXRD;
    let proof_address = helper.owner_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}

#[test]
fn test_case_auth_case_collect_nft_owner()
{
    let mut helper = MigrationHelper::new().unwrap();   
    helper.instantiate(100u16,  dec!("1")).unwrap();                                 

    let action = Action::CollectNftsInEmergency;
    let proof_address = helper.owner_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}


// all auth checks for super_admin


#[test]
fn test_case_auth_case_add_nfts_super_admin()
{
    let mut helper = MigrationHelper::new().unwrap();   
    helper.instantiate(100u16,  dec!("1")).unwrap();                                                               

    let action = Action::AddNftsForSale;
    let proof_address = helper.super_admin_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}

#[test]
fn test_case_auth_case_start_sale_nfts_super_admin()
{
    let mut helper = MigrationHelper::new().unwrap();    
    helper.instantiate(100u16,  dec!("1")).unwrap();                                                              

    let action = Action::StartSale;
    let proof_address = helper.super_admin_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}

#[test]
fn test_case_auth_case_pause_sale_super_admin()
{
    let mut helper = MigrationHelper::new().unwrap();     
    helper.instantiate(100u16,  dec!("1")).unwrap();                                                             

    let action = Action::PauseSale;
    let proof_address = helper.super_admin_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}

#[test]
fn test_case_auth_case_continue_sale_super_admin()
{
    let mut helper = MigrationHelper::new().unwrap();     
    helper.instantiate(100u16,  dec!("1")).unwrap();                                                             

    let action = Action::ContinueSale;
    let proof_address = helper.super_admin_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}

#[test]
fn test_case_auth_case_use_manual_price_super_admin()
{
    let mut helper = MigrationHelper::new().unwrap();     
    helper.instantiate(100u16,  dec!("1")).unwrap();                                                             

    let action = Action::UseManualUsdPrice;
    let proof_address = helper.super_admin_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}

#[test]
fn test_case_auth_case_use_oracle_price_super_admin()
{
    let mut helper = MigrationHelper::new().unwrap();       
    helper.instantiate(100u16,  dec!("1")).unwrap();                                                           

    let action = Action::UseOracleUsdPrice;
    let proof_address = helper.super_admin_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}

#[test]
#[should_panic]
fn test_case_auth_case_withdraw_xrd_super_admin()
{
    let mut helper = MigrationHelper::new().unwrap();       
    helper.instantiate(100u16,  dec!("1")).unwrap();                                                           

    let action = Action::WithdrawXRD;
    let proof_address = helper.super_admin_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}

#[test]
#[should_panic]
fn test_case_auth_case_collect_nft_super_admin()
{
    let mut helper = MigrationHelper::new().unwrap();      
    helper.instantiate(100u16,  dec!("1")).unwrap();                                                            

    let action = Action::CollectNftsInEmergency;
    let proof_address = helper.super_admin_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}


// all auth checks for admin


#[test]
fn test_case_auth_case_add_nfts_admin()
{
    let mut helper = MigrationHelper::new().unwrap();   
    helper.instantiate(100u16,  dec!("1")).unwrap();                                                               

    let action = Action::AddNftsForSale;
    let proof_address = helper.admin_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}

#[test]
#[should_panic]
fn test_case_auth_case_start_sale_nfts_admin()
{
    let mut helper = MigrationHelper::new().unwrap();    
    helper.instantiate(100u16,  dec!("1")).unwrap();                                                              

    let action = Action::StartSale;
    let proof_address = helper.admin_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}

#[test]
#[should_panic]
fn test_case_auth_case_pause_sale_admin()
{
    let mut helper = MigrationHelper::new().unwrap();     
    helper.instantiate(100u16,  dec!("1")).unwrap();                                                             

    let action = Action::PauseSale;
    let proof_address = helper.admin_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}

#[test]
#[should_panic]
fn test_case_auth_case_continue_sale_admin()
{
    let mut helper = MigrationHelper::new().unwrap();     
    helper.instantiate(100u16,  dec!("1")).unwrap();                                                             

    let action = Action::ContinueSale;
    let proof_address = helper.admin_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}

#[test]
#[should_panic]
fn test_case_auth_case_use_manual_price_admin()
{
    let mut helper = MigrationHelper::new().unwrap();     
    helper.instantiate(100u16,  dec!("1")).unwrap();                                                             

    let action = Action::UseManualUsdPrice;
    let proof_address = helper.admin_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}

#[test]
#[should_panic]
fn test_case_auth_case_use_oracle_price_admin()
{
    let mut helper = MigrationHelper::new().unwrap();       
    helper.instantiate(100u16,  dec!("1")).unwrap();                                                           

    let action = Action::UseOracleUsdPrice;
    let proof_address = helper.admin_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}

#[test]
#[should_panic]
fn test_case_auth_case_withdraw_xrd_admin()
{
    let mut helper = MigrationHelper::new().unwrap();       
    helper.instantiate(100u16,  dec!("1")).unwrap();                                                           

    let action = Action::WithdrawXRD;
    let proof_address = helper.admin_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}

#[test]
#[should_panic]
fn test_case_auth_case_collect_nft_admin()
{
    let mut helper = MigrationHelper::new().unwrap();      
    helper.instantiate(100u16,  dec!("1")).unwrap();                                                            

    let action = Action::CollectNftsInEmergency;
    let proof_address = helper.admin_badge_address.unwrap();

    helper.auth_testcase(proof_address, action).unwrap();
}