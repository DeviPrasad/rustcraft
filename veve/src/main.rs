use std::fs::File;
use std::io::Read;

use std::convert::TryInto;

use dirs;

const VEVE_CONFIG_BASE    : &str = ".veve";
const VEVE_CONFIG_STARTUP : &str = "startup";
const VEVE_CONFIG         : &str = "config";

const VEVE_CONFIG_MODE_DEV   : &str = "dev";
const VEVE_CONFIG_MODE_STAGE : &str = "staged";
const VEVE_CONFIG_MODE_PROD  : &str = "production";

fn read_config_keyvals(kvbuf: &str) -> bool {
    println!();

    let lines = kvbuf.lines();
    let mut count = 0;
    for ln in lines {
        let mut itkv = ln.split("=");

        if let (Some(key), Some(val)) = (itkv.next(), itkv.next()) {
            if key.len() > 0 && val.len() > 0 {
                println!("{:?} {:?}", key, val);
                count += 1;
            }
        }
    }

    count > 0
}

fn read_config_file(mode : &str) -> bool {
    let mut result = false;

    // std::path::PathBuf;
    let mut pathbuf = dirs::home_dir().unwrap();

    pathbuf.push(VEVE_CONFIG_BASE);
    pathbuf.push(VEVE_CONFIG_STARTUP);
    pathbuf.push(VEVE_CONFIG);
    pathbuf.push(mode);

    let st = pathbuf.as_os_str();
    println!("config file path {:?}", st);

    if let Ok(mut f) = File::open(pathbuf.into_os_string()) {
        if let Ok(md) = f.metadata() {
            if md.is_file() && md.len() > 0 {
                let mut kvbuf = String::with_capacity(md.len().try_into().unwrap());
                f.read_to_string(&mut kvbuf).unwrap();
                drop(f);
                result = read_config_keyvals(&kvbuf);
            }
        }
    }

    return result;
}

fn veve_init(mode : &str) -> Result<bool, bool> {
    if read_config_file(mode) { return Ok(true); }
    return Err(false);
}


fn main() {
    println!();
    match veve_init(VEVE_CONFIG_MODE_DEV) {
        Ok(_)  => { println!(); println!("veve: init passed."); }
        Err(_) => { println!(); println!("\nveve: initialization failed. Quitting...."); }
    }
    println!();
}
