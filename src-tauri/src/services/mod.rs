// services/mod.rs - Exports des services
pub mod csv_loader;
pub mod csv_cleaner;
pub mod metrics; // Module refactoré en sous-modules
pub mod volatility; // Module refactoré en sous-modules
pub mod calendar_scraper;
pub mod economic_event_loader;
pub mod calendar_converter;
pub mod event_correlation;
pub mod pair_data; // Module refactoré en sous-modules
pub mod session_analyzer;
pub mod session; // Module session avec corrélation calendrier
pub mod config_service;

pub use csv_loader::CsvLoader;
pub use csv_cleaner::{clean_european_csv, create_cleaned_dir, CleaningReport};
pub use metrics::MetricsCalculator;
pub use volatility::VolatilityAnalyzer; // Ré-exporté depuis volatility/mod.rs
pub use calendar_scraper::CalendarScraper;
pub use economic_event_loader::EconomicEventLoader;
pub use calendar_converter::CalendarConverter;
pub use event_correlation::{EventCorrelationService, CorrelationStats};
pub use pair_data::PairDataConverter;
pub use session::CalendarCorrelator;
pub use config_service::ConfigService;
