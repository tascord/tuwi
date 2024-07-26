pub mod widgets;
pub use widgets::*;

#[macro_export]
macro_rules! ab {
    ($e: expr) => {
        Arc::new(Box::new($e))
    };
}