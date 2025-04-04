use super::Gender;

impl std::fmt::Display for Gender {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        match self {
            Gender::Male => write!(f,"Male"),
            Gender::Female => write!(f,"Female"),
            Gender::Other => write!(f,"Other"),
            Gender::Any => write!(f,"Any"),
        }
    }


}