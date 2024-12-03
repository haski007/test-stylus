// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{
    alloy_primitives::{U256, Address},
    prelude::*,
    storage::{StorageMap, StorageUint, StorageAddress, StorageBool},
    msg,
    call::Call,
};

// Add this macro for require statements
macro_rules! require {
    ($cond:expr, $msg:expr) => {
        if !$cond {
            return Err(Vec::from($msg))
        }
    };
}

// USDT interface for deposits
sol_interface! {
    interface IUSDT {
        function transferFrom(address from, address to, uint256 value) external returns (bool);
        function balanceOf(address account) external view returns (uint256);
    }
}

// Project token interface for distribution
sol_interface! {
    interface IProjectToken {
        function transfer(address to, uint256 value) external returns (bool);
        function transferFrom(address from, address to, uint256 value) external returns (bool);
        function balanceOf(address account) external view returns (uint256);
    }
}

sol_storage! {
    #[entrypoint]
    pub struct InvestmentVault {
        address usdt_token;
        address project_token;
        address brand_wallet;
        bool distribution_active;
        uint256 total_deposits;
        mapping(address => uint256) deposits;
        mapping(address => uint256) token_allocations;
        mapping(address => bool) claimed;
    }
}

impl Default for InvestmentVault {
    fn default() -> Self {
        Self {
            usdt_token: unsafe { StorageAddress::new(U256::from(0), 0) },
            project_token: unsafe { StorageAddress::new(U256::from(1), 0) },
            brand_wallet: unsafe { StorageAddress::new(U256::from(2), 0) },
            distribution_active: unsafe { StorageBool::new(U256::from(3), 0) },
            total_deposits: unsafe { StorageUint::new(U256::from(4), 0) },
            deposits: unsafe { StorageMap::new(U256::from(5), 0) },
            token_allocations: unsafe { StorageMap::new(U256::from(6), 0) },
            claimed: unsafe { StorageMap::new(U256::from(7), 0) },
        }
    }
}

#[public]
impl InvestmentVault {
    /// Initialize the contract with necessary addresses
    pub fn initialize(
        &mut self,
        usdt_address: Address,
        project_token: Address,
        brand_wallet: Address,
    ) {
        // Check if already initialized
        if self.usdt_token.get() == Address::ZERO {
            self.usdt_token.set(usdt_address);
            self.project_token.set(project_token);
            self.brand_wallet.set(brand_wallet);
        }
    }

    /// Deposit USDT tokens and calculate project token allocation
    pub fn deposit_usdt(&mut self, amount: U256) -> Result<bool, Vec<u8>> {
        let sender = msg::sender();
        let usdt = IUSDT::new(self.usdt_token.get());
        let brand_wallet = self.brand_wallet.get();
        
        // Transfer USDT from sender to brand wallet
        usdt.transfer_from(
            Call::new_in(self),
            sender,
            brand_wallet,
            amount
        )?;

        // Update deposits
        let current_deposit = self.deposits.get(sender);
        self.deposits.insert(sender, current_deposit + amount);
        
        // Update total deposits
        let total = self.total_deposits.get();
        self.total_deposits.set(total + amount);

        // Calculate and store token allocation (example: 1:1 ratio)
        let current_allocation = self.token_allocations.get(sender);
        self.token_allocations.insert(sender, current_allocation + amount);

        Ok(true)
    }

    /// Start token distribution
    pub fn start_distribution(&mut self) -> Result<bool, Vec<u8>> {
        require!(msg::sender() == self.brand_wallet.get(), "Only brand wallet can start distribution");
        self.distribution_active.set(true);
        Ok(true)
    }

    /// Check if an address is eligible for claiming tokens
    pub fn check_eligibility(&self, user: Address) -> bool {
        let has_allocation = self.token_allocations.get(user) > U256::ZERO;
        let not_claimed = !self.claimed.get(user);
        has_allocation && not_claimed
    }

    /// Claim tokens
    pub fn claim_tokens(&mut self) -> Result<bool, Vec<u8>> {
        let sender = msg::sender();
        require!(self.distribution_active.get(), "Distribution not active");
        require!(self.check_eligibility(sender), "Not eligible for claim");

        let allocation = self.token_allocations.get(sender);
        require!(allocation > U256::ZERO, "No allocation");

        let project_token = IProjectToken::new(self.project_token.get());
        
        // Transfer tokens to the investor
        project_token.transfer(Call::new_in(self), sender, allocation)?;
        
        // Mark as claimed
        self.claimed.insert(sender, true);
        
        Ok(true)
    }

    // Getter functions
    pub fn get_allocation(&self, user: Address) -> U256 {
        self.token_allocations.get(user)
    }

    pub fn get_deposits(&self, user: Address) -> U256 {
        self.deposits.get(user)
    }

    pub fn is_claimed(&self, user: Address) -> bool {
        self.claimed.get(user)
    }

    pub fn is_distribution_active(&self) -> bool {
        self.distribution_active.get()
    }

    pub fn get_total_deposits(&self) -> U256 {
        self.total_deposits.get()
    }
}

#[cfg(test)]
mod lib_test;