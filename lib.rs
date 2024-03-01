/*
ABOUT THIS CONTRACT...
This contract offers a way for users to set up a universal profile for any one
account key. They can edit their profile data anytime. This profile becomes the
central jumping off point for others to find all of your app accounts and activity
across the system. 
*/

#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod geode_profile {

    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;
    use ink::storage::Mapping;
    use ink::storage::StorageVec;


    // PRELIMINARY DATA STRUCTURES >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct Profile {
        account: AccountId,
        display_name: Vec<u8>,
        location: Vec<u8>,
        tags: Vec<u8>,
        bio: Vec<u8>,
        photo_url: Vec<u8>,
        website_url1: Vec<u8>,
        website_url2: Vec<u8>,
        website_url3: Vec<u8>,
        life_and_work: AccountId,
        social: AccountId,
        private_messaging: AccountId,
        marketplace: AccountId,
        more_info: Vec<u8>,
        make_private: bool, 
    }
    
    impl Default for Profile {
        fn default() -> Profile {
            let default_addy = "000000000000000000000000000000000000000000000000";
            let default_addy_id32: AccountId = default_addy.as_bytes().try_into().unwrap();
            Profile {
                account: default_addy_id32,
                display_name: <Vec<u8>>::default(),
                location: <Vec<u8>>::default(),
                tags: <Vec<u8>>::default(),
                bio: <Vec<u8>>::default(),
                photo_url: <Vec<u8>>::default(),
                website_url1: <Vec<u8>>::default(),
                website_url2: <Vec<u8>>::default(),
                website_url3: <Vec<u8>>::default(),
                life_and_work: default_addy_id32,
                social: default_addy_id32,
                private_messaging: default_addy_id32,
                marketplace: default_addy_id32,
                more_info: <Vec<u8>>::default(),
                make_private: false,
            }
        }
    }


    // EVENT DEFINITIONS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
    // event that someone has updated a profile
    #[ink(event)]
    pub struct ProfileUpdate {
        #[ink(topic)]
        account: AccountId,
        location: Vec<u8>,
        tags: Vec<u8>,
    }

    // event that someone has filled out a new profile
    #[ink(event)]
    pub struct NewProfile {
        #[ink(topic)]
        account: AccountId,
        location: Vec<u8>,
        tags: Vec<u8>,
    }


    // ERROR DEFINITIONS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    // Errors that can occur upon calling this contract
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        // profile is too large to store
        ProfileTooLarge,
        // attempting to update a profile that does not exist
        NonexistentProfile,
        // usign the new profile message to make a duplicate profile
        DuplicateProfile,
    }

    // ACTUAL CONTRACT STORAGE >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
    #[ink(storage)]
    pub struct ContractStorage {
        profile_map: Mapping<AccountId, Profile>,
        profile_accounts: StorageVec<AccountId>
    }

    // CONTRACT LOGIC >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    impl ContractStorage {
        
        // CONSTRUCTORS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        // Constructors are implicitly payable when the contract is instantiated.

        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                profile_map: Mapping::default(),
                profile_accounts: StorageVec::default()
            }
        }

        // MESSAGES THAT ALTER CONTRACT STORAGE >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

        // 0 🟢 Make a NEW profile (restricted to account owner)
        #[ink(message)]
        pub fn new_profile(&mut self, 
            preferred_display_name: Vec<u8>,
            my_location: Vec<u8>,
            expertise_and_offerings_tags: Vec<u8>,
            my_bio: Vec<u8>,
            photo_link: Vec<u8>,
            website1: Vec<u8>,
            website2: Vec<u8>,
            website3: Vec<u8>,
            life_and_work_account: AccountId,
            social_account: AccountId,
            private_messaging_account: AccountId,
            marketplace_seller_account: AccountId,
            any_extra_info: Vec<u8>,
            hide_your_profile_from_search: bool,
        ) -> Result<(), Error> {

            let caller = Self::env().caller();
            // if the caller already has a profile, send an error
            if self.profile_map.contains(&caller) {
                return Err(Error::DuplicateProfile);
            }
            else {            
                // create the Profile structure
                let updated_profile = Profile {
                    account: caller,
                    display_name: preferred_display_name,
                    location: my_location.clone(),
                    tags: expertise_and_offerings_tags.clone(),
                    bio: my_bio,
                    photo_url: photo_link,
                    website_url1: website1,
                    website_url2: website2,
                    website_url3: website3,
                    life_and_work: life_and_work_account,
                    social: social_account,
                    private_messaging: private_messaging_account,
                    marketplace: marketplace_seller_account,
                    more_info: any_extra_info,
                    make_private: hide_your_profile_from_search,
                };
                
                //add the account to the storage vector of all accounts
                self.profile_accounts.push(&caller);
                // update the profile map in storage
                if self.profile_map.try_insert(caller, &updated_profile).is_err() {
                    return Err(Error::ProfileTooLarge);
                }

                // emit an event to the chain
                self.env().emit_event(NewProfile {
                    account: caller,
                    location: my_location,
                    tags: expertise_and_offerings_tags,
                });
            }
            Ok(())
        }

        // 1 🟢 UPDATE your profile (restricted to account owner)
        #[ink(message)]
        pub fn update_your_profile(&mut self, 
            preferred_display_name: Vec<u8>,
            my_location: Vec<u8>,
            expertise_and_offerings_tags: Vec<u8>,
            my_bio: Vec<u8>,
            photo_link: Vec<u8>,
            website1: Vec<u8>,
            website2: Vec<u8>,
            website3: Vec<u8>,
            life_and_work_account: AccountId,
            social_account: AccountId,
            private_messaging_account: AccountId,
            marketplace_seller_account: AccountId,
            any_extra_info: Vec<u8>,
            hide_your_profile_from_search: bool,
        ) -> Result<(), Error> {

            let caller = Self::env().caller();
            // check to see if this profile exists
            if self.profile_map.contains(&caller) {
                // create the Profile structure
                let updated_profile = Profile {
                    account: caller,
                    display_name: preferred_display_name,
                    location: my_location.clone(),
                    tags: expertise_and_offerings_tags.clone(),
                    bio: my_bio,
                    photo_url: photo_link,
                    website_url1: website1,
                    website_url2: website2,
                    website_url3: website3,
                    life_and_work: life_and_work_account,
                    social: social_account,
                    private_messaging: private_messaging_account,
                    marketplace: marketplace_seller_account,
                    more_info: any_extra_info,
                    make_private: hide_your_profile_from_search,
                };
                
                if self.profile_map.try_insert(caller, &updated_profile).is_err() {
                    return Err(Error::ProfileTooLarge);
                }

                // emit an event to the chain
                self.env().emit_event(ProfileUpdate {
                    account: caller,
                    location: my_location,
                    tags: expertise_and_offerings_tags,
                });

            }
            else {
                // send an error that the profile does not exist
                return Err(Error::NonexistentProfile);
            }

            Ok(())
        }


        // MESSAGE FUNCTIONS THAT RETRIEVE DATA FROM STORAGE  >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

        // 2 🟢 GET THE PROFILE FOR THE GIVEN ACCOUNT
        #[ink(message)]
        pub fn get_account_profile(&self, account: AccountId) -> Vec<Profile> {
            let mut results = Vec::new();
            if self.profile_map.contains(&account) {
                let this_profile = self.profile_map.get(&account).unwrap_or_default();
                results.push(this_profile);
            }
            results
        }

        // 3 🟢 SEARH PROFILES BY KEYWORD 
        // Returns all the profiles that match the keyword inputs
        #[ink(message)]
        pub fn get_matching_profiles_by_keyword(&self, keywords: Vec<u8>) -> Vec<Profile> {
            // set up your results vector
            let mut matching_profiles: Vec<Profile> = Vec::new();

            // iterate over the profile accounts vector to find profiles that match
            // get the length of the profile_accounts storagevec and loop over the index
            if self.profile_accounts.len() > 0 {
                for i in 0..self.profile_accounts.len() {
                    // get the profile for the account
                    let account = self.profile_accounts.get(i).unwrap();
                    let profile_match = self.profile_map.get(&account).unwrap_or_default();
                    // check to see if the account profile is private, if so move on
                    if !profile_match.make_private {
                        // make strings for each profile element that might be keyword worthy
                        let bio_vecu8 = profile_match.bio.clone();
                        let bio_string = String::from_utf8(bio_vecu8).unwrap_or_default();
                        let location_vecu8 = profile_match.location.clone();
                        let location_string = String::from_utf8(location_vecu8).unwrap_or_default();
                        let name_vecu8 = profile_match.display_name.clone();
                        let name_string = String::from_utf8(name_vecu8).unwrap_or_default();
                        let info_vecu8 = profile_match.more_info.clone();
                        let info_string = String::from_utf8(info_vecu8).unwrap_or_default();
                        let tag_vecu8 = profile_match.tags.clone();
                        let tag_string = String::from_utf8(tag_vecu8).unwrap_or_default();
                    
                        // make a string of the keywords we are searching for
                        let keywords_vecu8 = keywords.clone();
                        let search_string = String::from_utf8(keywords_vecu8).unwrap_or_default(); 

                        // if we get a match to any of those profile elements, add the profile to the results
                        if bio_string.contains(&search_string) || location_string.contains(&search_string) 
                        || name_string.contains(&search_string) || info_string.contains(&search_string) 
                        || tag_string.contains(&search_string) {
                            // add it to the vector of Profiles we will return
                            matching_profiles.push(profile_match);
                        }
                        
                    }
                    // continue iterating
                }
            }
            // return the results
            matching_profiles

        }


        // 4 🟢 SEARH PROFILES BY ACCOUNTID 
        // Returns all the profiles that include the given AccountId in any such field
        #[ink(message)]
        pub fn get_matching_profiles_by_account(&self, search_account_id: AccountId) -> Vec<Profile> {
            // set up your results vector
            let mut matching_profiles: Vec<Profile> = Vec::new();
            // iterate over the profile accounts vector to find profiles that match
            if self.profile_accounts.len() > 0 {
                for i in 0..self.profile_accounts.len() {
                    // get the profile for the account
                    let account = self.profile_accounts.get(i).unwrap();
                    let profile_match = self.profile_map.get(&account).unwrap_or_default();
                    // check to see if the account profile is private, if so move on
                    if !profile_match.make_private {
                        // for each account, get the profile fields that might have that AccountID in it
                        let acct_vec = Vec::from([profile_match.life_and_work, profile_match.social, 
                        profile_match.private_messaging, profile_match.marketplace, profile_match.account]);
                    
                        // if the search_account_id is any one of those, add the profile to the results vector
                        if acct_vec.contains(&search_account_id) {
                            // add it to the vector of Profiles we will return
                            matching_profiles.push(profile_match);
                        }
                    }
                    // finish iterating
                }
            }
            // return the results
            matching_profiles

        }

        // 5 🟢 Get all the accounts that have set their profile at least once
        #[ink(message)]
        pub fn get_accounts_with_profiles(&self) -> Vec<AccountId> {
            // set up return structure
            let mut accounts_list: Vec<AccountId> = Vec::new();
            // get all the accounts
            if self.profile_accounts.len() > 0 {
                for i in 0..self.profile_accounts.len() {
                    // get the account
                    let account = self.profile_accounts.get(i).unwrap();
                    accounts_list.push(account);
                }
            }    
            // return results
            accounts_list
        }

        // 6 🟢 Verify that this account has set their profile
        #[ink(message)]
        pub fn verify_account(&self, verify_account_id: AccountId) -> u8 {
            // set up return structure
            let mut result: u8 = 0;
            // check the map
            if self.profile_map.contains(&verify_account_id) {
               result = 1;
            }    
            // return results
            result
        }

        // END OF MESSAGE FUNCTIONS

    }
    // END OF CONTRACT LOGIC

}
