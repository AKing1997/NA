use scrypto::prelude::*;

#[derive(ScryptoSbor, ManifestSbor, NonFungibleData, Clone)]
pub struct IdentityData {
    pub identity_id: String,
    pub name: String,
    pub lastname: String
}

#[derive(ScryptoSbor, NonFungibleData, Clone)]
pub struct RealWorldAsset {
    pub asset_id: String,
    pub name: String,
    pub description: String,
    pub value: Decimal,
}

#[blueprint]
mod security_token {
    enable_method_auth! {
        roles {
            admin => updatable_by: [OWNER];
            manager => updatable_by: [admin];
            agent => updatable_by: [manager, admin];
            client => updatable_by: [agent, manager, admin];
        },
        methods {
            price_per_token => PUBLIC;
            get_addresses => restrict_to: [admin];
            create_identity_manager => restrict_to: [admin];
            create_identity_agent => restrict_to: [manager, admin];
            create_identity_client => restrict_to: [agent, manager, admin];
            exchange_xrd_for_secure_token => restrict_to: [client];
            change_price_per_token => restrict_to: [manager, admin];
            mint_real_world_asset_fungible => restrict_to: [client, admin];
            mint_real_world_asset_non_fungible => restrict_to: [client, admin];
        }
    }

    struct SecurityToken {
        manager_badge_manager: ResourceManager,
        agent_badge_manager: ResourceManager,
        client_badge_manager: ResourceManager,
        xrd_vault: Vault,
        secure_token: Vault,
        price_per_token: Decimal,
        component_address: ComponentAddress,
        asset_non_fungible_manager: ResourceManager,
    }

    impl SecurityToken {
        pub fn new(price_per_token: Decimal, radix_token_address: ResourceAddress) -> (Global<SecurityToken>, Bucket) {
            let (address_reservation, component_address) = Runtime::allocate_component_address(SecurityToken::blueprint_id());

            let owner_badge: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .metadata(metadata!(init {
                    "name" => "Identity Owner", locked;
                }))
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1).into();

            let manager_badge_manager = Self::create_badge("Identity Manager", component_address);
            let agent_badge_manager = Self::create_badge("Identity Agent", component_address);
            let client_badge_manager = Self::create_badge("Identity Client", component_address);

            let xrd_vault = Vault::new(radix_token_address);

            let secure_token = ResourceBuilder::new_fungible(OwnerRole::None)
            .metadata(metadata!(init {
                "name" => "Security Token", locked;
            }))
            .mint_roles(mint_roles! {
                minter => rule!(require(global_caller(component_address)));
                minter_updater => rule!(require(owner_badge.resource_address()));
            })
            .burn_roles(burn_roles! {
                burner => rule!(require(global_caller(component_address)));
                burner_updater => rule!(require(owner_badge.resource_address()));
            })
            .withdraw_roles(withdraw_roles! {
                withdrawer => rule!(require(client_badge_manager.address()) || require(owner_badge.resource_address()) || require(global_caller(component_address)));
                withdrawer_updater => rule!(require(owner_badge.resource_address()));
            })
            .deposit_roles(deposit_roles! {
                depositor => rule!(require(client_badge_manager.address()) || require(owner_badge.resource_address()) || require(global_caller(component_address)));
                depositor_updater => rule!(require(owner_badge.resource_address()));
            })
            .freeze_roles(freeze_roles! {
                freezer => rule!(require(global_caller(component_address)));
                freezer_updater => rule!(require(owner_badge.resource_address()));
            })
            .recall_roles(recall_roles! {
                recaller => rule!(require(global_caller(component_address)));
                recaller_updater => rule!(require(owner_badge.resource_address()));
            })
            .divisibility(DIVISIBILITY_MAXIMUM)
            .mint_initial_supply(1000000)
            .into();

            let owner_proof = owner_badge.as_fungible().create_proof_of_amount(1);

