use std::env;
use std::time::Duration;

use anyhow::Result;

use lambda_http::tracing::{
    self,
    metadata::LevelFilter,
    subscriber::{layer::SubscriberExt, util::SubscriberInitExt},
    Level,
};
use opentelemetry::KeyValue;
use opentelemetry_otlp::{TonicExporterBuilder, WithExportConfig};
use opentelemetry_sdk::{
    metrics::{
        reader::{DefaultAggregationSelector, DefaultTemporalitySelector},
        Instrument, MeterProviderBuilder, PeriodicReader, SdkMeterProvider, Stream,
    },
    runtime,
    trace::Tracer,
    Resource,
};
use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};

use crate::utils::Pipe;

fn setup_exporter() -> TonicExporterBuilder {
    let otel_exporter_endpoint = std::env::var("OTEL_ENDPOINT").expect("OTEL_ENDPOINT not set");

    opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(otel_exporter_endpoint)
}

fn resource() -> Result<Resource> {
    Ok(Resource::new([
        KeyValue::new("SERVICE_NAME", env!("CARGO_PKG_NAME")),
        KeyValue::new("SERVICE_VERSION", env!("CARGO_PKG_VERSION")),
        KeyValue::new("DEPLOYMENT_ENVIRONMENT", env::var("ENVIRONMENT")?),
    ]))
}

fn setup_meter_views(meter_provider_builder: MeterProviderBuilder) -> SdkMeterProvider {
    let view_payload_recieved = |instrument: &Instrument| -> Option<Stream> {
        //TODO: Add implementation here
        if instrument.name != "" {
            return None;
        }

        Some(Stream::new())
    };

    meter_provider_builder
        .with_view(view_payload_recieved)
        .build()
}

fn setup_meter_provider() -> Result<SdkMeterProvider> {
    let exporter = setup_exporter().build_metrics_exporter(
        Box::new(DefaultAggregationSelector::default()),
        Box::new(DefaultTemporalitySelector::default()),
    )?;

    let reader = PeriodicReader::builder(exporter, runtime::Tokio)
        .with_interval(Duration::from_secs(30))
        .build();

    let meter_provider = MeterProviderBuilder::default()
        .with_resource(resource()?)
        .with_reader(reader)
        .pipe(setup_meter_views);

    opentelemetry::global::set_meter_provider(meter_provider.clone());

    Ok(meter_provider)
}

fn setup_tracer() -> Result<Tracer> {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(setup_exporter())
        .with_batch_config(opentelemetry_sdk::trace::BatchConfig::default())
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;

    Ok(tracer)
}

pub(crate) struct OtelGuard {
    meter_provider: SdkMeterProvider,
}
impl Drop for OtelGuard {
    fn drop(&mut self) {
        self.meter_provider
            .shutdown()
            .expect("Failed to shutdown meter provider");
        opentelemetry::global::shutdown_tracer_provider();
    }
}

pub(crate) async fn setup_telemetry() -> Result<OtelGuard> {
    let meter_provider = setup_meter_provider()?;

    let json_layer = tracing::subscriber::fmt::layer().json();

    tracing::subscriber::registry()
        .with(LevelFilter::from_level(Level::INFO))
        .with(json_layer)
        .with(MetricsLayer::new(meter_provider.clone()))
        .with(OpenTelemetryLayer::new(setup_tracer()?))
        .init();

    Ok(OtelGuard { meter_provider })
}
