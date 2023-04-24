#![allow(dead_code)]

use mongodb::{options::ClientOptions, Client};

use crate::writer::{mongo, Writer};

/// File type to which fetched schemas will be stored (currently only Parquet and Json are supported)
/// Default is Json
#[derive(Debug, Default)]
pub enum FileType {
    Parquet,
    #[default]
    Json,
}

/// Source to which fetched schemas will be stored
/// Mongo stores schemas in a MongoDB database (default): String argument is the MongoDB URI
/// FileSystem stores schemas in a local directory: FileType argument is the file type
/// S3 stores schemas in an S3 bucket: FileType argument is the file type
#[derive(Debug)]
pub enum Source {
    Mongo(String),
    FileSystem(FileType),
    S3(FileType),
}

impl Default for Source {
    fn default() -> Self {
        let mongo_uri: String =
            std::env::var("MONGO_URI").unwrap_or("mongodb://localhost:27017".to_string());
        Source::Mongo(mongo_uri)
    }
}

/// Application Context
/// Contains information about the source to which fetched schemas will be stored
pub struct Application {
    source: Source,
    writer: Box<dyn Writer>,
}

impl Application {
    /// Creates a build Application Context
    /// If no source is provided, the default is a MongoDB database at localhost:27017
    pub async fn build(source: Option<Source>) -> anyhow::Result<Self> {
        let source = source.unwrap_or_default();
        // TODO: Add support for FileSystem and S3
        let writer = match source {
            // Creates a new MongoWriter
            Source::Mongo(_) => {
                let mongo_uri =
                    std::env::var("MONGO_URI").unwrap_or("mongodb://localhost:27017".to_string());
                let client_option = ClientOptions::parse(mongo_uri).await?;
                let client = Client::with_options(client_option)?;
                let database = std::env::var("MONGO_DATABASE").map(Some).unwrap_or(None);
                let collection = std::env::var("MONGO_COLLECTION").map(Some).unwrap_or(None);
                mongo::MongoWriter::new(client, database, collection)
            }
            _ => unimplemented!(),
        };
        let writer = Box::new(writer);
        Ok(Self { source, writer })
    }

    /// Returns the source to which fetched schemas will be stored
    pub fn source(&self) -> &Source {
        &self.source
    }
}
