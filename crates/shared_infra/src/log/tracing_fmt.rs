use chrono::{DateTime, Local};
use serde_json::{json, Map, Value};
use std::fmt;
use tracing::field::Visit;
use tracing_core::{Event, Field, Subscriber};
use tracing_subscriber::fmt::{
    format::{self, FormatEvent, FormatFields},
    FmtContext, FormattedFields,
};
use tracing_subscriber::registry::LookupSpan;

pub struct PglJsonTracingFormatter;

pub struct JsonVisitor {
    values: Map<String, Value>,
}

impl JsonVisitor {
    fn new() -> Self {
        Self { values: Map::new() }
    }
}

impl Visit for JsonVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if field.name() == "message" {
            // Parse the value as JSON and insert it directly
            let json_value = serde_json::from_str(&format!("{:?}", value))
                .unwrap_or_else(|_| json!(format!("{:?}", value)));
            self.values.insert(field.name().to_string(), json_value);
        } else {
            self.values.insert(
                field.name().to_string(),
                Value::String(format!("{:?}", value)),
            );
        }
    }
}

impl<S, N> FormatEvent<S, N> for PglJsonTracingFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let mut visitor = JsonVisitor::new();
        event.record(&mut visitor);

        if let Some(scope) = ctx.event_scope() {
            for span in scope.from_root() {
                let ext = span.extensions();
                let fields = &ext
                    .get::<FormattedFields<N>>()
                    .expect("will never be `None`");

                let mut parsed: Map<String, Value> = serde_json::from_str(&fields.fields).unwrap();

                visitor.values.append(&mut parsed);
            }
        }

        let now: DateTime<Local> = Local::now();

        let format = "%d-%m-%Y %H:%M:%S%.3f";
        let date_string = now.format(format).to_string();

        visitor
            .values
            .insert("timestamp".to_string(), date_string.into());

        let json = Value::Object(visitor.values);
        writeln!(writer, "{}", json)
    }
}
