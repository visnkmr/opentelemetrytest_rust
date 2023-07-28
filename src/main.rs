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
    let tracer = init_tracer()?;
    let mut WIDGET_WIDTH: i32 = 420;
    let mut WIDGET_HEIGHT: i32 = 400;
    let mut WIDGET_PADDING: i32 = 20;

    // let mut span=tracer
    //                 .span_builder("init")
    //                 .with_kind(SpanKind::Internal)
    //                 .start(&tracer);
    let mut span = global::tracer("example-opentelemetry/mainfun").start("started");
    span.add_event("mainfun".to_string(), vec![]);

    let cx = Context::current_with_span(span);
    let mut app = App::default();
            
            let mut win = Window::default().with_size(WIDGET_WIDTH, WIDGET_HEIGHT).with_label("Choose browser");

            let (s, r) = fltk::app::channel();

           
            let mut vpack=fltk::group::Pack::new(WIDGET_PADDING,
                WIDGET_PADDING,
                WIDGET_WIDTH - 40,
                WIDGET_HEIGHT - 40,"");
                win.resizable(&vpack);
              
                let mut framet = fltk::frame::Frame::default()
                .with_size(800,60)
                // .center_of(&win)
                .with_label("Loading");
              
            framet.set_label_size(12);
            
            fltk::frame::Frame::default().with_size(20, 10);
            let mut ttb=fltk::group::Pack::default().with_size(
                10,
                40) ;
                fltk::frame::Frame::default().with_size(20, 30);
            // let mut bframe1 = fltk::frame::Frame::default().with_size(300, 60);
            let mut b11 = Button::default().with_size(150,30);
            b11.set_label("All browsers");
            // b1.emit(s, "refresh".to_string());
            // let mut hpack=hpack.clone();
            b11.emit(s.clone(),"all".to_string());
            

            ttb.end();
            ttb.set_type(fltk::group::PackType::Horizontal);
            win.make_resizable(true);
            // win.resizable(&vpack);

            vpack.end();    
            vpack.set_type(fltk::group::PackType::Vertical);
            
            win.show_with_env_args();

            win.end();
            win.show();
            while app.wait() {
                // setframe(&mut frame, "");
                // frame=frame.clone();
                match r.recv() {
                    Some(val) => {
                        if(val == "all"){
                            // let cx = Context::current();
                            // let span = cx.span();
                            let mut span = global::tracer("all").start("all");
                            span.add_event("button clicked".to_string(), vec![]);
                            fltk::app::quit()
                        }
                    },
                    None=>{
                        
                    }
                }
            }.with_context(cx.clone());

    async{
        let cx = Context::current();
        let span = cx.span();

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
    .with_context(cx.clone())
    .await;
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
