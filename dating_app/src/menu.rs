
use uuid::Uuid;
use crate::service::DatingService;
use crate::input::Input;
use crate::profile::Profile;

pub struct Menu;


impl Menu {

    pub fn show_authenticated_menu(service : &mut DatingService,current_user :&mut Option<Uuid>){

        let userid = current_user.unwrap();
        let user = service.get_user(userid).unwrap();

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
            "1" => println!("1"),
            "2" => println!("1"),
            "3" => println!("1"),
            "4" => println!("1"),
            "5" => println!("1"),
            "6" => println!("1"),
            "7" => println!("1"),
            "8" => println!("1"),
            "9" => println!("1"),
            "10" => println!("1"),
            _ => println!("Invalid opion. Try again"),
        }

    }
    pub fn show_unauthenticated_menu(service : &mut DatingService,current_user :&mut Option<Uuid>){
        println!("\nMain Menu");
        println!("1 Login ");
        println!("2 Create Profle ");
        println!("3 Exit ");
        println!("Choose an option ");

        let data = Input::read();    

        match data.as_str() {
            "1" => println!("Login"),
            "2" => println!("Create profile"),
            "3" => {
                println!("Thank you for using dating App.");
                std::process::exit(0);
            } ,
            _ => println!("Invalid option. Try again."),
        }
    }
    
}