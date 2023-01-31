#![forbid(unsafe_code)]

use std::time::Duration;
// Main Library file
use sabaton_mw::{error::MiddlewareError, NodeBuilder};
use tracing::{debug, info, span, Level};

pub fn example_node_main() -> Result<(), MiddlewareError> {
    let node = NodeBuilder::default()
        //.multi_threaded()  Enable this if you want a multi-threaded runtime
        //.with_namespace("namespace".into())
        //.with_num_workers(4)    // Number of work threads. Fixed to 1 for single threaded runtime.
        .build("example-node".to_owned())
        .expect("Node creation error");

    let res = node.spin(|| {
        span!(target: "MAIN", Level::TRACE, "Application Main Loop");
        info!("Application Main Loop Started with tick interval 100mS");

        let mut ticker = tokio::time::interval(Duration::from_millis(100));

        let _task = tokio::spawn(async move {
            //loop {
            //let _ = ticker.tick().await;
            //debug!("Tick");
            //}
            let device_unique_id = get_device_unique_id().unwrap();
            println!("device_unique_id:{}", device_unique_id);
        });
    });

    res
}

// Returns the current version of the target
pub fn get_device_unique_id() -> Result<String, std::io::Error> {
    let device_unique_id = std::fs::read_to_string("/sys/firmware/devicetree/base/serial-number")?;

    Ok(device_unique_id)
}
