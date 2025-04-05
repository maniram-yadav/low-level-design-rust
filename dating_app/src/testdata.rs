use crate::service::DatingService;

pub struct TestData<'a> {
    service : &'a mut DatingService
}

impl<'a > TestData<'a > {
    pub fn initialize_test_data(service : &'a mut DatingService ){
        println!("Initializing Data");
    }

}