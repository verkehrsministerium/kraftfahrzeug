use arraydeque::{ArrayDeque, Wrapping};
use cursive::theme::{BaseColor, Color};
use cursive::utils::markup::StyledString;
use cursive::view::View;
use cursive::{Printer, Vec2};
use flexi_logger::{writers::LogWriter, DeferredNow, Level, Record};
use std::cell::RefCell;
use std::thread;

type LogBuffer = ArrayDeque<[StyledString; 2048], Wrapping>;

pub(crate) fn logs_with<F, R>(f: F) -> R
where
    F: FnOnce(&RefCell<LogBuffer>) -> R,
{
    thread_local! {
        static LOGS: RefCell<LogBuffer> = RefCell::new(LogBuffer::new());
    }
    LOGS.with(f)
}

pub struct DebugView;

impl DebugView {
    pub fn new() -> Self {
        Self
    }
}

impl View for DebugView {
    fn draw(&self, printer: &Printer<'_, '_>) {
        logs_with(|logs| {
            // Only print the last logs, so skip what doesn't fit
            let skipped = logs.borrow().len().saturating_sub(printer.size.y);

            for (i, msg) in logs.borrow().iter().skip(skipped).enumerate() {
                printer.print_styled((0, i), msg.into());
            }
        });
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        logs_with(|logs| {
            // The longest line sets the width
            let w = logs
                .borrow()
                .iter()
                .map(|msg| msg.width())
                .max()
                .unwrap_or(1);
            let h = logs.borrow().len();
            let w = if w < constraint.x { constraint.x } else { w };
            let h = if h < constraint.y { constraint.y } else { h };

            Vec2::new(w, h)
        })
    }
}

pub struct CursiveLogWriter;

pub fn cursive_log_writer() -> Box<CursiveLogWriter> {
    Box::new(CursiveLogWriter)
}

impl LogWriter for CursiveLogWriter {
    fn write(&self, now: &mut DeferredNow, record: &Record) -> std::io::Result<()> {
        let color = Color::Dark(match record.level() {
            Level::Trace => BaseColor::Green,
            Level::Debug => BaseColor::Cyan,
            Level::Info => BaseColor::Blue,
            Level::Warn => BaseColor::Yellow,
            Level::Error => BaseColor::Red,
        });

        let mut line = StyledString::new();
        line.append_styled(format!("{}", now.now().format("%T%.3f")), color);
        line.append_plain(format!(
            " [{}] ",
            thread::current().name().unwrap_or("(unnamed)"),
        ));
        line.append_styled(format!("{}", record.level()), color);
        line.append_plain(format!(
            " <{}:{}> ",
            record.file().unwrap_or("(unnamed)"),
            record.line().unwrap_or(0),
        ));
        line.append_styled(format!("{}", &record.args()), color);

        logs_with(|logs| {
            logs.borrow_mut().push_back(line);
        });

        Ok(())
    }

    fn flush(&self) -> std::io::Result<()> {
        // we are not buffering
        Ok(())
    }

    fn max_log_level(&self) -> log::LevelFilter {
        log::LevelFilter::max()
    }
}
