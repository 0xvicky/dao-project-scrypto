use scrypto::prelude::*;

#[blueprint]
mod dao {

    struct Dao {
        // Define what resources and data will be managed by Hello components
        admin_badge_addr: ResourceAddress,
        dao_token: Vault,
        treasury: Vault,
        entry_fee: Decimal,
        total_members: Decimal,
    }

    impl Dao {
        // Implement the functions and methods which will manage those resources and data
        // This is a function, and can be called directly on the blueprint once deployed
        pub fn new(entry_fee: Decimal) -> (Global<Dao>, Bucket) {
            //Function to create the authorities : SEller
            pub fn instantiate_authority(name: &str, symbol: &str) -> Bucket {
                let badge: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
                    .metadata(metadata!(
                      init{
                        "name"=>name,locked;
                        "symbol"=>symbol,locked;
                        "description"=>"One who can withdraw XRD and can entry_fee",locked;
                      }
                    ))
                    .divisibility(DIVISIBILITY_NONE)
                    .mint_initial_supply(1);

                badge
            }

            let dao_bucket: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .metadata(metadata!(
                    init {
                        "name" => "DAO Token", locked;
                        "symbol" => "R-DAO", locked;
                    }
                ))
                .mint_initial_supply(0);

            let admin_badge = instantiate_authority("DAO Admin", "ADMIN");

            // Instantiate a Hello component, populating its vault with our supply of 1000 HelloToken
            let component = Self {
                treasury: Vault::new(RADIX_TOKEN),
                dao_token: Vault::with_bucket(dao_bucket),
                admin_badge_addr: admin_badge.resource_address(), //To instantiate radix token
                entry_fee,
                total_members: Decimal::ZERO,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize();

            (component, admin_badge)
        }

        // This is a method, because it needs a reference to self.  Methods can only be called on components
        pub fn get_members(&mut self) -> Decimal {
            info!("Total Members in DAO: {}", self.total_members);
            self.total_members
        }

        pub fn join(&mut self) {}

        // pub fn withdraw(&mut self, amount: Decimal) -> Bucket {
        //     info!("Withdrawing Funds...");
        //     self.treasury.take(amount)
        // }

        // pub fn change_price(&mut self, new_price: Decimal) {
        //     self.entry_fee = new_price
        // }
    }
}
