pub mod cli {
use std::fmt;
use crate::errors::{ErrorExecution, ErrorValidation};
use crate::context::Context;
use crate::db::Db::Entry;
use chrono::prelude::*;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[warn(unused_variables)]
#[warn(unused_imports)]
pub trait Command {
    fn validate(&self, context: &Context) -> Result<(), ErrorValidation>;
    fn execute(&self, context: &Context) -> Result<(), ErrorExecution>;
    fn display(&self);
}

// maybe ent_name and ent_pass is much better

pub struct AddCmd {
    name: String,
    password: String,
}


impl  AddCmd {
    pub fn new(name: String, password: String) -> AddCmd
    {
        AddCmd{name, password}
    }
}

impl PartialEq for AddCmd {
    fn eq(&self, other: &Self) -> bool {
        if (self.name == other.name) && (self.password == other.password) {return true}
        false
    }
}

impl fmt::Debug for AddCmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Add Command")
         .field("name", &self.name)
         .field("password", &self.password)
         .finish()
    }
}

impl Command for AddCmd {
    fn execute(&self, context: &Context) -> Result<(), ErrorExecution>  {

        let new_entry = Entry {
            id: 0 as u32, // wil be ignored by sqlite
            ent_name: self.name.clone(),
            password_hash: self.password.clone(),
            timestamp: Utc::now().to_rfc3339()

        };

        context.db.add_entry(new_entry).unwrap();

        Ok(())   
    }

    fn validate(&self, context: &Context) -> Result<(), ErrorValidation>  {
        if context.kgc.is_master_key_provided() {
            println!("Master key is provided");
        }
        else {
            println!("Master key is not provided");
            return Err(ErrorValidation::UnprovidedMasterKey);
        }
        return Ok(())
    }

    fn display(&self) {
        println!("Add command with name = {}, password = {}", self.name, self.password);
        ()
    }
}




pub struct GetCmd {
    ent_name: String
}


impl GetCmd {
    pub fn new(ent_name: String) -> Self {
        GetCmd{ent_name}
    }
}


impl Command for GetCmd {

    fn execute(&self, context: &Context) -> Result<(), ErrorExecution> {
        let res = match context.db.get_entry_by_name(&self.ent_name) {
            Ok(val) => val,
            Err(err) => match err {
                rusqlite::Error::QueryReturnedNoRows => {
                    println!("No match for the given entry.");
                    return Err(ErrorExecution::NoMatchingEntry);
                },
                _ => {
                    println!("An error occurred: {}", err);
                    return Err(ErrorExecution::Unknown);
                }
            }
        };
    
        println!("{} {}", res.ent_name, res.password_hash);
    
        Ok(())
    }

    fn validate(&self, context: &Context) -> Result<(), ErrorValidation>  {
        if context.kgc.is_master_key_provided() {
            println!("Master key is provided");
        }
        else {
            println!("Master key is not provided");
            return Err(ErrorValidation::UnprovidedMasterKey);
        }
        return Ok(())
    }

    fn display(&self) {
        println!("Get command with entry name = {}", self.ent_name);
        ()
    }
}

pub struct InitCmd {
    // for now is emty 
}

impl InitCmd {
    pub fn new() -> Self {
        InitCmd{}
    }

}

impl Command for InitCmd {
    fn execute(&self, context: &Context) -> Result<(), ErrorExecution>  {
        

        let master_pwd  = rpassword::prompt_password("type a master password ==> ").unwrap();
        let master_pwd_confirmed = rpassword::prompt_password("type the master password again ==> ").unwrap();

        if master_pwd != master_pwd_confirmed {
            return Err(ErrorExecution::PasswordMismatch);
        }

        let salt:String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

        println!("{salt}");
        println!("{master_pwd}");

        println!("kgc = {:?}", context.kgc);
        context.kgc.set_salt(salt);
        println!("kgc = {:?}", context.kgc);



        Ok(())
    }

    fn validate(&self, context: &Context) -> Result<(), ErrorValidation>  {

        // somthing like already provided master key should be handeled;

        // if context.kgc.is_master_key_provided() {
        //     println!("Master key is provided");
        // }
        // else {
        //     println!("Master key is not provided");
        //     return Err(ErrorValidation::UnprovidedMasterKey);
        // }
        return Ok(())
    }

    fn display(&self) {
        println!("Init Command");
        ()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let add_commad: AddCmd = AddCmd::new("facebook".to_string(), "whocares".to_string());
        assert_eq!(add_commad, AddCmd {
            name: "facebook".to_string(),
            password: "whocares".to_string()
        }
        );
    }
}
}