            let asset_non_fungible_manager: ResourceManager = ResourceBuilder::new_string_non_fungible::<RealWorldAsset>(OwnerRole::None)
            .metadata(metadata!(init {
                "name" => "Real World Asset", locked;
            }))
            .mint_roles(mint_roles! {
                minter => rule!(require(global_caller(component_address)));
                minter_updater => rule!(require(owner_badge.resource_address()));
            })
            .burn_roles(burn_roles! {
                burner => rule!(require(global_caller(component_address)));
                burner_updater => rule!(require(owner_badge.resource_address()));
            })
            .withdraw_roles(withdraw_roles! {
                withdrawer => rule!(require(client_badge_manager.address()) || require(owner_badge.resource_address()) || require(global_caller(component_address)));
                withdrawer_updater => rule!(require(owner_badge.resource_address()));
            })
            .deposit_roles(deposit_roles! {
                depositor => rule!(require(client_badge_manager.address()) || require(owner_badge.resource_address()) || require(global_caller(component_address)));
                depositor_updater => rule!(require(owner_badge.resource_address()));
            })
            .freeze_roles(freeze_roles! {
                freezer => rule!(require(global_caller(component_address)));
                freezer_updater => rule!(require(owner_badge.resource_address()));
            })
            .recall_roles(recall_roles! {
                recaller => rule!(require(global_caller(component_address)));
                recaller_updater => rule!(require(owner_badge.resource_address()));
            })
            .create_with_no_initial_supply();
    
            LocalAuthZone::push(owner_proof);

            let identity_instance: SecurityToken = Self {
                manager_badge_manager,
                agent_badge_manager,
                client_badge_manager,
                xrd_vault,
                secure_token: Vault::with_bucket(secure_token),
                price_per_token,
                component_address,
                asset_non_fungible_manager
            };

            let global_identity: Global<SecurityToken> = identity_instance
                .instantiate()
                .prepare_to_globalize(OwnerRole::Fixed(rule!(require(owner_badge.resource_address()))))
                .roles(roles! {
                    admin => rule!(require(owner_badge.resource_address()));
                    manager => rule!(require(manager_badge_manager.address()));
                    agent => rule!(require(agent_badge_manager.address()));
                    client => rule!(require(client_badge_manager.address()));
                })
                .with_address(address_reservation)
                .globalize();

            (global_identity, owner_badge)
        }

        pub fn get_addresses(&self) -> HashMap<String, ResourceAddress> {
            let mut addresses = HashMap::new();
            addresses.insert("Manager Badge Address".to_string(), self.manager_badge_manager.address());
            addresses.insert("Agent Badge Address".to_string(), self.agent_badge_manager.address());
            addresses.insert("Client Badge Address".to_string(), self.client_badge_manager.address());
            addresses.insert("XRD Vault Address".to_string(), self.xrd_vault.resource_address());
            addresses
        }
        
        pub fn mint_real_world_asset_fungible(
            &self,
            name: String,
            assert_id: String,
            description: String,
            initial_supply: Decimal,
            divisibility: u8
        ) -> Bucket {
            ResourceBuilder::new_fungible(OwnerRole::None)
            .metadata(metadata!(init {
                "name" => name.clone(), updatable;
                "assert_id" => assert_id.clone(), updatable;
                "description" => description.clone(), updatable;
            }))
            .mint_roles(mint_roles! {
                minter => rule!(require(global_caller(self.component_address)));
                minter_updater => rule!(require(global_caller(self.component_address)));
            })
            .burn_roles(burn_roles! {
                burner => rule!(require(global_caller(self.component_address)));
                burner_updater => rule!(require(global_caller(self.component_address)));
            })
            .withdraw_roles(withdraw_roles! {
                withdrawer => rule!(require(self.client_badge_manager.address()) || require(global_caller(self.component_address)));
                withdrawer_updater => rule!(require(global_caller(self.component_address)));
            })
            .deposit_roles(deposit_roles! {
                depositor => rule!(require(self.client_badge_manager.address()) || require(global_caller(self.component_address)));
                depositor_updater => rule!(require(global_caller(self.component_address)));
            })
            .freeze_roles(freeze_roles! {
                freezer => rule!(require(global_caller(self.component_address)));
                freezer_updater => rule!(require(global_caller(self.component_address)));
            })
            .recall_roles(recall_roles! {
                recaller => rule!(require(global_caller(self.component_address)));
                recaller_updater => rule!(require(global_caller(self.component_address)));
            })
            .divisibility(divisibility)
            .mint_initial_supply(initial_supply)
            .into()
        }

