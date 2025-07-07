This is a simple, fast, and lightweight network scanner built in Rust. It pings all IP addresses in a local subnet (192.168.1.0/24 by default), identifies their live hosts, and attempts to get
their hostnames using 'nbtstat' WINDOWS ONLY. The requirements for using this program are just having Rust installed, using a Windows OS, and having an internet connection. To use this program, simply clone the repo and run
'cargo run' in terminal and you should see devices connected to your network start to pop up (and press Ctrl + C to terminate).
DISCLAIMER: This tool is intended for educational and personal network scanning only.
Do not use it on networks you don’t own or have permission to scan.

