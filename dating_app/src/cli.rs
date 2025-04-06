use crate::menu::Menu;
use crate::profile::Profile;
use crate::service::DatingService;
use uuid::Uuid;
pub struct Cli;

impl Cli {
    pub fn run_cli(service : &mut DatingService) {
        
        let mut current_user:Option<Uuid> = None;
        let mut profile = Profile::new(service);

        loop {
            if current_user.is_none() {
                Menu::show_unauthenticated_menu(&mut profile,&mut current_user);                
            } else {
                Menu::show_authenticated_menu(&mut profile,&mut current_user);                
            }
        }
        
    }
}