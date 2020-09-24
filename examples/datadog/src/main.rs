use opentelemetry::api::{Key, Span, TraceContextExt, Tracer};
use opentelemetry::global;
use std::thread;
use std::time::Duration;
use opentelemetry_contrib::datadog::ApiVersion;

fn bar() {
    let tracer = global::tracer("component-bar");
    let span = tracer.start("bar");
    span.set_attribute(Key::new("span.type").string("sql"));
    span.set_attribute(Key::new("sql.query").string("SELECT * FROM table"));
    thread::sleep(Duration::from_millis(6));
    span.end()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tracer = opentelemetry_contrib::datadog::new_pipeline()
        .with_service_name("trace-demo")
        .with_version(ApiVersion::Version05)
        .install()?;

    tracer.in_span("foo", |cx| {
        let span = cx.span();
        span.set_attribute(Key::new("span.type").string("web"));
        span.set_attribute(Key::new("http.url").string("http://localhost:8080/foo"));
        span.set_attribute(Key::new("http.method").string("GET"));
        span.set_attribute(Key::new("http.status_code").i64(200));

        thread::sleep(Duration::from_millis(6));
        bar();
        thread::sleep(Duration::from_millis(6));
    });

    Ok(())
}
