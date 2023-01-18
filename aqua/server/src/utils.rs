// used for pretty printing multiple lines of text
use indoc::formatdoc;
use rocket::serde::{json::Json, Serialize};
use std::error::Error;
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::Write;
extern crate chrono;

use chrono::Local;

#[derive(Serialize, Debug)]
pub struct ResponseStructure<T> {
    status: u16,
    name: String,
    data: T,
}

pub struct ApiResponse {}

pub type ApiRes<T> = Json<ResponseStructure<T>>;

pub trait ApiStatuses {
    fn log_handler<T>(response: &ResponseStructure<T>) -> Result<(), Box<dyn Error>>;
    fn send_log<T>(response: &ResponseStructure<T>);
    fn success<T>(data: T) -> ApiRes<T>;
    fn error<T>(data: T) -> ApiRes<T>;
    fn not_found<T>(data: T) -> ApiRes<T>;
}

impl ApiStatuses for ApiResponse {
    fn log_handler<T>(response: &ResponseStructure<T>) -> Result<(), Box<dyn Error>> {
        let filename = "log.txt";
        let mut log_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(filename)
            .unwrap();
        // write to log file
        let date = Local::now();

        let log_addition = formatdoc! {"
              TIME: {time}
              STATUS: {status}
              STATUS_CODE: {status_code}
              -------------------------
        ",
        time = date.format("%Y-%m-%d - %H:%M:%S"),
        status = response.name.to_string(),
        status_code = response.status.to_string()
        };

        write!(log_file, "{log_addition}")?;

        Ok(())
    }
    fn send_log<T>(response: &ResponseStructure<T>) {
        let log_req = Self::log_handler(response);

        if log_req.is_err() {
            println!("[Internal API Issue]: logs not being sent to log.txt");
        }
    }
    fn success<T>(data: T) -> ApiRes<T> {
        let res = ResponseStructure {
            status: 200,
            name: String::from("Success"),
            data,
        };

        Self::send_log(&res);
        Json(res)
    }
    fn error<T>(data: T) -> ApiRes<T> {
        let res = ResponseStructure {
            status: 500,
            name: String::from("Internal Server Error"),
            data,
        };

        Self::send_log(&res);
        Json(res)
    }
    fn not_found<T>(data: T) -> ApiRes<T> {
        let res = ResponseStructure {
            status: 404,
            name: String::from("Not Found"),
            data,
        };

        Self::send_log(&res);
        Json(res)
    }
}
