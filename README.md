# rootkit-scanner
This is just something to help scan for rootkits taking advantage of rkhunter program. It really stemed from trying to lock down devices at home with so many security exploits being generated during the COVID-19 timeframe.

Inspirations:

* [https://slicedpi.net/2017/11/25/securing-your-pi/](https://slicedpi.net/2017/11/25/securing-your-pi/)

## Objective
Setup a Rust script that will call rkhunter (rootkit hunter) and then run in Jenkins on daily basis.

## Requirements

* Rust - [https://www.rust-lang.org/](https://www.rust-lang.org/)
* Jenkins - [https://www.jenkins.io/](https://www.jenkins.io/)
* rkhunter - [http://rkhunter.sourceforge.net/](http://rkhunter.sourceforge.net/)

All this will run on RaspberryPi setups, should run on other Linux OSes as well. 


