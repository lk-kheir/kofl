pub mod Config {

    use crate::utils::Utils::{check_existing_config, get_home_dir};
    use serde::{Deserialize, Serialize};
    use std::env;
    use std::fmt::Debug;
    use std::fs;
    use std::path::PathBuf;
    use toml;

    #[derive(Serialize, Deserialize)]
    pub struct KoflGlobalConfig {
        config_path: PathBuf,
        data_storage_path: PathBuf,
        user_id: String,
        username: String,
        salt: String,
        hashed_pwd: String,
        master_key_provided: bool,
    }

    impl KoflGlobalConfig {
        pub fn new() -> KoflGlobalConfig {
            let home_dir = get_home_dir().expect("Home directory not found");
            let key = "USER";
            KoflGlobalConfig {
                config_path: home_dir.join(".kofl"), // Example using the home directory
                data_storage_path: home_dir.join("kofl.sqlite"),
                user_id: String::from("1234567"), // dummy change later with random num generator,
                username: match env::var(key) {
                    Ok(val) => val,
                    Err(_) => String::from("user_12234"),
                },
                salt: String::from(""),
                hashed_pwd: String::from(""),
                master_key_provided: false,
            }
        }

        pub fn get_data_storage_path<'a>(&'a self) -> &'a PathBuf {
            &self.data_storage_path
        }

        pub fn get_config_path<'a>(&'a self) -> &'a PathBuf {
            &self.config_path
        }

        pub fn set_salt(&mut self, salt_val: String) {
            self.salt = salt_val.clone();
        }

        pub fn get_salt(&self) -> String{
            self.salt.clone()
        }

        pub fn get_user_login(&self) -> String {
            self.username.clone()
        }

        pub fn set_master_key_hash(&mut self, hash_val: String) {
            self.hashed_pwd = hash_val.clone();
        }
        pub fn get_hashed_pwd(&self) -> String {
            self.hashed_pwd.clone()
        }

        pub fn set_master_key_provided(&mut self, is_set: bool) {
            self.master_key_provided = true;
        }
        pub fn is_master_key_provided(&self) -> bool {
            self.master_key_provided
        }

        pub fn load(&mut self) {
            if check_existing_config() {
                match self.read_config_from_toml_file() {
                    Ok(config) => {
                        *self = config;
                    }
                    Err(e) => {
                        println!("Failed to load config: {}", e);
                        // Handle error, e.g., use default values or exit
                    }
                }
            } else {
                println!("config file does not exists");
                self.write_config_to_toml_file();
            }
        }

        pub fn update(&self) {
            self.write_config_to_toml_file();
        }

        fn serialize_to_toml(&self) -> String {
            toml::to_string(self).expect("could not serialize struct into toml string")
        }

        fn write_config_to_toml_file(&self) {
            let toml_str = self.serialize_to_toml();
            println!("toml str =\n{}", toml_str);
            let config_pth = &self.config_path;
            fs::write(config_pth, toml_str).expect("could not create toml file for config");
        }

        fn read_config_from_toml_file(
            &self,
        ) -> Result<KoflGlobalConfig, Box<dyn std::error::Error>> {
            let config_pth = &self.config_path;
            let toml_str = fs::read_to_string(config_pth)?;
            let config: KoflGlobalConfig = toml::from_str(&toml_str)?;
            Ok(config)
        }
    
    
    
    }

    impl Debug for KoflGlobalConfig {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "\nKofl Global Configuration:\n\
             ├─ User Info:\n\
             │  ├─ Username: {}\n\
             │  └─ User ID: {}\n\
             ├─ Paths:\n\
             │  ├─ Config: {}\n\
             │  └─ Storage: {}\n\
             ├─ Security:\n\
             │  ├─ Master Key Set: {}\n\
             │  ├─ Salt Present: {}\n\
             │  └─ Password Hash: {}\n\
             └─ Status: {}\n",
                self.username,
                self.user_id,
                self.config_path.display(),
                self.data_storage_path.display(),
                if self.master_key_provided {
                    "Yes"
                } else {
                    "No"
                },
                if !self.salt.is_empty() { "Yes" } else { "No" },
                if !self.hashed_pwd.is_empty() {
                    "Set"
                } else {
                    "Not Set"
                },
                if self.master_key_provided {
                    "Configured"
                } else {
                    "Needs Setup"
                }
            )
        }
    }
}
