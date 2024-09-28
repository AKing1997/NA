# Security Token on Radix based on ERC-3643 with OnChainID

## Overview
This project demonstrates a simple Security Token implementation on the Radix network, inspired by the ERC-3643 protocol and based on OnChainID. ERC-3643 is a standard for security tokens that incorporates identity management to ensure compliance with regulations. The implementation focuses on creating a token that is managed and traded based on identity and permissions.

## Key Features
- **OnChain Identity Management**: Supports roles like admin, manager, agent, and client, similar to the OnChainID protocol, ensuring only authorized actors can interact with the token.
- **Compliance via Identity Verification**: All token minting, transfer, and asset interactions are governed by verified identities, as required by ERC-3643.
- **Tokenized Real-World Assets**: Provides functionality to mint both fungible and non-fungible tokens (NFTs) representing real-world assets.
- **XRD and Secure Token Exchange**: Allows clients to exchange XRD for security tokens based on a defined price per token.

## What is ERC-3643?
ERC-3643 is a standard for issuing and managing permissioned tokens on the blockchain, where transfers and interactions are controlled by identities registered on-chain. It supports real-world use cases such as:
- Regulated security tokens
- Compliance with know-your-customer (KYC) and anti-money laundering (AML) requirements
- Enforcing rules and restrictions on transfers

The standard is commonly used for creating security tokens that can be transferred only between authorized wallets and entities.

## What is OnChainID?
OnChainID is a digital identity solution for decentralized networks. It allows for on-chain identity verification, so token transfers and other sensitive operations can be carried out based on the verified identities of participants. OnChainID ensures that only KYC-compliant actors can interact with certain assets, following the rules set by regulators.

This project aims to implement an OnChainID-like solution for identity management in the Radix network, which integrates with the functionalities defined by ERC-3643.

## Project Structure
- **security_token.rs**: The main blueprint defining the SecurityToken component.
- **Identity Roles**: The blueprint supports multiple roles including admin, manager, agent, and client. These roles are associated with non-fungible tokens (NFTs) that represent verified identities.
- **Real-World Assets**: The blueprint allows minting of both fungible and non-fungible tokens representing real-world assets.
- **Token Exchange**: A method for exchanging XRD tokens for the security tokens based on a defined price.

## Functions Overview
### Identity Management:
- `create_identity_manager`: Mints a non-fungible token (NFT) representing a manager.
- `create_identity_agent`: Mints an NFT for an agent role.
- `create_identity_client`: Mints an NFT for a client role.

### Asset Management:
- `mint_real_world_asset_fungible`: Mints fungible tokens representing real-world assets.
- `mint_real_world_asset_non_fungible`: Mints non-fungible tokens for unique assets.

### Token Exchange:
- `exchange_xrd_for_secure_token`: Allows clients to exchange XRD tokens for security tokens at a predefined price.

### Admin Controls:
- `change_price_per_token`: Admin can update the price of the security token.

## Setup and Deployment

### Prerequisites
Before running the project, ensure you have the following:
- **Radix CLI (resim)**: Installed and set up. Follow the Radix documentation to install the necessary tools.
- **Rust and Scrypto**: The project is written in Rust using the Scrypto framework for Radix.

### Steps to Deploy
1. Clone the repository:
    ```bash
    git clone https://github.com/AKing1997/NA.git
    cd NA
    ```

2. Compile the Blueprint: Ensure you are in the root of the project directory, and run:
    ```bash
    scrypto build
    ```

3. Deploy the Blueprint: Deploy the compiled blueprint to the Radix network:
    ```bash
    resim publish .
    ```

4. Instantiate the Component: After publishing the blueprint, instantiate the SecurityToken component. Refer to the `security_token.rtm` manifest provided for an example of how to interact with the component and initialize the token:
    ```bash
    resim run security_token.rtm
    ```

5. Interact with the Component: Use the manifest file or individual Radix CLI commands to interact with the deployed SecurityToken component. You can create identities, mint tokens, and exchange XRD for security tokens.

### Example Usage
- Mint Real-World Asset Fungible Token:
    ```bash
    resim run mint_real_world_asset_fungible.rtm
    ```

- Exchange XRD for Security Tokens:
    ```bash
    resim run exchange_xrd_for_tokens.rtm
    ```

- Get Component Addresses: You can retrieve the addresses of manager, agent, and client badges along with vault addresses using:
    ```bash
    resim run get_addresses.rtm
    ```

## Next Steps
While this project implements the core ideas of ERC-3643 with OnChainID functionality, there are several potential improvements:
- Integration with External Identity Providers: For enhanced KYC/AML compliance, integrate with third-party identity verification services.
- Enhanced Permissioning: Extend role-based access to fine-grained operations within the token contract.
- Compliance Automation: Add automation around regulatory checks and reporting, especially in relation to asset ownership and transfers.

## Contributing
Feel free to open issues or submit pull requests for improvements, bug fixes, or new features.

Contributions are welcome!
