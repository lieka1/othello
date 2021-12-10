use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
enum GamePositionState {
    Empty = 0,
    Black = 1,
    White = 2,
}

impl ops::Not for GamePositionState {
    type Output = GamePositionState;

    fn not(self) -> GamePositionState {
        if self == GamePositionState::White {
            return GamePositionState::Black;
        }
        if self == GamePositionState::Black {
            return GamePositionState::White;
        }

        GamePositionState::Empty
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GamePlayer {
    White = 0,
    Black = 1,
}

impl ops::Not for GamePlayer {
    type Output = GamePlayer;
    fn not(self) -> GamePlayer {
        if self == GamePlayer::White {
            return GamePlayer::Black;
        }
        if self == GamePlayer::Black {
            return GamePlayer::White;
        }

        panic!("invalid player");
    }
}

impl GamePlayer {
    fn get_chess_color(self) -> GamePositionState {
        if self == GamePlayer::White {
            return GamePositionState::White;
        }
        if self == GamePlayer::Black {
            return GamePositionState::Black;
        }

        panic!("invalid player");
    }
}

#[derive(Debug)]
pub struct GameState {
    game_board: [GamePositionState; 64],
    pub current_player: GamePlayer,
}

impl GameState {
    pub fn print_board(&self) {
        println!(
            " [    0,     1,     2,     3,     4,     5,     6,     7]");
        for i in 0..8 {
            println!(
                "{}[{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}]",
                i,
                self.game_board[GameState::pos_to_arr_pos(0, i)],
                self.game_board[GameState::pos_to_arr_pos(1, i)],
                self.game_board[GameState::pos_to_arr_pos(2, i)],
                self.game_board[GameState::pos_to_arr_pos(3, i)],
                self.game_board[GameState::pos_to_arr_pos(4, i)],
                self.game_board[GameState::pos_to_arr_pos(5, i)],
                self.game_board[GameState::pos_to_arr_pos(6, i)],
                self.game_board[GameState::pos_to_arr_pos(7, i)],
            )
        }
    }

    

    pub fn play(&mut self, player: GamePlayer, x: i32, y: i32) -> bool {
        if player != self.current_player {
            return false;
        }
        if !self.do_move(x, y) {
            return false;
        }

        self.current_player = !self.current_player;

        return true;
    }

    fn pos_to_arr_pos(x: i32, y: i32) -> usize {
        (x + 8 * y) as usize
    }

    fn do_move(&mut self, x: i32, y: i32) -> bool {
        let mut func_success = false;
        let current_player_color = self.current_player.get_chess_color();

        if self.game_board[GameState::pos_to_arr_pos(x, y)] != GamePositionState::Empty {
            return false;
        }

        let mut tar_y_index = y + 1;
        let mut tar_x_index = x;
        // downward
        if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
            == !current_player_color
        {
            for i in 2..8 {
                tar_y_index = y + i;

                if tar_y_index > 8 {
                    break;
                }

                if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
                    == current_player_color
                {
                    for j in 1..(i + 1) {
                        tar_y_index = y + i - j;
                        self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)] =
                            current_player_color;
                    }

                    func_success = true;
                    break;
                }
                else if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
                    == GamePositionState::Empty
                {
                    break;
                }
            }
        }

        // upward
        tar_y_index = y - 1;
        if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
            == !current_player_color
        {
            for i in 2..8 {
                tar_y_index = y - i;

                if tar_y_index < 0 {
                    break;
                }

                if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
                    == current_player_color
                {
                    for j in 1..(i + 1) {
                        tar_y_index = y - i + j;
                        self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)] =
                            current_player_color;
                    }

                    func_success = true;
                    break;
                }
                else if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
                    == GamePositionState::Empty
                {
                    break;
                }
            }
        }

        // right
        tar_x_index = x + 1;
        tar_y_index = y;
        if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
            == !current_player_color
        {
            for i in 2..8 {
                tar_x_index = x + i;

                if tar_x_index > 8 {
                    break;
                }

                if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
                    == current_player_color
                {
                    for j in 1..(i + 1) {
                        tar_x_index = x + i - j;
                        self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)] =
                            current_player_color;
                    }

                    func_success = true;
                    break;
                } else if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
                    == GamePositionState::Empty
                {
                    break;
                }
            }
        }

        // left
        tar_x_index = x - 1;
        tar_y_index = y;
        if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
            == !current_player_color
        {
            for i in 2..8 {
                tar_x_index = x - i;

                if tar_x_index < 0 {
                    break;
                }

                if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
                    == current_player_color
                {
                    for j in 1..(i + 1) {
                        tar_x_index = x - i + j;
                        self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)] =
                            current_player_color;
                    }

                    func_success = true;
                    break;
                }
                else if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
                    == GamePositionState::Empty
                {
                    break;
                }
            }
        }

        // top left
        tar_x_index = x - 1;
        tar_y_index = y - 1;
        if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
            == !current_player_color
        {
            for i in 2..8 {
                tar_x_index = x - i;
                tar_y_index = y - i;

                if tar_x_index < 0 || tar_y_index < 0 {
                    break;
                }

                if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
                    == current_player_color
                {
                    for j in 1..(i + 1) {
                        tar_x_index = x - i + j;
                        tar_y_index = y - i + j;
                        self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)] =
                            current_player_color;
                    }

                    func_success = true;
                    break;
                }
                else if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
                    == GamePositionState::Empty
                {
                    break;
                }
            }
        }

        // bottom right
        tar_x_index = x + 1;
        tar_y_index = y + 1;
        if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
            == !current_player_color
        {
            for i in 2..8 {
                tar_x_index = x + i;
                tar_y_index = y + i;

                if tar_x_index > 8 || tar_y_index > 8 {
                    break;
                }

                if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
                    == current_player_color
                {
                    for j in 1..(i + 1) {
                        tar_x_index = x + i - j;
                        tar_y_index = y + i - j;
                        self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)] =
                            current_player_color;
                    }

                    func_success = true;
                    break;
                }
                else if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
                    == GamePositionState::Empty
                {
                    break;
                }
            }
        }

        // bottom left
        tar_x_index = x - 1;
        tar_y_index = y + 1;
        if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
            == !current_player_color
        {
            for i in 2..8 {
                tar_x_index = x - i;
                tar_y_index = y + i;

                if tar_x_index < 0 || tar_y_index > 8 {
                    break;
                }

                if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
                    == current_player_color
                {
                    for j in 1..(i + 1) {
                        tar_x_index = x - i + j;
                        tar_y_index = y + i - j;
                        self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)] =
                            current_player_color;
                    }

                    func_success = true;
                    break;
                }
                else if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
                    == GamePositionState::Empty
                {
                    break;
                }
            }
        }

        // top right
        tar_x_index = x + 1;
        tar_y_index = y - 1;
        if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
            == !current_player_color
        {
            for i in 2..8 {
                tar_x_index = x + i;
                tar_y_index = y - i;

                if tar_x_index > 8 || tar_y_index < 8 {
                    break;
                }

                if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
                    == current_player_color
                {
                    for j in 1..(i + 1) {
                        tar_x_index = x + i - j;
                        tar_y_index = y - i + j;
                        self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)] =
                            current_player_color;
                    }

                    func_success = true;
                    break;
                }
                else if self.game_board[GameState::pos_to_arr_pos(tar_x_index, tar_y_index)]
                    == GamePositionState::Empty
                {
                    break;
                }
            }
        }

        return func_success;
    }

    pub fn new() -> GameState {
        let mut board = [GamePositionState::Empty; 64];

        board[3 + 3 * 8] = GamePositionState::White;
        board[4 + 3 * 8] = GamePositionState::Black;
        board[3 + 4 * 8] = GamePositionState::Black;
        board[4 + 4 * 8] = GamePositionState::White;
        GameState {
            game_board: board,
            current_player: GamePlayer::Black,
        }
    }
}
