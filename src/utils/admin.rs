
pub struct Admin{
    pub name: String,
    pub email: String,
    pub password: String,
    pub active_session: bool 
}

impl Admin{

    pub fn set_name(&mut self,name:String){
        self.name = name;
    }
    pub fn set_email(&mut self,email:String){
        self.email = email;
    }
    pub fn set_password(&mut self,password:String){
        self.password = password;
    }
    pub fn toggle_session(&mut self){
        self.active_session = !self.active_session
    }

    pub fn verify_cred(&self,username: &String,password: &String) -> bool{
        if self.name==*username && self.password==*password{
            return true;
        }
        
        return false;

    }

}

