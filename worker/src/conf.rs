use std;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json::Result;

const DEFAULT_PATH: &str = "./static/conf.json";

#[derive(Serialize, Deserialize)]
struct Team {
    name: String,
    ip: String,
}

#[derive(Serialize, Deserialize)]
struct Service {
    name: String,
    flag: String,
    port: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    teams: Vec<Team>,
    services: Vec<Service>,
}

impl Config {
    fn read_conf(path: &str) -> Result<Config> {
        let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
        let conf: Config = serde_json::from_str(&contents)?;
        Ok(conf)
    }

    fn write_conf(path: &str, conf: Config) -> std::io::Result<()> {
        let contents = serde_json::to_string(&conf)?;
        let mut file = File::create(path)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    fn get_teams() -> Result<Vec<Team>> {
        let conf: Config = Self::read_conf(DEFAULT_PATH)?;
        Ok(conf.teams)
    }

    fn get_services() -> Result<Vec<Service>> {
        let conf: Config = Self::read_conf(DEFAULT_PATH)?;
        Ok(conf.services)
    }

    pub fn get_team_list() -> Result<Vec<String>> {
        let list = Self::get_teams()?;
        Ok(list
            .iter()
            .map(|team| team.name.clone())
            .collect::<Vec<String>>())
    }

    pub fn get_service_list() -> Result<Vec<String>> {
        let list = Self::get_services()?;
        Ok(list
            .iter()
            .map(|team| team.name.clone())
            .collect::<Vec<String>>())
    }

    pub fn team_name_to_ip(name: String) -> std::result::Result<String, &'static str> {
        let list = match Self::get_teams() {
            Ok(list) => list,
            Err(_) => return Err("JSON parsing error"),
        };
        for team in list.iter() {
            if name == (&team).name {
                return Ok(team.ip.clone());
            }
        }
        Err("Not found")
    }

