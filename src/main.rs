use std::process::Command;

fn main() {
    println!("Attempting to scan for rootkits with rkhunter");

    update();
	scan();
}

fn update() {
  	let mut rkh  = Command::new("rkhunter");

	println!("");
	println!("Attempting to update rkhunter...");
	rkh.arg("--update");
	rkh.status().expect("process failed to execute");

	println!();
	println!("rkhunter update completed!");
 	
}

fn scan() {
	let mut rkh  = Command::new("rkhunter");

	println!("");
	println!("Attempting to rootkit scan with rkhunter...");
	// Execute `ls` in the current directory of the program.
	
	rkh.arg("-c");
	rkh.arg("--enable");
	rkh.arg("all");
	rkh.arg("--rwo");
	rkh.arg("--sk");
	rkh.status().expect("rkhunter failed to execute!");

	println!();
	println!("rootkit scan completed!");

}
