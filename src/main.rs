use log::{debug, error, info, trace};
mod logging;
fn main() {
    logging::init_logger();
}
