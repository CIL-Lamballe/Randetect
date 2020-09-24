#!/bin/bash
arch=('aarch64' 'x86_64')
printf "Check code base:\n"
if ! (cargo fmt && cargo clippy --all-targets --all-features -- -D clippy::pedantic)
then
	tput bold
	tput setaf 196
	printf "Code is not clean, Randetect will not compile.\nPlease check the code!\n"
	tput init
	exit 1
fi
for a in ${arch[@]}
do
	printf "Compiling for => "; tput setaf 93; tput bold printf "${a}"; tput init; printf "\n";
	if (cross build --target=${a}-unknown-linux-musl --release &&
		cp target/${a}-unknown-linux-musl/release/randetect randetect_${a}-musl)
	then
		tput bold
		tput setaf 46
		printf "Randetect_${a} successfully compiled!\n"
		tput init
	else
		tput bold
		tput setaf 196
		printf "Could not compile randetect for ${a}. Build failed!\n"
		tput init
	fi
done