        pub fn mint_real_world_asset_non_fungible(
            &mut self,
            asset_id: String,
            name: String,
            description: String,
            value: Decimal
        ) -> Bucket {
            let real_world_asset = RealWorldAsset {
                asset_id: asset_id.clone(),
                name,
                description,
                value
            };
            let non_fungible_id = NonFungibleLocalId::string(asset_id.clone()).expect("Failed to create NonFungibleLocalId");

            self.asset_non_fungible_manager.mint_non_fungible(&non_fungible_id, real_world_asset.clone())
        }

        pub fn exchange_xrd_for_secure_token(&mut self, amount_of_xrd: Bucket) -> Bucket {

            let token_amount: Decimal = amount_of_xrd.amount() / self.price_per_token;
            assert!(self.secure_token.amount() >= token_amount, "Not enough tokens in the vault.");
            self.xrd_vault.put(amount_of_xrd);
            self.secure_token.take(token_amount)
        }

        pub fn price_per_token(&mut self) -> Decimal {
            self.price_per_token
        }

        pub fn change_price_per_token(&mut self, new_price: Decimal) {
            self.price_per_token = new_price;
        }

        pub fn create_identity_manager(&mut self, identity_id: String, name: String, lastname: String, mut account: Global<Account>) {
            let identity_data = IdentityData { identity_id: identity_id.clone(), name, lastname };
            let non_fungible_id = NonFungibleLocalId::string(identity_id.clone()).expect("Failed to create NonFungibleLocalId");

            let identity_bucket = self.manager_badge_manager.mint_non_fungible(&non_fungible_id, identity_data.clone());
            account.try_deposit_or_abort(identity_bucket, None);
        }

        pub fn create_identity_agent(&mut self, identity_id: String, name: String, lastname: String, mut account: Global<Account>) {
            let identity_data = IdentityData { identity_id: identity_id.clone(), name, lastname };
            let non_fungible_id = NonFungibleLocalId::string(identity_id.clone()).expect("Failed to create NonFungibleLocalId");

            let identity_bucket = self.agent_badge_manager.mint_non_fungible(&non_fungible_id, identity_data.clone());
            account.try_deposit_or_abort(identity_bucket, None);
        }

        pub fn create_identity_client(&mut self, identity_id: String, name: String, lastname: String, mut account: Global<Account>) {
            let identity_data = IdentityData { identity_id: identity_id.clone(), name, lastname };
            let non_fungible_id = NonFungibleLocalId::string(identity_id.clone()).expect("Failed to create NonFungibleLocalId");

            let identity_bucket = self.client_badge_manager.mint_non_fungible(&non_fungible_id, identity_data.clone());
            account.try_deposit_or_abort(identity_bucket, None);
        }

        fn create_badge(name: &str, component_address: ComponentAddress) -> ResourceManager {
            ResourceBuilder::new_string_non_fungible::<IdentityData>(OwnerRole::None)
                .metadata(metadata! {
                    init {
                        "name" => name, locked;
                        "description" => "This represents an identity badge", locked;
                    }
                })
                .mint_roles(mint_roles! {
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(deny_all);
                })
                .burn_roles(burn_roles! {
                    burner => rule!(require(global_caller(component_address)));
                    burner_updater => rule!(deny_all);
                })
                .withdraw_roles(withdraw_roles! {
                    withdrawer => rule!(deny_all);
                    withdrawer_updater => rule!(deny_all);
                })
                .create_with_no_initial_supply()
        }
    }
}