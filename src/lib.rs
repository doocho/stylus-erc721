// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

// Modules and imports
mod erc721;

use crate::erc721::{Erc721, Erc721Error, Erc721Params};
use alloy_primitives::{Address, U256};
/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{msg, prelude::*};
// use crate::ownable::Ownable;

// mod ownable;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

/// Immutable definitions
struct StylusNFTParams;
impl Erc721Params for StylusNFTParams {
    const NAME: &'static str = "VeriWell NFT";
    const SYMBOL: &'static str = "VWNFT";

    fn token_uri(token_id: U256) -> String {
        format!(
            "{}",
            "https://veriwell-nft.s3.us-east-1.amazonaws.com/veriwell.json"
        )
    }
}

// Define the entrypoint as a Solidity storage object. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
sol_storage! {
    #[entrypoint]
    struct StylusNFT {
        #[borrow] // Allows erc721 to access StylusNFT's storage and make calls
        Erc721<StylusNFTParams> erc721;
        // #[borrow]
        // Ownable ownable;
    }
}

#[external]
#[inherit(Erc721<StylusNFTParams>)]
// #[inherit(Erc721<StylusNFTParams>, Ownable)]
impl StylusNFT {
    /// Mints an NFT
    pub fn mint(&mut self) -> Result<(), Erc721Error> {
        // self.ownable.only_owner();
        let minter = msg::sender();
        self.erc721.mint(minter)?;
        Ok(())
    }

    /// Mints an NFT to another address
    pub fn mint_to(&mut self, to: Address) -> Result<(), Erc721Error> {
        // self.ownable.only_owner();
        self.erc721.mint(to)?;
        Ok(())
    }

    /// Burns an NFT
    pub fn burn(&mut self, token_id: U256) -> Result<(), Erc721Error> {
        // This function checks that msg::sender() owns the specified token_id
        self.erc721.burn(msg::sender(), token_id)?;
        Ok(())
    }

    /// Total supply
    pub fn total_supply(&mut self) -> Result<U256, Erc721Error> {
        Ok(self.erc721.total_supply.get())
    }
}
