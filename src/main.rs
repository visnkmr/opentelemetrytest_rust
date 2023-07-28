use std::{time::Duration, error::Error, thread, sync::mpsc};
use fltk::{
    enums::{Color, FrameType, Event, CallbackTrigger},
    app::MouseButton,
    app::{App,*},
    prelude::{DisplayExt, GroupExt, WidgetBase, WidgetExt},
    text::{TextBuffer, TextDisplay},
    window::Window,
    button::{Button,CheckButton},
   input::Input,
    prelude::*, frame::Frame,
};
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
    init_tracer()?;
    let tracer = global::tracer("example-opentelemetry/mainfun");
    let mut WIDGET_WIDTH: i32 = 420;
    let mut WIDGET_HEIGHT: i32 = 400;
    let mut WIDGET_PADDING: i32 = 20;

    // let mut span=tracer
    //                 .span_builder("init")
    //                 .with_kind(SpanKind::Internal)
    //                 .start(&tracer);
    let mut span = tracer.start("started");
    span.add_event("mainfun".to_string(), vec![]);

    let cx = Context::current_with_span(span);

    thread::spawn(||async{
        let cx = Context::current();
        // let span = cx.span();
        let mut span = global::tracer("infunction").start("startfun");
        span.add_event("in async".to_string(), vec![]);
        test(&"v".to_string()).with_context(cx).await;
        tokio::time::sleep(Duration::from_millis(150)).await;
        
        for i in 1..10{
            let cx = Context::current();
            let span = cx.span();

        span.add_event(format!("in for loop #{i}").to_string(), vec![]);
            test(&format!("{i}")).with_context(cx).await;
        }
    }
    
    // .await
).join().with_context(cx.clone());
    // let (s, r) = mpsc::channel();
    let span = cx.span();
    span.add_event("outside async".to_string(), vec![]);
    test(&"v".to_string()).with_context(cx).await;
    // tokio::time::sleep(Duration::from_millis(150)).await;
    println!("Hello, world!");
    global::shutdown_tracer_provider();
    // process::exit(0);
    Ok(())
}

async fn test(v: &String,){
    // let cx = Context::current();
    //     let span = cx.span();
    let mut span = global::tracer("infunction").start("startfun");
    span.add_event(v.to_string(), vec![]);
    print!("{}",v);
    let mut k=false;
    
    // tokio::time::sleep(Duration::from_millis(1500)).await;
}
