use std::{time::Duration, error::Error};

use opentelemetry::{trace::{TraceError, Tracer, TraceContextExt, FutureExt, SpanKind, Span, get_active_span}, sdk::{trace::Config, Resource}, KeyValue, global, Key, Context};
fn init_tracer() -> Result<opentelemetry::sdk::trace::Tracer, TraceError> {
    opentelemetry_jaeger::new_pipeline()
        .with_service_name("trace-demo")
        .with_trace_config(Config::default().with_resource(Resource::new(vec![
            KeyValue::new("exporter", "otlp-jaeger"),
            KeyValue::new("service.name", "perlink"),
            KeyValue::new("service.version", "0.1"),
            KeyValue::new("host.os", std::env::consts::OS),
            KeyValue::new("host.architecture", std::env::consts::ARCH),
       
        ])))
        .install_batch(opentelemetry::runtime::Tokio)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>>  {
    let tracer = init_tracer()?;
    // let mut span=tracer
    //                 .span_builder("init")
    //                 .with_kind(SpanKind::Internal)
    //                 .start(&tracer);
    let mut span = global::tracer("example-opentelemetry/mainfun").start("started");
    span.add_event("mainfun".to_string(), vec![]);

    let cx = Context::current_with_span(span);
    test(&"v".to_string()).with_context(cx).await;

    tokio::time::sleep(Duration::from_millis(150)).await;
    println!("Hello, world!");
    global::shutdown_tracer_provider();
    Ok(())
}

async fn test(v: &String,){
    // let cx = Context::current();
    //     let span = cx.span();
    let mut span = global::tracer("infunction").start("startfun");
    span.add_event(v.to_string(), vec![]);
    print!("{}",v);
    tokio::time::sleep(Duration::from_millis(1500)).await;
}
