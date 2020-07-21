use std::process::Command;

// Rust program that will do a rootkit scan with
// the command rkhunter
fn main(){

	// Process updating rkhunter to latest rootkit check
	println!();
	println!("Attempting to update rkhunter...");
	let command = "rkhunter"; // Not using mut because var will get used again
	let mut options = ["--update"].to_vec(); // need to specify that its a Vec with .to_vec()
	let  expect_msg = command.to_owned() + " command failed to process!"; // Not using mut because var will get used again
	run_command(command, options, &expect_msg);
	println!();
	println!("rkhunter update completed!");

	// Process rootkit scan with rkhunter
	println!();
	println!("Attempting to rootkit scan with rkhunter...");
	// no need for command because we are using rkhunter again set previously
	// no need for expect_msg because we are using the previously set one again
	options = ["-c","--enable","all","--rwo","--sk"].to_vec(); // options are changing so mutable comes in handy
	run_command(command, options, &expect_msg);
	println!();
	println!("rootkit scan completed!");
}

// Will run the command and parameters you pass it
// It expect the parameters 
// command - command you want to run
// options - options for the command
// expect_msg - message for .expect()
fn run_command(command: &str, options: Vec<&str>, expect_msg: &str){

	let mut runner = Command::new(command); // making a mutable because it might need to add options

	if options.len() > 0 {
		// Only add options if options are greater than 0
		for opt in options {
			runner.arg(opt);
		}
	}
	
	// execute the command and generate error message if it fails
	runner.status().expect(expect_msg);
}

