
use uuid::Uuid;
use crate::service::DatingService;
use crate::input::Input;
use crate::profile::Profile;

pub struct Menu;


impl Menu {
    
    pub fn show_authenticated_menu(service : &mut DatingService,current_user :&mut Option<Uuid>){
        println!("\nWelcome ")
    }
    pub fn show_unauthenticated_menu(service : &mut DatingService,current_user :&mut Option<Uuid>){
        println!("\nmain Menu");
        let data = Input::read();    
    }
    
}