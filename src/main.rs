use std::io;
use std::process;
use rand::Rng;

fn main() {
	let mut num_arr: [usize; 16] = [0; 16];
	let mut zero_index: usize = 15;
	let mut move_count: usize = 0;
	let scramble_len = 2;
	let mut rng = rand::thread_rng();
	for i in 1..16 {
		num_arr[i-1] = i;
	}
	let mut curr_directions: [bool; 4];
	let running: bool = true;
	let mut scrambling: u8 = 0;
	let mut last_move: usize = 5;
    let mut states = Vec::new();
	loop {
		curr_directions = available_moves(zero_index);
		
		if running != true {
			break;
		}
		let up: bool;
		let down: bool;
		let left: bool;
		let right: bool;
		let mut quit: bool = false;
		if scrambling >= scramble_len {
			print_array(num_arr, move_count, last_move, (*states).to_vec(), quit);
	
			println!("Enter your move (w/a/s/d) or q to quit: ");
	
			let mut next_move = String::new();
	    	io::stdin()
		        .read_line(&mut next_move)
		        .expect("Failed to read line");
			next_move.pop();
			next_move = next_move.to_lowercase();
			
			let str_move: &str = &*next_move;
			let down_in: &str = "s";
			let up_in: &str = "w";
			let left_in: &str = "a";
			let right_in: &str = "d";
			let quit_in: &str = "q";
			up = str_move.eq(up_in);
			down = str_move.eq(down_in);
			left = str_move.eq(left_in);
			right = str_move.eq(right_in);
			quit = str_move.eq(quit_in);
			if quit {
				process::exit(1);
			}
			if up || down || left || right {
				move_count += 1;
			}
		} else {
			let mut rand_move: u8 = rng.gen_range(0, 4);
			let mut regen: bool = !curr_directions[rand_move as usize];
			loop {
				if !regen {
					break;
				}
				rand_move = rng.gen_range(0, 4);
				regen = !curr_directions[rand_move as usize];
			}
			if rand_move == 0 {
				up = true;
				down = false;
				left = false;
				right = false;
			} else if rand_move == 1 {
				up = false;
				down = true;
				left = false;
				right = false;
			} else if rand_move == 2 {
				up = false;
				down = false;
				left = true;
				right = false;
			} else {
				up = false;
				down = false;
				left = false;
				right = true;
			}
		}
		
		if up && curr_directions[0] {
			let temp: usize = num_arr[zero_index + 4];
			num_arr[zero_index + 4] = num_arr[zero_index];
			num_arr[zero_index] = temp;
			zero_index = zero_index + 4;
            if scrambling >= scramble_len {
                last_move = 0;
                states.push(num_arr);
            }
		} else if down && curr_directions[1] {
			let temp: usize = num_arr[zero_index - 4];
			num_arr[zero_index - 4] = num_arr[zero_index];
			num_arr[zero_index] = temp;
			zero_index = zero_index - 4;
			if scrambling >= scramble_len {
                last_move = 1;
                states.push(num_arr);
            }
		} else if left && curr_directions[2] {
			let temp: usize = num_arr[zero_index + 1];
			num_arr[zero_index + 1] = num_arr[zero_index];
			num_arr[zero_index] = temp;
			zero_index = zero_index + 1;
			if scrambling >= scramble_len {
                last_move = 2;
                states.push(num_arr);
            }
		} else if right && curr_directions[3] {
			let temp: usize = num_arr[zero_index - 1];
			num_arr[zero_index - 1] = num_arr[zero_index];
			num_arr[zero_index] = temp;
			zero_index = zero_index - 1;
			if scrambling >= scramble_len {
                last_move = 3;
                states.push(num_arr);
            }
		}

		if scrambling < scramble_len {
			scrambling = scrambling + 1;
		}
	}
}

fn find_next_move(num_arr: [usize; 16], zero_index: usize, move_count: usize, last_move: usize, last_move_solid: usize, states: Vec<[usize; 16]>) -> (usize, usize) {
	if is_solved(num_arr) {
		return(last_move, 0);
	}
	if move_count >= 10 {
        return (last_move, distance_to_sol(num_arr));
	}
	let mut test: [usize; 16] = num_arr;
	let clone: [usize; 16] = num_arr.clone();
	let mut zero: usize = zero_index;
	let curr_directions: [bool; 4] = available_moves(zero);
	let mut best: usize = usize::MAX;
	let mut best_move: usize = 5;
	let mut i = 0;
	while i < 4 {
		if curr_directions[i] {
			let num_solved: usize = get_num_solved(test);
			if i == 0 && last_move != opposite(i) && last_move_solid != opposite(i){
				let temp: usize = test[zero + 4];
				test[zero + 4] = test[zero];
				test[zero] = temp;
				zero = zero + 4;
			} else if i == 1 && last_move != opposite(i) && last_move_solid != opposite(i){
				let temp: usize = test[zero - 4];
				test[zero - 4] = test[zero];
				test[zero] = temp;
				zero = zero - 4;
			} else if i == 2 && last_move != opposite(i)  && last_move_solid != opposite(i){
				let temp: usize = test[zero + 1];
				test[zero + 1] = test[zero];
				test[zero] = temp;
				zero = zero + 1;
			} else if i == 3 && last_move != opposite(i) && last_move_solid != opposite(i){
				let temp: usize = test[zero - 1];
				test[zero - 1] = test[zero];
				test[zero] = temp;
				zero = zero - 1;
			}
            let mut notPast: bool = true;
            for arr in &states {
                if is_same(*arr, test) {
                    notPast = false;
                }
            }
			let mut distance_temp: usize = find_next_move(test, zero, move_count + 1, i, last_move_solid, (*states).to_vec()).1;
			if num_solved > get_num_solved(test) && distance_temp != usize::MAX {
				distance_temp += num_solved - get_num_solved(test);
			}
			if distance_temp != usize::MAX && distance_temp + move_count < best && i != opposite(last_move_solid) && notPast{
				best_move = i;
				best = distance_temp + move_count;
			}
			test = clone;
		}
		i += 1;
	}
	return (best_move, best);
}

