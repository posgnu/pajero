use object::*;
use rocksdb::{rocksdb_options::Options, DB};
use std::collections::HashMap;

pub fn SetTeamInfo() -> Result<(), String> {
    let opts = Options::new();
    let result = DB::destroy(&opts, "./db");

    let team1 = Team {
        name: "team1".to_string(),
    };
    let service1_1 = Service {
        title: "pwn".to_string(),
        description: "Too hard".to_string(),
    };
    let service_variant1_1 = ServiceVariant {
        service: service1_1,
        port: 80,
        published_by: team1.clone(),
        version: "0x12213123".to_string(),
    };

    let mut services = Vec::new();
    services.push(service_variant1_1);

    let service_provider1 = ServiceProvider::new(team1.clone(), "0.0.0.0".to_string(), services)?;

    service_provider1.insert();

    Ok(())
}
