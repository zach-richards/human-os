mod system;

use crate::system::sys_stats::SharedSysStats;

fn print_sys_stats(sys_stats: SharedSysStats) {
    println!("SharedSysStats\n--------------------------");
    println!("TIMING");
    println!("last_activity: {}", SharedSysStats.last_activity);
    println!("idle_since: {}", SharedSysStats.idle_since);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        print_sys_stats(SharedSysStats);
    }
}
