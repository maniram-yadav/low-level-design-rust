
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
        println!("10. Logout");
        print!("Choose an option: ");
        let choice = Input::read();

        match choice.as_str() { 
            "1" => profile.add_interests(current_user.unwrap()),
            "2" => println!("2"),
            "3" => println!("3"),
            "4" => println!("4"),
            "5" => println!("5"),
            "6" => println!("6"),
            "7" => println!("7"),
            "8" => println!("8"),
            "9" => println!("9"),
            "10" =>{
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