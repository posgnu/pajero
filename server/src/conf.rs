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
    pub fn read_conf(path: &str) -> Result<Config> {
        let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
        let conf: Config = serde_json::from_str(&contents)?;
        Ok(conf)
    }

    pub fn write_conf(path: &str, conf: Config) -> std::io::Result<()> {
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

    pub fn team_name_to_ip(name: &str) -> std::result::Result<String, &str> {
        let list = match Self::get_teams() {
            Ok(list) => list,
            Err(_) => return Err("JSON parsing error"),
        };
        for team in list.iter() {
            if name == &team.name {
                return Ok(team.ip.clone());
            }
        }
        Err("Not found")
    }

    pub fn team_ip_to_name(ip: &str) -> std::result::Result<String, &str> {
        let list = match Self::get_teams() {
            Ok(list) => list,
            Err(_) => return Err("JSON parsing error"),
        };
        for team in list.iter() {
            if ip == &team.ip {
                return Ok(team.name.clone());
            }
        }
        Err("Not found")
    }

    pub fn service_name_to_flag(name: &str) -> std::result::Result<String, &str> {
        let list = match Self::get_services() {
            Ok(list) => list,
            Err(_) => return Err("JSON parsing error"),
        };
        for service in list.iter() {
            if name == &service.name {
                return Ok(service.flag.clone());
            }
        }
        Err("Not found")
    }

    pub fn service_name_to_port(name: &str) -> std::result::Result<u32, &str> {
        let list = match Self::get_services() {
            Ok(list) => list,
            Err(_) => return Err("JSON parsing error"),
        };
        for service in list.iter() {
            if name == &service.name {
                return Ok(service.port.clone());
            }
        }
        Err("Not found")
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
        assert_eq!(Config::team_name_to_ip("PLUS").unwrap(), "0.0.0.0");
        assert_eq!(Config::team_name_to_ip("PLUS1").unwrap(), "0.0.0.1");
        assert_eq!(Config::team_name_to_ip("PLUS2").unwrap(), "0.0.0.2");
    }

    #[test]
    fn test_team_ip_to_name() {
        assert_eq!(Config::team_ip_to_name("0.0.0.0").unwrap(), "PLUS");
        assert_eq!(Config::team_ip_to_name("0.0.0.1").unwrap(), "PLUS1");
        assert_eq!(Config::team_ip_to_name("0.0.0.2").unwrap(), "PLUS2");
    }

    #[test]
    fn test_service_name_to_flag() {
        assert_eq!(Config::service_name_to_flag("bof").unwrap(), "DEFCON{");
    }

    #[test]
    fn test_service_name_to_port() {
        assert_eq!(Config::service_name_to_port("bof").unwrap(), 8888);
        assert_eq!(Config::service_name_to_port("uaf").unwrap(), 7777);
    }

    #[test]
    fn test_write_conf() {
        let config: Config = Config::read_conf(DEFAULT_PATH).unwrap();
        Config::write_conf(DEFAULT_PATH, config).unwrap();
    }
}
