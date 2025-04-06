
use uuid::Uuid;
use crate::service::DatingService;
use crate::input::Input;
use crate::profile::Profile;
use crate::login::Login;

pub struct Menu;


impl Menu {

    pub fn show_authenticated_menu(profile : &mut Profile<'_>,current_user :&mut Option<Uuid>){

        let userid = current_user.unwrap();
        let user = profile.service.get_user(userid).unwrap();

        println!("\nWelcome, {}!", user.name);
        println!("1. Add Interests");
        println!("2. Set Partner Preferences");
        println!("3. Get Best Profile");
        println!("4. Accept Profile");
        println!("5. Decline Profile");
        println!("6. List Matched Profiles");
        println!("7. Buy Boost");
        println!("8. Show Stats (Admin Only)");
        println!("9. Super Accept Profile");
        println!("10. List All users");
        println!("11. Logout");
        print!("Choose an option: ");

        let choice = Input::read();
        let user_id = current_user.unwrap();
        match choice.as_str() { 
            "1" => profile.add_interests(current_user.unwrap()),
            "2" => profile.set_partner_preferences(user_id),
            "3" => profile.get_best_profile(user_id),
            "4" => profile.accept_profile(user_id),
            "5" => profile.decline_profile(user_id),
            "6" => profile.list_matched_profiles(user_id),
            "7" => profile.buy_boost(user_id),
            "8" => profile.show_stats(user_id),
            "9" => profile.super_accept_profile(user_id),
            "10" => profile.list_all_users(user_id),
            "11" =>{
                *current_user = None;
                println!("Logged out successfully");
            },
            _ => println!("Invalid opion. Try again"),
        }

    }
    pub fn show_unauthenticated_menu(profile : &mut Profile<'_>,current_user :&mut Option<Uuid>){
        
        println!("\nMain Menu");
        println!("1 Login ");
        println!("2 Create Profle ");
        println!("3 Exit ");
        println!("Choose an option ");

        let data = Input::read();    

        match data.as_str() {
            "1" => Login::login(profile.service,current_user),
            "2" => profile.create_profile(current_user),
            "3" => {
                println!("Thank you for using dating App.");
                std::process::exit(0);
            } ,
            _ => println!("Invalid option. Try again."),
        }
    }
    
}