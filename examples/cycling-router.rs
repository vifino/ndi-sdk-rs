// NDI Router that cycles through all sources switches every 5 seconds.
use ndi_sdk::{FindInstance, RouteInstance, Source};
use std::{thread::sleep, time::Duration};

fn main() {
    let mut fi = FindInstance::create(None).unwrap();
    let my_name = "Source Scanning Router";
    let ri = RouteInstance::create(my_name, &vec!["Public"]).unwrap();

    loop {
        fi.wait_for_sources(1000).unwrap();
        let some_sources: Vec<Source> = fi.get_current_sources().unwrap();
        println!("Found {} sources", some_sources.len());
        for source in some_sources {
            let is_me =
                source.ndi_name.contains(my_name) && source.url_address.starts_with("127.0.0.1");
            if !is_me {
                let source = source.clone();
                println!("Name: {}\nURL: {}\n", source.ndi_name, source.url_address);
                ri.change(&source).unwrap();
                sleep(Duration::from_secs(5));
            }
        }
    }
}
