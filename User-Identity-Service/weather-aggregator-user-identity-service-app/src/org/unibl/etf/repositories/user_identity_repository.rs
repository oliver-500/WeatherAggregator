

#[derive(Debug)]
pub struct UserIdentityRepository{

}

impl UserIdentityRepository {
    pub fn new() -> Self {
        Self {}
    }



}

impl Default for UserIdentityRepository {
    fn default() -> Self {
        Self::new()
    }
}