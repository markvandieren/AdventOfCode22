today:
	clear
	cargo run --release --bin day13

test_and_run: test run
	
test:
	cargo nextest run

run:
	cargo run --bin day1
	cargo run --bin day2
	cargo run --bin day3
	cargo run --bin day4
	cargo run --bin day5
	cargo run --bin day6
	cargo run --bin day7
	cargo run --bin day8
	cargo run --bin day9
	cargo run --bin day10
	cargo run --bin day11
	cargo run --bin day12
	cargo run --bin day13
