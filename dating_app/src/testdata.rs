use crate::service::DatingService;
use crate::service::Gender;
use crate::service::User;
use crate::service::PartnerPreferences;

pub struct TestData;

impl TestData {
    pub fn initialize_test_data(service : &mut DatingService ){
        println!("Initializing Data");
        
        let mut alice = User::new("Alice".to_string(),25,Gender::Male);
        alice.interests.extend(vec!["Pets".to_string(), "Movies".to_string(), "Travel".to_string()]);
        alice.partner_prefs = Some(PartnerPreferences::new(23,30,Gender::Male,false));

        let mut bob = User::new("Bob".to_string(), 28, Gender::Male);
        bob.interests.extend(vec!["Football".to_string(), "Movies".to_string(), "Gaming".to_string()]);
        bob.partner_prefs = Some(PartnerPreferences::new(22, 27, Gender::Female, false));

        let mut charlie = User::new("Charlie".to_string(), 30, Gender::Male);
        charlie.interests.extend(vec!["Books".to_string(), "Hiking".to_string(), "Photography".to_string()]);
        charlie.partner_prefs = Some(PartnerPreferences::new(25, 35, Gender::Female, false));

        let mut diana = User::new("Diana".to_string(), 22, Gender::Female);
        diana.interests.extend(vec!["Cooking".to_string(), "Travel".to_string(), "Music".to_string()]);
        diana.partner_prefs = Some(PartnerPreferences::new(25, 32, Gender::Male, false));

        let mut eve = User::new("Eve".to_string(), 27, Gender::Female);
        eve.interests.extend(vec!["Pets".to_string(), "Books".to_string(), "Photography".to_string()]);
        eve.partner_prefs = Some(PartnerPreferences::new(25, 35, Gender::Male, false));

        let mut admin = User::new("Admin".to_string(), 35, Gender::Other);
        admin.interests.extend(vec![]);
        admin.partner_prefs = None;
        admin.is_admin = true;

        service.add_user(alice);
        service.add_user(bob);
        service.add_user(charlie);
        service.add_user(diana);
        service.add_user(eve);
        service.add_user(admin);
    }

}