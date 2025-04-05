use crate::service::DatingService;
use crate::input::Input;
use crate::service::User;
use uuid::Uuid;
use crate::service::Gender;

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
}