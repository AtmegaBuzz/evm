

pub struct Admin{
    pub name: &str,
    pub email: &str,
    pub password: &str,
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


}
