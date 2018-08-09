use bincode::{deserialize, serialize};
use rocksdb::{Writable, DB};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Team {
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Service {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ServiceVariant {
    pub service: Service,
    pub port: u8,
    pub published_by: Team,
    pub version: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ServiceProvider {
    team: Team,
    connection: String,
    service_variants: Vec<ServiceVariant>,
}

#[derive(PartialEq)]
pub struct ExploitRequest<'a> {
    title: String,
    packet: &'a [u8],
    service_provider: &'a [ServiceProvider],
}

#[derive(PartialEq)]
pub struct ExploitResponse {
    succeed: bool,
    flag: String,
}

impl ServiceProvider {
    pub fn new(
        team: Team,
        connection: String,
        service_variants: Vec<ServiceVariant>,
    ) -> Result<Self, String> {
        Ok(ServiceProvider {
            team,
            connection,
            service_variants,
        })
    }

    pub fn insert(&self) -> Result<(), String> {
        let db = DB::open_default("./db").unwrap();
        let encoded: Vec<u8> = serialize(self).unwrap();

        db.put(self.team.name.as_bytes(), encoded.as_slice())?;

        Ok(())
    }

    pub fn get(&self) -> Result<(), String> {
        let db = DB::open_default("./db").unwrap();

        match db.get(self.team.name.as_bytes()) {
            Ok(Some(value)) => println!("retrieved value {}", value.to_utf8().unwrap()),
            Ok(None) => println!("value not found"),
            Err(e) => println!("operational problem encountered: {}", e),
        }

        Ok(())
    }
}
