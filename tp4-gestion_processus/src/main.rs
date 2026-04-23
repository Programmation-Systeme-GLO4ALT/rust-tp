mod gestion;

use gestion::*;

fn main() {
    let mut pm = ProcessManager::new();

    // create init process
    let init = pm.create_process(
        String::from("init"),
        Priority::High,
        1024,
        None,
    );

    // create child process
    let bash = pm.create_process(
        String::from("bash"),
        Priority::Normal,
        4096,
        Some(init),
    );

    // change state of bash to running
    pm.change_state(bash, ProcessState::Running { cpu_id: 0 })
        .unwrap();

    // show summary
    pm.print_summary();

    // kill bash
    match pm.kill_process(bash) {
        Ok(code) => println!("bash terminated with code {}", code),
        Err(e) => eprintln!("Error: {}", e),
    }

    // show summary again
    pm.print_summary();
}