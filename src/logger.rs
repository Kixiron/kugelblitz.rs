use console::style;
use flexi_logger::{
    colored_detailed_format, Age, Cleanup, Criterion, DeferredNow, Duplicate, Logger, Naming,
    Record,
};
use log::{kv, Level};

pub fn setup_logger() -> Result<(), Box<dyn std::error::Error>> {
    Logger::with_str("off, kugelblitz=info")
        .check_parser_error()?
        .log_to_file()
        .directory("./data/logs")
        .print_message()
        .rotate(
            // Create a new log file every day
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            // Keep logs for 7 days and zips for a year
            Cleanup::KeepLogAndZipFiles(7, 365),
        )
        .format_for_files(colored_detailed_format)
        .format_for_stderr(pretty_print)
        .duplicate_to_stderr(Duplicate::Info)
        .start()?;

    Ok(())
}

fn pretty_print(
    write: &mut dyn std::io::Write,
    _now: &mut DeferredNow,
    record: &Record<'_>,
) -> Result<(), std::io::Error> {
    write!(
        write,
        "{}{}",
        format_message(&record),
        format_kv_pairs(&record),
    )
}

fn format_kv_pairs(record: &Record) -> String {
    struct Visitor {
        string: String,
    }

    impl<'kvs> kv::Visitor<'kvs> for Visitor {
        fn visit_pair(
            &mut self,
            key: kv::Key<'kvs>,
            val: kv::Value<'kvs>,
        ) -> Result<(), kv::Error> {
            let string = &format!("   › {}: {}\n", style(key).magenta(), val);
            self.string.push_str(string);
            Ok(())
        }
    }

    let mut visitor = Visitor {
        string: String::new(),
    };
    record.key_values().visit(&mut visitor).unwrap();
    visitor.string
}

fn format_message(record: &Record<'_>) -> String {
    use Level::*;
    let symbol = match record.level() {
        Trace => format!("{}", "◯"),
        Debug => format!("{}", "◎"),
        Info => format!("{}", "●"),
        Warn => format!("{}", "⌿"),
        Error => format!("{}", "✖"),
    };

    let msg = format!("{}  {}", symbol, style(record.args()).underlined());
    match record.level() {
        Trace | Debug | Info => format!("{}", style(msg).green()),
        Warn => format!("{}", style(msg).yellow()),
        Error => format!("{}", style(msg).red()),
    }
}
