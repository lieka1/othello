mod othello;

fn parseInput(input:  &str) -> Option<[i32; 2]>{
    let mut pos: [i32; 2] = [0; 2];
        
    let mut splited_input = input.split(",");

    for i in 0..2{
        let ele = splited_input.next();        
        match ele {
            Some(tar_str) => {
                let my_int = tar_str.parse::<i32>();
                match my_int {
                    Ok(parsed_str)=>{
                        pos[i] = parsed_str;
                    }
                    ,
                    Err(_)=> return None,
                }
            }
            ,
            None  => return None,
        };
    }

    let ele = splited_input.next();   
    match ele {
        Some(_) => return None,
        None => return Some(pos),
    };
}

fn main() {
    let mut game = othello::GameState::new();
    let mut do_exit = false;
    let mut line = String::new();
    game.print_board();

    while !do_exit {
        std::io::stdin().read_line(&mut line).unwrap();
        
        let val_parsed = parseInput(line.trim());
        
        match val_parsed {
            Some(parsed_arr) =>{
                game.play(game.current_player, parsed_arr[0], parsed_arr[1]);
            },
            None=>{
                if line.trim() == "q" {
                    do_exit = true;
                }
                println!("invalid move");
            }
        }
        
        line.clear();
        game.print_board();
        println!("current player : {:?}", game.current_player);
    }
   

}
