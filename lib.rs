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
        make_private: bool, 
        timestamp: u64,
    }
    
    impl Default for Profile {
        fn default() -> Profile {
            Profile {
                account: AccountId::from([0x0; 32]),
                display_name: <Vec<u8>>::default(),
                location: <Vec<u8>>::default(),
                tags: <Vec<u8>>::default(),
                bio: <Vec<u8>>::default(),
                photo_url: <Vec<u8>>::default(),
                website_url1: <Vec<u8>>::default(),
                website_url2: <Vec<u8>>::default(),
                website_url3: <Vec<u8>>::default(),
                life_and_work: AccountId::from([0x0; 32]),
                social: AccountId::from([0x0; 32]),
                private_messaging: AccountId::from([0x0; 32]),
                marketplace: AccountId::from([0x0; 32]),
                make_private: false,
                timestamp: u64:: default(),
            }
        }
    }


    // EVENT DEFINITIONS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
    // event that someone has updated a profile
    #[ink(event)]
    pub struct ProfileUpdate {
        #[ink(topic)]
        account: AccountId,
        #[ink(topic)]
        display_name: Vec<u8>,
        #[ink(topic)]
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
    }

    // event that someone has filled out a new profile
    #[ink(event)]
    pub struct NewProfile {
        #[ink(topic)]
        account: AccountId,
        #[ink(topic)]
        display_name: Vec<u8>,
        #[ink(topic)]
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
        CannotUpdate,
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
            hide_your_profile_from_search: bool,
        ) -> Result<(), Error> {

            let caller = Self::env().caller();
            // if the caller already has a profile OR data is too big, send an error
            if self.profile_map.contains(&caller) || preferred_display_name.len() > 180
            || my_location.len() > 200 || expertise_and_offerings_tags.len() > 600 
            || my_bio.len() > 1200 || photo_link.len() > 300 || website1.len() > 100
            || website2.len() > 100 || website3.len() > 100 {
                return Err(Error::CannotUpdate);
            }
            else {            
                // create the Profile structure
                let updated_profile = Profile {
                    account: caller,
                    display_name: preferred_display_name.clone(),
                    location: my_location.clone(),
                    tags: expertise_and_offerings_tags.clone(),
                    bio: my_bio.clone(),
                    photo_url: photo_link.clone(),
                    website_url1: website1.clone(),
                    website_url2: website2.clone(),
                    website_url3: website3.clone(),
                    life_and_work: life_and_work_account,
                    social: social_account,
                    private_messaging: private_messaging_account,
                    marketplace: marketplace_seller_account,
                    make_private: hide_your_profile_from_search,
                    timestamp: self.env().block_timestamp(),
                };
                
                // update profile_accounts StorageVec
                self.profile_accounts.push(&caller);
                
                // update the profile map in storage
                if self.profile_map.try_insert(caller, &updated_profile).is_err() {
                    return Err(Error::ProfileTooLarge);
                }

                // emit an event to the chain IF the profile is not hidden
                if hide_your_profile_from_search == false {
                    self.env().emit_event(NewProfile {
                        account: caller,
                        display_name: preferred_display_name,
                        location: my_location,
                        tags: expertise_and_offerings_tags,
                        bio: my_bio,
                        photo_url: photo_link,
                        website_url1: website1,
                        website_url2: website2,
                        website_url3: website3,
                        life_and_work: life_and_work_account,
                        social: social_account,
                        private_messaging: private_messaging_account,
                        marketplace: marketplace_seller_account, 
                    });
                }
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
            hide_your_profile_from_search: bool,
        ) -> Result<(), Error> {
            // If the data is too large, send an error first
            if preferred_display_name.len() > 180
            || my_location.len() > 200 || expertise_and_offerings_tags.len() > 600 
            || my_bio.len() > 1200 || photo_link.len() > 300 || website1.len() > 100
            || website2.len() > 100 || website3.len() > 100 {
                return Err(Error::CannotUpdate);
            }
            else {
                let caller = Self::env().caller();
                // check to see if this profile exists
                if self.profile_map.contains(&caller) {
                    // create the Profile structure
                    let updated_profile = Profile {
                        account: caller,
                        display_name: preferred_display_name.clone(),
                        location: my_location.clone(),
                        tags: expertise_and_offerings_tags.clone(),
                        bio: my_bio.clone(),
                        photo_url: photo_link.clone(),
                        website_url1: website1.clone(),
                        website_url2: website2.clone(),
                        website_url3: website3.clone(),
                        life_and_work: life_and_work_account,
                        social: social_account,
                        private_messaging: private_messaging_account,
                        marketplace: marketplace_seller_account,
                        make_private: hide_your_profile_from_search,
                        timestamp: self.env().block_timestamp(),
                    };
                    
                    // add the account to the storage vector of all accounts
                    self.profile_accounts.push(&caller);

                    // update the profile_map
                    if self.profile_map.try_insert(caller, &updated_profile).is_err() {
                        return Err(Error::ProfileTooLarge);
                    }

                    // emit an event to the chain IF the profile is not hidden
                    if hide_your_profile_from_search == false {
                        self.env().emit_event(ProfileUpdate {
                            account: caller,
                            display_name: preferred_display_name,
                            location: my_location,
                            tags: expertise_and_offerings_tags,
                            bio: my_bio,
                            photo_url: photo_link,
                            website_url1: website1,
                            website_url2: website2,
                            website_url3: website3,
                            life_and_work: life_and_work_account,
                            social: social_account,
                            private_messaging: private_messaging_account,
                            marketplace: marketplace_seller_account, 
                        });
                    }

                }
                else {
                    // send an error that the profile does not exist
                    return Err(Error::NonexistentProfile);
                }
            }

            Ok(())
        }


        // MESSAGE FUNCTIONS THAT RETRIEVE DATA FROM STORAGE  >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

        // 2 🟢 GET THE PROFILE FOR A GIVEN ACCOUNT
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
        pub fn get_matching_profiles_by_keyword(&self, 
            keywords1: Vec<u8>, 
            keywords2: Vec<u8>, 
            keywords3: Vec<u8>) -> Vec<Profile> {
            // set up your results vector
            let mut matching_profiles: Vec<Profile> = Vec::new();
            // make a string of the keywords we are searching for
            let search_string1 = String::from_utf8(keywords1).unwrap_or_default(); 
            let search_string2 = String::from_utf8(keywords2).unwrap_or_default();
            let search_string3 = String::from_utf8(keywords3).unwrap_or_default();

            // iterate over the profile accounts vector to find profiles that match
            if self.profile_accounts.len() > 0 {
                for i in 0..self.profile_accounts.len() {
                    // get the profile for the account
                    let account = self.profile_accounts.get(i).unwrap();
                    // get the profile for the account
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
                        let tag_vecu8 = profile_match.tags.clone();
                        let tag_string = String::from_utf8(tag_vecu8).unwrap_or_default();
                    
                        // if we get a match to any of those profile elements, add the profile to the results
                        // must match on ALL keywords
                        if (bio_string.contains(&search_string1) || location_string.contains(&search_string1) 
                        || name_string.contains(&search_string1) || tag_string.contains(&search_string1))
                        && (bio_string.contains(&search_string2) || location_string.contains(&search_string2) 
                        || name_string.contains(&search_string2) || tag_string.contains(&search_string2))
                        && (bio_string.contains(&search_string3) || location_string.contains(&search_string3) 
                        || name_string.contains(&search_string3) || tag_string.contains(&search_string3)) {
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
                    // get the profile for the account
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

        // 5 🟢 Verify that this account has set their profile
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
