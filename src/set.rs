use object::*;
use rocksdb::{rocksdb_options::Options, DB};
//use std::collections::HashMap;

pub fn set_team_info() -> Result<(), String> {
    let opts = Options::new();
    DB::destroy(&opts, "./db")?;

    // Set a team's information per team
    let team1 = Team {
        name: "0daysober".to_string(),
    };
    let service1_1 = Service {
        title: "pointless".to_string(),
        description: "Too hard".to_string(),
    };
    let service_variant1_1 = ServiceVariant {
        service: service1_1,
        port: "4242".to_string(),
        published_by: team1.clone(),
        version: "0x111111".to_string(),
    };

    let mut services = Vec::new();
    services.push(service_variant1_1);

    let service_provider1 = ServiceProvider::new(team1.clone(), "10.13.37.01".to_string(), services)?;

    service_provider1.insert()?;
    // End

    println!("Setting the initial teams information is complete");
    Ok(())
}
