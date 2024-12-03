#[cfg(test)]
mod tests {
    use crate::InvestmentVault;
    use stylus_sdk::alloy_primitives::{U256, Address};

    // Helper function to create a test address
    fn test_address(num: u8) -> Address {
        Address::with_last_byte(num)
    }

    #[test]
    fn test_initialization() {
        let mut vault = InvestmentVault::default();
        let usdt = test_address(1);
        let project = test_address(2);
        let brand = test_address(3);

        vault.initialize(usdt, project, brand);

        assert_eq!(vault.usdt_token.get(), usdt);
        assert_eq!(vault.project_token.get(), project);
        assert_eq!(vault.brand_wallet.get(), brand);
        assert_eq!(vault.distribution_active.get(), false);
        assert_eq!(vault.total_deposits.get(), U256::ZERO);
    }

    #[test]
    fn test_deposit_usdt() {
        let mut vault = InvestmentVault::default();
        let usdt = test_address(1);
        let project = test_address(2);
        let brand = test_address(3);
        let investor = test_address(4);

        vault.initialize(usdt, project, brand);
        
        // Note: In a real test environment, we would need to mock the USDT contract
        // Here we're just testing the state changes
        let deposit_amount = U256::from(1000);
        
        // Test deposit updates
        let result = vault.deposit_usdt(deposit_amount);
        assert!(result.is_ok());
        
        assert_eq!(vault.get_deposits(investor), deposit_amount);
        assert_eq!(vault.get_total_deposits(), deposit_amount);
        assert_eq!(vault.get_allocation(investor), deposit_amount); // 1:1 ratio
    }

    #[test]
    fn test_distribution_activation() {
        let mut vault = InvestmentVault::default();
        let usdt = test_address(1);
        let project = test_address(2);
        let brand = test_address(3);

        vault.initialize(usdt, project, brand);
        
        // Test activation by non-brand wallet (should fail)
        let result = vault.start_distribution();
        assert!(result.is_err());
        assert_eq!(vault.is_distribution_active(), false);

        // TODO: Test activation by brand wallet
        // This would require mocking msg::sender()
    }

    #[test]
    fn test_eligibility_checks() {
        let mut vault = InvestmentVault::default();
        let usdt = test_address(1);
        let project = test_address(2);
        let brand = test_address(3);
        let investor = test_address(4);

        vault.initialize(usdt, project, brand);
        
        // Initially not eligible (no allocation)
        assert_eq!(vault.check_eligibility(investor), false);
        
        // After deposit (simulated)
        let deposit_amount = U256::from(1000);
        let _ = vault.deposit_usdt(deposit_amount);
        
        // Should be eligible after deposit and before claiming
        assert_eq!(vault.check_eligibility(investor), true);
    }

    #[test]
    fn test_token_claiming() {
        let mut vault = InvestmentVault::default();
        let usdt = test_address(1);
        let project = test_address(2);
        let brand = test_address(3);
        let investor = test_address(4);

        vault.initialize(usdt, project, brand);
        
        // Test claiming before distribution is active
        let result = vault.claim_tokens();
        assert!(result.is_err());
        
        // Simulate deposit and distribution activation
        let deposit_amount = U256::from(1000);
        let _ = vault.deposit_usdt(deposit_amount);
        let _ = vault.start_distribution();
        
        // Test claiming
        let result = vault.claim_tokens();
        assert!(result.is_ok());
        
        // Check claimed status
        assert_eq!(vault.is_claimed(investor), true);
        
        // Test double claiming (should fail)
        let result = vault.claim_tokens();
        assert!(result.is_err());
    }

    #[test]
    fn test_getter_functions() {
        let mut vault = InvestmentVault::default();
        let usdt = test_address(1);
        let project = test_address(2);
        let brand = test_address(3);
        let investor = test_address(4);

        vault.initialize(usdt, project, brand);
        
        // Test initial values
        assert_eq!(vault.get_allocation(investor), U256::ZERO);
        assert_eq!(vault.get_deposits(investor), U256::ZERO);
        assert_eq!(vault.is_claimed(investor), false);
        assert_eq!(vault.is_distribution_active(), false);
        assert_eq!(vault.get_total_deposits(), U256::ZERO);
        
        // Test after deposit
        let deposit_amount = U256::from(1000);
        let _ = vault.deposit_usdt(deposit_amount);
        
        assert_eq!(vault.get_allocation(investor), deposit_amount);
        assert_eq!(vault.get_deposits(investor), deposit_amount);
        assert_eq!(vault.get_total_deposits(), deposit_amount);
    }
} 