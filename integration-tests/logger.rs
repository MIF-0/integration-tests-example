use std::fmt::Arguments;
use chrono::Local;
use env_logger::fmt::Color;
use env_logger::Logger;
use log::{LevelFilter, Record, Log};
use once_cell::sync::OnceCell;
use std::io::Write;
use log::Level::Error;

const LOGGER_TESTING: OnceCell<Logger> = OnceCell::new();
const LOGGER_STATIC_INFO: OnceCell<Logger> = OnceCell::new();


fn create_testing_logger() -> Logger {
    let mut builder = env_logger::Builder::default();

    builder.filter(None, LevelFilter::Info)
        .format(|buf, record| {
            let mut grey_stype = buf.style();
            grey_stype.set_color(Color::Rgb(128, 128, 128));

            let mut level_style = buf.style();
            level_style.set_color(Color::Blue).set_bold(true);

            let mut test_stype = buf.style();
            test_stype.set_color(Color::Green).set_bold(true);

            writeln!(buf, "{}{} {} ",
                     grey_stype.value("["),
                     Local::now().format("%Y-%m-%dT%H:%M:%S.%s"),
                     test_stype.value(record.args()))
        });
    builder.build()
}

fn create_test_static_info_logger() -> Logger {
    let mut builder = env_logger::Builder::default();

    builder.filter(None, LevelFilter::Info)
        .format(|buf, record| {
            let mut grey_stype = buf.style();
            grey_stype.set_color(Color::Rgb(128, 128, 128));

            let mut level_style = buf.style();
            level_style
                .set_color(Color::Blue)
                .set_bold(true);

            let mut test_stype = buf.style();
            test_stype.set_color(Color::Green).set_bold(true);
            if record.level().eq(&Error) {
                test_stype.set_color(Color::Red);
            }
            writeln!(buf, "{} {} {} {}",
                     grey_stype.value("["),
                     grey_stype.value("============"),
                     test_stype.value(record.args()),
                     grey_stype.value("============"),
            )
        });
    builder.build()
}

pub fn log_static_info(message: Arguments) {
    let record = Record::builder()
        .args(message)
        .build();
    LOGGER_STATIC_INFO.get_or_init(create_test_static_info_logger).log(&record);
}

pub fn log_test(message: Arguments) {
    let record = Record::builder()
        .args(message)
        .build();
    LOGGER_TESTING.get_or_init(create_testing_logger).log(&record);
}

pub fn log_error_test(message: Arguments) {
    let record = Record::builder()
        .level(Error)
        .args(message)
        .build();
    LOGGER_TESTING.get_or_init(create_testing_logger).log(&record);
}