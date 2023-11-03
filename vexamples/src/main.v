module main

import os

fn main() {
	print_header('Welcome to World!')
	println('PID number: ${os.getpid()}')
}
