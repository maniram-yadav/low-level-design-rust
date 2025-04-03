use super::Commit;

pub struct Branch{
    pub name :String,
    pub commits: Vec<Commit>,
}