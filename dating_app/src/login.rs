
// mod caretservice;
// mod input;
use uuid::Uuid;
use crate::service::DatingService;
use crate::input::Input;
pub struct Login ;

impl Login {
    pub fn login(datingService : &DatingService, currrent_user : &mut Option<Uuid>){
        println!("Enter your user id : ");
        let userid = Input::read();

        if let Ok(uuid) = Uuid::parse_str(&userid) {
            if datingService.users.contains_key(&uuid) {
                *currrent_user = Some(uuid);
                println!("Login successfull");
            } else {
                println!("User not found. try Again");
            }
        } else {
            println!("Invalid user id format");
        }

    }
}
