use crate::service::DatingService;
use crate::input::Input;
use crate::service::User;
use uuid::Uuid;
use crate::service::Gender;
use crate::service::PartnerPreferences;

pub struct Profile<'a> {
   pub
    service:&'a mut DatingService
}

impl<'a> Profile<'a>{

    pub fn new(service:&'a mut DatingService) -> Self {
        Self {
            service 
        }
    }

    pub fn create_profile(&mut self ,current_user : &mut Option<Uuid> ){
       
        println!("Create a new profile");
        print!("\nEnter Name : ");
        let name = Input::read();
        print!("\nEnter Age : ");
        let age_str = Input::read();

        let age = match age_str.parse::<u8>() {

            Ok(age) => age,
            Err(_) => {
                println!("Invalid agePlease enter a number between 0 to 255");
                return ;
            }

        };

        println!("\nEnter Gender (male, Female, Other) : ");
        let gender_str = Input::read();

        let gender = match gender_str.to_lowercase().as_str() {
            "male" => Gender::Male,
            "female" => Gender::Female,
            "other" => Gender::Other,
            "any" => Gender::Any,
            _ => {
                println!("Invalid gender. Please enter male, female, other , any ");
                return ;
            }
        };

        let new_user = User::new(name,age,gender);
        self.service.add_user(new_user.clone());
        *current_user = Some(new_user.id);

        println!("Profile created successfully with id {}. Add interests and Preferences",new_user.id);
        
    }

    pub fn add_interests(&mut self ,user_id : Uuid ){ 
        
        println!("\n Available interests : ");
        for (i,interest) in self.service.available_interests.iter().enumerate() {
            println!("{}. {} ",i+1,interest);
        }

        println!("Entr all interests you want to add (seperated by comma ): ");
        let input = Input::read();
        let total_interest = self.service.available_interests.len();

        let interests = self.service.available_interests.clone();

        if let Some(user) = self.service.get_user_mut(user_id) {
            
            for interest in input.split(',') {

                if let Ok(index) = interest.trim().parse::<usize>() {
                    if index > 0 && index < total_interest {
                        
                        user.interests.insert(interests[index-1].clone());
                    } 
                }
            }
            println!("interest updated successfully");
        }

    }

    pub fn set_partner_preferences(&mut self ,user_id : Uuid ){ 
        println!("\nEnter preferences ") ;
        
        print!("Min Age : ");
        let min_age_str = Input::read();
        let min_age = match min_age_str.parse::<u8>() {
            Ok(age) => age,
            Err(_) => {
                println!("Enter valid age");
                return;
            }
        };

        print!("Max Age : ");
        let max_age_str = Input::read();
        let max_age = match max_age_str.parse::<u8>() {
            Ok(age) => age,
            Err(_) => {
                println!("Enter valid age");
                return;
            }
        };

        print!("Gender (male,female,other,any) : ");
        let gender_str = Input::read();
        let gender = match gender_str.to_lowercase().as_str() {
            "male" => Gender::Male,
            "female" => Gender::Female,
            "any" => Gender::Any,
            "other" => Gender::Other,
            _ => {
                println!("Enter valid Gender");
                return;
            }
        };

        if let Some(user) = self.service.get_user_mut(user_id) {
            user.partner_prefs = Some(PartnerPreferences::new( min_age,max_age,gender,false));
            println!("partner Preference set successfully.");
        }
        

    }

    pub fn get_best_profile(&mut self ,user_id : Uuid ){ 

        if let Some(user) = self.service.get_user(user_id) {
        
            if let Some(best_profile) = self.service.find_best_profile(user) {
                println!("\n Best profile for you");
                println!("Name : {} ",best_profile.name);
                println!("Age : {} ",best_profile.age);
                println!("Gender : {} ",best_profile.gender);
                let interests_cloned = best_profile.interests.clone();
                let interests : Vec<String> = interests_cloned.into_iter().collect();

                println!("Interests : {} ",interests.join(", "));
                let mutual_interests = self.service.count_mutual_interests(user,&best_profile);
                println!("Total Mutual Interests : {} ",mutual_interests);

                let is_preferred = self.service.is_preferred_profile(user,&best_profile);
                println!("Matches your preferences : {} ",is_preferred);
            } else {
                println!("No profile available at the moment. Check back later ");
            }
        }

    }

    pub fn accept_profile(&mut self ,user_id : Uuid ){ 

        println!("\nEnter profile id to accept : ")    ;
        let id = Input::read();

        if let Ok(profile_to_accept_id) = Uuid::parse_str(&id) {
            if self.service.accept_profile(user_id,profile_to_accept_id) {
                println!("Profile accepted! You have got match if they also accepted you.");
            } else {
                println!("Profile accepted ");
            }
        } else {
            println!("Invalid UUId format.");
        }
    }


    pub fn decline_profile(&mut self ,user_id : Uuid ){ 
        
    }
    pub fn list_matched_profiles(&mut self ,user_id : Uuid ){ 
        
    }

    pub fn buy_boost(&mut self ,user_id : Uuid ){ 
        
    }

    pub fn show_stats(&mut self ,user_id : Uuid ){ 
        
    }

    pub fn super_accept_profile(&mut self ,user_id : Uuid ){ 
        
    }


}