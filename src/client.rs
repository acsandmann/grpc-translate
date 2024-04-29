use std::env;

use language::{language_service_client::LanguageServiceClient, LanguageRequest};
use tonic::transport::Channel;
use tracing::{info, warn};
use tracing_subscriber::FmtSubscriber;

pub mod language {
  tonic::include_proto!("language");
}

/// Initializes logging for the application.
fn init_logging() {
  let subscriber = FmtSubscriber::builder().with_env_filter(tracing_subscriber::EnvFilter::from_default_env()).finish();
  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

/// Parses command-line arguments and returns a concatenated string of all
/// arguments.
fn parse_arguments() -> String { env::args().skip(1).collect::<Vec<_>>().join(" ") }

/// Returns the server address from environment variables or defaults if not
/// set.
fn server_address() -> String { env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string()) }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv::dotenv().ok();
  init_logging();

  let args = parse_arguments();
  if args.is_empty() {
    warn!("No text provided to detect language.");
    return Ok(());
  }

  let addr = server_address();
  let mut client = LanguageServiceClient::connect(addr).await?;

  let request = tonic::Request::new(LanguageRequest { text: args });

  info!("Sending request: {:#?}", request);

  let response = client.detect_language(request).await?;

  info!("Received response: {:#?}", response);

  Ok(())
}
