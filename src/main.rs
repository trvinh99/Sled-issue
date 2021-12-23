use bastion::Bastion;
use ledb::Storage;
use smol::Timer;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Duration;
use std::time::SystemTime;
use sysinfo::{Disk, DiskExt, System, SystemExt};

pub const RECORD_DIR: &'static str = "/data/lexhub/record";

fn main() {
    Bastion::init();
    Bastion::start();

    // insert();
    // let record_folder = format!("{}/2021-12-08", RECORD_DIR);

    // std::fs::remove_dir_all(record_folder).unwrap();

    let frame_folder = "/home/lexhub/record/2021-12-22";

    if Path::new(&frame_folder).exists() {
        std::fs::remove_dir_all(frame_folder).unwrap();
    }

    Bastion::block_until_stopped();
}

fn insert() {
    let mut file = File::open("src/logo.png").unwrap();
    let mut contents = vec![];
    file.read_to_end(&mut contents).unwrap();

    for i in 1..=30 {
        let contents = contents.clone();
        let record_db_config = sled::Config::default()
            .path(format!("src/record/{}", i))
            //.cache_capacity(10 * 1024 * 1024)
            .mode(sled::Mode::HighThroughput);
        let record_db = record_db_config.open().unwrap();
        bastion::spawn!(async move {
            let mut i = 0;
            while i < 1000 {
                let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                    Ok(n) => n.as_nanos(),
                    Err(_) => panic!("SystemTime before UNIX EPOCH!"),
                };

                let _ = record_db.insert(now.to_string().as_bytes(), contents.to_vec());
                record_db.flush();

                i += 1;

                Timer::after(Duration::from_millis(200)).await;
            }
        });
    }
}
