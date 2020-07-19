use std::process::Command;

fn main() {
    println!("Attempting to scan for rootkits with rkhunter");

    update();
    scan();

    println!("rootkit scan completed!")
}

fn update(){
   
   Command::new("sudo")
	.arg("rkhunter")
   	.arg("--update")
        .spawn()
        .expect("rkhunter update failed to start");
 	
}

fn scan(){
   Command::new("sudo")
	.arg("rkhunter")
	.arg("-c")
	.arg("--enable")
	.arg("all")
	.arg("--rwo")
	.arg("--sk")
	.spawn()
	.expect("rkhunter scan failed.");

}
