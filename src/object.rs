extern crate rocksdb;
extern crate bincode;


use self::rocksdb::{DB, WriteBatch, Writable};
use self::bincode::{serialize, deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Team {
    name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Service {
    title: String,
    description: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ServiceVariant {
    service: Service,
    description: String,
    published_by: Team,
    version: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ServiceProvider {
    team: Team,
    connection: String,
    service_variants: [ServiceVariant;32],
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
    pub fn new(team: Team, connection: String, service_variants: [ServiceVariant;32]) -> Result<Self, String> {
        Ok(ServiceProvider {
            team,
            connection,
            service_variants
        })
    }

    pub fn insert(&self) -> Result<(), String> {
        let db = DB::open_default("./db").unwrap();
        let encoded: Vec<u8> = serialize(self).unwrap();

        db.put(self.team.name.as_bytes(), encoded.as_slice());

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