use std::{env, net::SocketAddr};

use language::{
  language_service_server::{LanguageService, LanguageServiceServer},
  LanguageReply, LanguageRequest,
};
use lingua::{
  Language,
  Language::{English, French, German, Spanish, Turkish},
  LanguageDetector, LanguageDetectorBuilder,
};
use tonic::{transport::Server, Request, Response, Status};
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

pub mod language {
  tonic::include_proto!("language");
}

pub struct Service {
  detector: LanguageDetector,
}

impl Service {
  pub fn new(detector: LanguageDetector) -> Self { Self { detector } }
}

#[tonic::async_trait]
impl LanguageService for Service {
  async fn detect_language(&self, request: Request<LanguageRequest>) -> Result<Response<LanguageReply>, Status> {
    info!("Received a request from {:?}", request.remote_addr());

    let text = request.into_inner().text;
    if text.is_empty() {
      error!("Received an empty text for language detection");
      return Err(Status::invalid_argument("Text cannot be empty"));
    }

    let detected_language = self.detector.detect_language_of(&text);
    let reply = LanguageReply {
      language: detected_language.map_or_else(|| "Unknown".to_string(), |lang| format!("{:?}", lang)),
    };

    Ok(Response::new(reply))
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv::dotenv().ok();

  let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).finish();
  tracing::subscriber::set_global_default(subscriber)?;

  let addr: SocketAddr = env::var("LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string()).parse().expect("Failed to parse LISTEN_ADDR");

  let languages = vec![English, French, German, Spanish, Turkish];
  info!("Supported languages: {:?}", languages);

  let detector = LanguageDetectorBuilder::from_languages(&languages).build();
  info!("Language detector initialized");

  let service = Service::new(detector);
  info!("Starting server on {}", addr);

  Server::builder().add_service(LanguageServiceServer::new(service)).serve(addr).await?;

  Ok(())
}
