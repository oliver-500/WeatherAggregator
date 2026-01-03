use std::time::Duration;
use opentelemetry::global;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::{ExporterBuildError, WithExportConfig, WithTonicConfig};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::{BatchConfigBuilder, BatchSpanProcessor, SdkTracerProvider, SpanExporter};
use tracing::dispatcher::set_global_default;
use tracing::{Dispatch, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{EnvFilter, Registry};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use crate::org::unibl::etf::configuration::settings::TracingSettings;

fn build_tracing_span_exporter(tracing_backend_settings: TracingSettings)
                               -> Result<impl SpanExporter, ExporterBuildError> {
    let tls_config = tracing_backend_settings.get_tls_config();

    let builder =
        opentelemetry_otlp::SpanExporter::builder()
            .with_tonic()
            .with_timeout(Duration::from_millis(tracing_backend_settings.timeout_in_ms as u64));

    match tls_config {
        Ok(tls_config) => {
            if let Some(tls_config) = tls_config {
                let endpoint =
                    format!("https://{}:{}",
                            tracing_backend_settings.host,
                            tracing_backend_settings.port,
                    );
                return builder.with_tls_config(tls_config).with_endpoint(endpoint).build();
            }
            else {
                let endpoint =
                    format!("http://{}:{}",
                            tracing_backend_settings.host,
                            tracing_backend_settings.port,
                    );
                return builder.with_endpoint(endpoint).build();
            }
        }
        Err(e) => {
            panic!("Could not get required tls config. {}", e);
        }
    }
}

pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
    tracing_backend_settings: TracingSettings,

) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name.clone(), sink);

    //fail because setup is incorrect hence tracing backend would never be connected to,
    //this won't fail when tracing backend is offline
    let otlp_exporter = build_tracing_span_exporter(tracing_backend_settings.clone())
        .expect("Could not create OTLP tracing exporter.");

    let batch_config = BatchConfigBuilder::default()
        .with_max_export_batch_size(tracing_backend_settings.max_export_batch_size as usize)
        .with_max_queue_size(tracing_backend_settings.max_queue_size as usize)
        .with_scheduled_delay(Duration::from_millis(tracing_backend_settings.scheduled_delay_in_ms as u64))
        .with_max_concurrent_exports(tracing_backend_settings.max_concurrent_exports as usize)
        .build(); //env
    let batch_processor = BatchSpanProcessor::builder(otlp_exporter)
        .with_batch_config(batch_config)
        .build();

    let resource = Resource::builder().with_service_name(name.clone()).build();

    let tracer_provider = SdkTracerProvider::builder()
        .with_span_processor(batch_processor)
        .with_resource(resource)
        .build();
    let tracer = tracer_provider.tracer("cache_service_tracer");
    global::set_tracer_provider(tracer_provider); // then register globally

    let otel_layer = OpenTelemetryLayer::new(tracer);

    global::set_text_map_propagator(TraceContextPropagator::new());

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(otel_layer)
}

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger.");
    set_global_default(Dispatch::from(subscriber)).expect("Failed to set subscriber.");
}