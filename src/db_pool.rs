use std::ops::Deref;

use futures::executor;
use r2d2::ManageConnection;
use wither::mongodb::{Client, Database, error::Error};

pub struct WitherConnectionManger {
    uri: String
}

impl WitherConnectionManger {
    pub fn from_uri(uri: &str) -> Self {
        Self { uri: uri.into() }
    }
}

pub struct WitherConnection {
    client: Client,
    db: Database
}

impl Deref for WitherConnection {
    type Target = Database;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}

impl ManageConnection for WitherConnectionManger {
    type Connection = WitherConnection;
    type Error = Error;

    fn connect(&self) -> Result<Self::Connection, Error> {
        let client = executor::block_on(Client::with_uri_str(self.uri.as_ref())).expect("failed connected to db");

        Ok(WitherConnection {
            db: client.database("production"),
            client: client
        })
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Error> {
        Ok(())
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        self.is_valid(conn).is_err()
    }
}
