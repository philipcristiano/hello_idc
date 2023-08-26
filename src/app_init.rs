use init_tracing_opentelemetry::tracing_subscriber_ext;
use tracing::Level;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry;

use opentelemetry::{global, runtime, Key, KeyValue};
use opentelemetry_sdk::{
    metrics::{
        reader::{DefaultAggregationSelector, DefaultTemporalitySelector},
        Aggregation, Instrument, MeterProvider, PeriodicReader, Stream,
    },
    trace::{BatchConfig, RandomIdGenerator, Sampler, Tracer},
    Resource,
};
use opentelemetry_semantic_conventions::{
    resource::{DEPLOYMENT_ENVIRONMENT, SERVICE_NAME, SERVICE_VERSION},
    SCHEMA_URL,
};
use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};

pub fn logging(level: Level, is_json: bool) {
    let subscriber = tracing_subscriber::fmt().with_max_level(level);
    if is_json {
        subscriber.json().init()
    } else {
        subscriber.init()
    };
}

pub fn tracing(level: Level) {
    let subscriber = registry()
        //.with(tracing_subscriber_ext::build_otel_layer().expect("Couldn't setup otel layer"))
        .with(OpenTelemetryLayer::new(init_tracer()))
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_filter(LevelFilter::from_level(level)),
        );
    tracing::subscriber::set_global_default(subscriber).expect("Could not setup tracing/logging")
}

// Create a Resource that captures information about the entity for which telemetry is recorded.
//fn resource() -> Resource {
//    Resource::from_schema_url([KeyValue::new(SERVICE_NAME, "app_init")], SCHEMA_URL)
//}
fn resource() -> Resource {
    Resource::from_schema_url(
        [
            KeyValue::new(SERVICE_NAME, "app_init"),
            KeyValue::new(SERVICE_VERSION, "0.1.0"),
            KeyValue::new(DEPLOYMENT_ENVIRONMENT, "develop"),
        ],
        SCHEMA_URL,
    )
}

// Construct Tracer for OpenTelemetryLayer
fn init_tracer() -> Tracer {
    use opentelemetry_otlp::TonicExporterBuilder;
    let exporter = TonicExporterBuilder::default();
    //let otlp_exporter = opentelemetry_otlp::new_exporter().http();
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(
            opentelemetry_sdk::trace::Config::default()
                // Customize sampling strategy
                .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
                    1.0,
                ))))
                // If export trace to AWS X-Ray, you can use XrayIdGenerator
                .with_id_generator(RandomIdGenerator::default())
                .with_resource(resource()),
        )
        .with_batch_config(BatchConfig::default())
        .with_exporter(exporter)
        .install_batch(runtime::Tokio)
        .unwrap()
}