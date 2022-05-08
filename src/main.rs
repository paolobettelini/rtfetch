use sysinfo::{{System, SystemExt}};

fn main() {
    let sys = System::new_all();

    let host_name = sys.host_name().unwrap();
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let kernel_version = sys.kernel_version().unwrap();
    let os_version = sys.os_version().unwrap();
    let uptime = sys.uptime();
    let cores = sys.physical_core_count().unwrap();
    let system_name = sys.name().unwrap();

    let result = include!(concat!(env!("OUT_DIR"), "/result.rs"));
    println!("{}", result);
}