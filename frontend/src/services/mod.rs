pub mod spotify;
pub mod steam;

#[macro_export]
macro_rules! route {
    ($path:expr) => {
        concat!(env!("BACKEND_URL"), $path)
    };
}