    pub fn team_ip_to_name(ip: String) -> std::result::Result<String, &'static str> {
        let list = match Self::get_teams() {
            Ok(list) => list,
            Err(_) => return Err("JSON parsing error"),
        };
        for team in list.iter() {
            if ip == (&team).ip {
                return Ok(team.name.clone());
            }
        }
        Err("Not found")
    }

    pub fn service_name_to_flag(name: String) -> std::result::Result<String, &'static str> {
        let list = match Self::get_services() {
            Ok(list) => list,
            Err(_) => return Err("JSON parsing error"),
        };
        for service in list.iter() {
            if name == (&service).name {
                return Ok(service.flag.clone());
            }
        }
        Err("Not found")
    }

    pub fn service_name_to_port(name: String) -> std::result::Result<u32, &'static str> {
        let list = match Self::get_services() {
            Ok(list) => list,
            Err(_) => return Err("JSON parsing error"),
        };
        for service in list.iter() {
            if name == (&service).name {
                return Ok(service.port.clone());
            }
        }
        Err("Not found")
    }

    pub fn service_port_to_name(port: u32) -> std::result::Result<String, &'static str> {
        let list = match Self::get_services() {
            Ok(list) => list,
            Err(_) => return Err("JSON parsing error"),
        };
        for service in list.iter() {
            if port == (&service).port {
                return Ok(service.name.clone());
            }
        }
        Err("Not found")
    }

    pub fn add_team(name: String, ip: String) -> std::result::Result<(), &'static str> {
        let new_team: Team = Team {
            name: name.clone(),
            ip: ip.clone(),
        };
        let mut conf: Config = match Self::read_conf(DEFAULT_PATH) {
            Ok(conf) => conf,
            Err(_) => return Err("JSON parsing error"),
        };
        if let Some(_index) = conf.teams.iter().position(|x| x.name == name) {
            match Self::update_team(name, ip) {
                Ok(()) => {}
                Err(_) => return Err("JSON error"),
            };
            return Ok(());
        }
        conf.teams.push(new_team);
        match Self::write_conf(DEFAULT_PATH, conf) {
            Ok(()) => {}
            Err(_) => return Err("JSON error"),
        }

        Ok(())
    }

    pub fn remove_team(name: String) -> std::result::Result<(), &'static str> {
        let mut conf: Config = match Self::read_conf(DEFAULT_PATH) {
            Ok(conf) => conf,
            Err(_) => return Err("JSON parsing error"),
        };
        conf.teams.retain(|x| x.name != name);
        match Self::write_conf(DEFAULT_PATH, conf) {
            Ok(()) => {}
            Err(_) => return Err("JSON error"),
        }
        Ok(())
    }

    pub fn update_team(name: String, ip: String) -> std::result::Result<(), &'static str> {
        let mut conf: Config = match Self::read_conf(DEFAULT_PATH) {
            Ok(conf) => conf,
            Err(_) => return Err("JSON parsing error"),
        };
        if let Some(index) = conf.teams.iter().position(|x| x.name == name) {
            conf.teams[index] = Team { name, ip };
        }
        match Self::write_conf(DEFAULT_PATH, conf) {
            Ok(()) => {}
            Err(_) => return Err("JSON error"),
        }
        Ok(())
    }

    pub fn add_service(
        name: String,
        flag: String,
        port: u32,
    ) -> std::result::Result<(), &'static str> {
        let new_service: Service = Service {
            name: name.clone(),
            flag: flag.clone(),
            port: port,
        };
        let mut conf: Config = match Self::read_conf(DEFAULT_PATH) {
            Ok(conf) => conf,
            Err(_) => return Err("JSON parsing error"),
        };
        if let Some(_index) = conf.teams.iter().position(|x| x.name == name) {
            match Self::update_service(name, flag, port) {
                Ok(()) => {}
                Err(_) => return Err("JSON error"),
            };
            return Ok(());
        }
        conf.services.push(new_service);
        match Self::write_conf(DEFAULT_PATH, conf) {
            Ok(()) => {}
            Err(_) => return Err("JSON error"),
        }

        Ok(())
    }

    pub fn remove_service(name: String) -> std::result::Result<(), &'static str> {
        let mut conf: Config = match Self::read_conf(DEFAULT_PATH) {
            Ok(conf) => conf,
            Err(_) => return Err("JSON parsing error"),
        };
        conf.services.retain(|x| x.name != name);
        match Self::write_conf(DEFAULT_PATH, conf) {
            Ok(()) => {}
            Err(_) => return Err("JSON error"),
        }
        Ok(())
    }

    pub fn update_service(
        name: String,
        flag: String,
        port: u32,
    ) -> std::result::Result<(), &'static str> {
        let mut conf: Config = match Self::read_conf(DEFAULT_PATH) {
            Ok(conf) => conf,
            Err(_) => return Err("JSON parsing error"),
        };
        if let Some(index) = conf.services.iter().position(|x| x.name == name) {
            conf.services[index] = Service { name, flag, port };
        }
        match Self::write_conf(DEFAULT_PATH, conf) {
            Ok(()) => {}
            Err(_) => return Err("JSON error"),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    const DEFAULT_PATH: &str = "./static/conf.json";

    #[test]
    fn test_read_conf() {
        let config: Config = Config::read_conf(DEFAULT_PATH).unwrap();
        assert_eq!(config.teams[0].name, "PLUS");
        assert_eq!(config.teams[0].ip, "0.0.0.0");

        assert_eq!(config.teams[1].name, "PLUS1");
        assert_eq!(config.teams[1].ip, "0.0.0.1");

        assert_eq!(config.teams[2].name, "PLUS2");
        assert_eq!(config.teams[2].ip, "0.0.0.2");

        assert_eq!(config.services[0].name, "bof");
        assert_eq!(config.services[0].flag, "DEFCON{");
        assert_eq!(config.services[0].port, 8888);

        assert_eq!(config.services[1].name, "uaf");
        assert_eq!(config.services[1].flag, "DEFCON{");
        assert_eq!(config.services[1].port, 7777);
    }

    #[test]
    fn test_get_teams() {
        let teams = Config::get_teams().unwrap();

        assert_eq!(teams[0].name, "PLUS");
        assert_eq!(teams[0].ip, "0.0.0.0");

        assert_eq!(teams[1].name, "PLUS1");
        assert_eq!(teams[1].ip, "0.0.0.1");

        assert_eq!(teams[2].name, "PLUS2");
        assert_eq!(teams[2].ip, "0.0.0.2");
    }

    #[test]
    fn test_get_services() {
        let services = Config::get_services().unwrap();

        assert_eq!(services[0].name, "bof");
        assert_eq!(services[0].flag, "DEFCON{");
        assert_eq!(services[0].port, 8888);

        assert_eq!(services[1].name, "uaf");
        assert_eq!(services[1].flag, "DEFCON{");
        assert_eq!(services[1].port, 7777);
    }

    #[test]
    fn test_get_team_list() {
        let team_list = Config::get_team_list().unwrap();
        assert_eq!(vec!["PLUS", "PLUS1", "PLUS2"], team_list)
    }

    #[test]
    fn test_get_service_list() {
        let service_list = Config::get_service_list().unwrap();
        assert_eq!(vec!["bof", "uaf"], service_list)
    }

    #[test]
    fn test_team_name_to_ip() {
        assert_eq!(
            Config::team_name_to_ip("PLUS".to_string()),
            Ok("0.0.0.0".to_string())
        );
        assert_eq!(
            Config::team_name_to_ip("PLUS1".to_string()),
            Ok("0.0.0.1".to_string())
        );
        assert_eq!(
            Config::team_name_to_ip("PLUS2".to_string()),
            Ok("0.0.0.2".to_string())
        );
    }

    #[test]
    fn test_team_ip_to_name() {
        assert_eq!(
            Config::team_ip_to_name("0.0.0.0".to_string()),
            Ok("PLUS".to_string())
        );
        assert_eq!(
            Config::team_ip_to_name("0.0.0.1".to_string()),
            Ok("PLUS1".to_string())
        );
        assert_eq!(
            Config::team_ip_to_name("0.0.0.2".to_string()),
            Ok("PLUS2".to_string())
        );
    }

    #[test]
    fn test_service_name_to_flag() {
        assert_eq!(
            Config::service_name_to_flag("bof".to_string()),
            Ok("DEFCON{".to_string())
        );
    }

    #[test]
    fn test_service_name_to_port() {
        assert_eq!(Config::service_name_to_port("bof".to_string()), Ok(8888));
        assert_eq!(Config::service_name_to_port("uaf".to_string()), Ok(7777));
    }

    #[test]
    fn test_service_port_to_name() {
        assert_eq!(Config::service_port_to_name(8888), Ok("bof".to_string()));
        assert_eq!(Config::service_port_to_name(7777), Ok("uaf".to_string()));
    }

    #[test]
    fn test_write_conf() {
        let config: Config = Config::read_conf(DEFAULT_PATH).unwrap();
        Config::write_conf(DEFAULT_PATH, config).unwrap();
    }

    #[test]
    fn test_add_remove_update_team() {
        assert_eq!(
            Config::add_team("EXAMPLE".to_string(), "1.1.1.1".to_string()),
            Ok(())
        );
        assert_eq!(
            Config::team_name_to_ip("EXAMPLE".to_string()),
            Ok("1.1.1.1".to_string())
        );
        assert_eq!(
            Config::update_team("EXAMPLE".to_string(), "2.2.2.2".to_string()),
            Ok(())
        );
        assert_eq!(
            Config::team_name_to_ip("EXAMPLE".to_string()),
            Ok("2.2.2.2".to_string())
        );
        assert_eq!(Config::remove_team("EXAMPLE".to_string()), Ok(()));
        assert_eq!(
            Config::team_name_to_ip("EXAMPLE".to_string()),
            Err("Not found")
        );
    }

    #[test]
    fn test_add_remove_update_service() {
        assert_eq!(
            Config::add_service("EXAMPLE".to_string(), "FLAG".to_string(), 0),
            Ok(())
        );
        assert_eq!(
            Config::service_name_to_flag("EXAMPLE".to_string()),
            Ok("FLAG".to_string())
        );
        assert_eq!(Config::service_name_to_port("EXAMPLE".to_string()), Ok(0));
        assert_eq!(
            Config::update_service("EXAMPLE".to_string(), "GALF".to_string(), 1),
            Ok(())
        );
        assert_eq!(
            Config::service_name_to_flag("EXAMPLE".to_string()),
            Ok("GALF".to_string())
        );
        assert_eq!(Config::service_name_to_port("EXAMPLE".to_string()), Ok(1));
        assert_eq!(Config::remove_service("EXAMPLE".to_string()), Ok(()));
        assert_eq!(
            Config::service_name_to_flag("EXAMPLE".to_string()),
            Err("Not found")
        );
    }
}
