/*
ABOUT THIS CONTRACT...
This contract offers a way for users to set up a universal profile for any one
account key. They can edit their profile data anytime. This profile data
might be displayed in apps like Geode Social, Marketplace, Life & Work, etc.
*/

#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod geode_profile {

    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;
    use ink::storage::Mapping;


    // PRELIMINARY DATA STRUCTURES >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(
            ink::storage::traits::StorageLayout, 
            scale_info::TypeInfo,
            Debug,
            PartialEq,
            Eq
        )
    )]
    pub struct Profile {
        account: AccountId,
        name: Vec<u8>,
        location: Vec<u8>,
        bio: Vec<u8>,
        photo_url: Vec<u8>,
        website_url1: Vec<u8>,
        website_url2: Vec<u8>,
        website_url3: Vec<u8>,
        make_private: bool, 
    }
    
    impl Default for Profile {
        fn default() -> Profile {
            let default_addy = "000000000000000000000000000000000000000000000000";
            let default_addy_id32: AccountId = default_addy.as_bytes().try_into().unwrap();
            Profile {
                account: default_addy_id32,
                name: <Vec<u8>>::default(),
                location: <Vec<u8>>::default(),
                bio: <Vec<u8>>::default(),
                photo_url: <Vec<u8>>::default(),
                website_url1: <Vec<u8>>::default(),
                website_url2: <Vec<u8>>::default(),
                website_url3: <Vec<u8>>::default(),
                make_private: false,
            }
        }
    }


    // EVENT DEFINITIONS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
    // no events will be written to the chain. Profiles are fully editable
    // and therefore stored in the contract only.


    // ERROR DEFINITIONS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    // Errors that can occur upon calling this contract
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        // no errors here to show
    }

    // ACTUAL CONTRACT STORAGE >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
    #[ink(storage)]
    pub struct ContractStorage {
        profile_map: Mapping<AccountId, Profile>,
        profile_accounts: Vec<AccountId>
    }

    // CONTRACT LOGIC >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    impl ContractStorage {
        
        // CONSTRUCTORS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        // Constructors are implicitly payable when the contract is instantiated.

        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                profile_map: Mapping::default(),
                profile_accounts: <Vec<AccountId>>::default()
            }
        }

        // MESSGE FUNCTIONS THAT ALTER CONTRACT STORAGE >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        
        // Set or update your profile (restricted to account owner)
        #[ink(message)]
        pub fn update_your_profile(&mut self, 
            display_name: Vec<u8>,
            location: Vec<u8>,
            bio: Vec<u8>,
            photo_url: Vec<u8>,
            website_url1: Vec<u8>,
            website_url2: Vec<u8>,
            website_url3: Vec<u8>,
            hide_your_profile_from_search: bool,
        ) -> Result<(), Error> {

            let caller = Self::env().caller();
            let updated_profile = Profile {
                account: caller,
                name: display_name,
                location: location,
                bio: bio,
                photo_url: photo_url,
                website_url1: website_url1,
                website_url2: website_url2,
                website_url3: website_url3,
                make_private: hide_your_profile_from_search,
            };
            // update the storage map
            self.profile_map.insert(&caller, &updated_profile);
            
            Ok(())
        }


        // MESSAGE FUNCTIONS THAT RETRIEVE DATA FROM STORAGE  >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

        // GET THE PROFILE FOR THE GIVEN ACCOUNT
        #[ink(message)]
        pub fn get_account_profile(&self, account: AccountId) -> Profile {
            let profile = self.profile_map.get(&account).unwrap_or_default();
            profile
        }

        // SEARH PROFILES BY KEYWORD 
        // Returns all the profiles that match the keyword inputs
        #[ink(message)]
        pub fn get_matching_profiles(&self, keywords: Vec<u8>) -> Vec<Profile> {
            // set up your results vector
            let mut matching_profiles: Vec<Profile> = Vec::new();

            // iterate over the profile accounts vector to find profiles that match
            for account in self.profile_accounts.iter() {

                // make strings for each profile element that might be keyword worthy
                // bio, location and geode_apps_username
                let bio_vecu8 = self.profile_map.get(&account).unwrap_or_default().bio;
                let bio_string = String::from_utf8(bio_vecu8).unwrap_or_default();
                let location_vecu8 = self.profile_map.get(&account).unwrap_or_default().location;
                let location_string = String::from_utf8(location_vecu8).unwrap_or_default();
                let name_vecu8 = self.profile_map.get(&account).unwrap_or_default().name;
                let name_string = String::from_utf8(name_vecu8).unwrap_or_default();
                
                // make a string of the keywords we are searching for
                let keywords_vecu8 = keywords.clone();
                let search_string = String::from_utf8(keywords_vecu8).unwrap_or_default(); 

                // if we get a match to any of those profile elements, add the profile to the results
                if bio_string.contains(&search_string) || location_string.contains(&search_string) 
                || name_string.contains(&search_string) {
                    // get the Profile for that account
                    let profile_match = self.profile_map.get(&account).unwrap_or_default();
                    // add it to the vector of Profiles we will return
                    matching_profiles.push(profile_match);
                }

            }
            // return the results
            matching_profiles

        }

        // END OF MESSAGE FUNCTIONS

    }
    // END OF CONTRACT LOGIC

}
