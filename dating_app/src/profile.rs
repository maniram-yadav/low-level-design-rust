use crate::service::DatingService;

pub struct Profile<'a> {
    service:&'a mut DatingService
}

impl<'a> Profile<'a>{

    pub fn new(service:&'a mut DatingService) -> Self {
        Self {
            service 
        }
    }
}