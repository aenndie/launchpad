mod helper;

#[cfg(test)]
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

}