use bastion::Bastion;
use smol::Timer;
use std::fs::File;
use std::io::Read;
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

    let system = System::new_all();

    let disk_info = system.disks();

    let mut space = 0;
    let mut avail = 0;

    for disk in disk_info {
        println!("NAME: {:?}", disk.mount_point());
        println!("TYPE: {:?}", disk.type_());
        space += disk.total_space();
        avail += disk.available_space();
    }

    println!("SPACE: {}", space);
    println!("AVAIL: {}", avail);

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