fn get_num_solved(num_arr:[usize; 16]) -> usize {
	let mut count: usize = 0;
	for i in 0..14 {
		if num_arr[i] == i+1 {
			count += 1;
		}
	}
	return count;
}

fn available_moves(zero_index: usize) -> [bool; 4] {
	let mut curr_directions: [bool; 4] = [false, false, false, false];
	if (zero_index + 1) % 4 != 1 {
			curr_directions[3] = true;
	} else {
		curr_directions[3] = false;
	}

	if (zero_index + 1) % 4 != 0 {
		curr_directions[2] = true;
	} else {
		curr_directions[2] = false;
	}

	if ((zero_index as f64) + 1.0) / 4.0 > 1.0 {
		curr_directions[1] = true;
	} else {
		curr_directions[1] = false;
	}

	if ((zero_index as f64) + 1.0) / 4.0 <= 3.0 {
		curr_directions[0] = true;
	} else {
		curr_directions[0] = false;
	}
	return curr_directions;
}

fn is_solved(num_arr: [usize; 16]) -> bool{
	for i in 0..14 {
		if num_arr[i] > num_arr[i+1] {
			return false;
		}
	}
	return true;
}

fn is_same(num_arr: [usize; 16], test: [usize; 16]) -> bool{
    let mut out: bool = true;
	for i in 0..15 {
		if num_arr[i] != test[i] {
			out = false;
		}
	}
	return out;
}

fn opposite(num: usize) -> usize {
    if num == 0 {
        return 1
    } else if num == 1 {
        return 0
    } else if num == 2 {
        return 3
    } else if num == 3 {
        return 2
    }
    return 5
}

fn distance_to_sol(num_arr: [usize; 16]) -> usize {
	let mut dist: usize = 0;
	for i in 0..15 {
		if num_arr[i] != (i + 1) {
			dist += 1;
		}
	}
	return dist;
}

fn print_array(num_arr: [usize; 16], move_count: usize, last_move: usize, states: Vec<[usize; 16]>, quit: bool) {
	print!("\x1B[2J\x1B[1;1H");
	let mut out: String = "".to_string();
	let mut zero: usize = 0;
	for i in 0..16 {
		if num_arr[i] == 0 {
			zero = i;
			out = format!("{}{}", out, " ");
			if (i+1) % 4 == 0 {
				out = format!("{}\n", out);
			} else {
				out = format!("{}\t", out);
			}
		} else if (i+1) % 4 == 0 {
			out = format!("{}{}\n", out, format!("{}", num_arr[i]));
		} else {
			out = format!("{}{}\t", out, format!("{}", num_arr[i]));
		}
	}
	println!("{}",out);
	println!("Solved: {}\n", is_solved(num_arr));
	println!("Move Count: {}\n", move_count);
	if !is_solved(num_arr) {
		let mut move_out: String = "".to_string();
		if find_next_move(num_arr, zero, 0, last_move, last_move, (*states).to_vec()).0 == 0 {
			move_out = "w".to_string();
		} else if find_next_move(num_arr, zero, 0, last_move, last_move, (*states).to_vec()).0 == 1 {
			move_out = "s".to_string();
		} else if find_next_move(num_arr, zero, 0, last_move, last_move, (*states).to_vec()).0 == 2 {
			move_out = "a".to_string();
		} else if find_next_move(num_arr, zero, 0, last_move, last_move, (*states).to_vec()).0 == 3 {
			move_out = "d".to_string();
		}
		println!("Next Move: {} {}\n", move_out, find_next_move(num_arr, zero, 0, last_move, last_move, (*states).to_vec()).1);
	} else {
		print!("\x1B[2J\x1B[1;1H");
		println!("Congratulations! Your solution was {} moves long.", move_count);
		process::exit(0x0100);
	}
	if quit {
		print!("\x1B[2J\x1B[1;1H");
		println!("Thanks for playing!");
		process::exit(0x0100);
	}
}