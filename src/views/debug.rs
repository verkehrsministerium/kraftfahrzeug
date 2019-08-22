use cursive::{view::View, Vec2, Printer};
use flexi_logger::{FormatFunction, DeferredNow, Record, writers::LogWriter};
use arraydeque::{ArrayDeque, Wrapping};
use std::cell::RefCell;

pub(crate) fn logs_with<F, R>(f: F) -> R
where
    F: FnOnce(&RefCell<ArrayDeque<[String; 2048], Wrapping>>) -> R,
{
    thread_local! {
        static LOGS: RefCell<ArrayDeque<[String; 2048], Wrapping>> =
            RefCell::new(ArrayDeque::new());
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
        log::info!(target: "{cursive}", "drawing");

        logs_with(|logs| {
            // Only print the last logs, so skip what doesn't fit
            let skipped = logs.borrow().len().saturating_sub(printer.size.y);

            for (i, msg) in logs.borrow().iter().skip(skipped).enumerate() {
                printer.print((0, i), &msg);
            }
        });
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        logs_with(|logs| {
            // The longest line sets the width
            let w = logs.borrow().iter()
                .map(|msg| msg.len())
                .max()
                .unwrap_or(1);
            let h = logs.borrow().len();
            let w = if w < constraint.x { constraint.x } else { w };
            let h = if h < constraint.y { constraint.y } else { h };

            Vec2::new(w, h)
        })
    }
}

pub struct CursiveLogWriter {
    format: FormatFunction,
}

pub fn cursive_log_writer(format: FormatFunction) -> Box<CursiveLogWriter> {
    Box::new(CursiveLogWriter {
        format,
    })
}

impl LogWriter for CursiveLogWriter {
    fn write(&self, now: &mut DeferredNow, record: &Record) -> std::io::Result<()> {
        let mut line = Vec::new();
        (self.format)(&mut line, now, record)?;

        let line = unsafe { String::from_utf8_unchecked(line) };

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
