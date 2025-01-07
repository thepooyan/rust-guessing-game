use crate::util;

pub struct Game {
    trg: u32,
    l_range: u32,
    h_range: u32,
    br: bool,
    bs_count: u8
}

impl Game {
    pub fn new() -> Self {
        Game {
            trg: 12,
            l_range: 0,
            h_range: 100,
            br: false,
            bs_count: 0
        }
    }
    pub fn start(&mut self) {
        println!("starting game...");
        self.g_loop();
    }
    fn g_loop(&mut self) {
        loop {
            let a = format!("guess? ({}, {}) ", self.l_range, self.h_range);
            let res = crate::util::get_prompt(&a);
            self.analysis(res);
            if self.br {break}
        }
    }
    fn analysis(&mut self, res: String) {
        if let Ok(n) = res.trim().parse::<u32>() {
            if n == self.trg {return self.win()}
            if self.h_range < n || n < self.l_range {
                self.bs_count += 1;
                if self.bs_count == 3 {return self.freak_out();}
                return println!("Are you crazy?");
            }
            if self.trg < n {
                println!("guess lower");
                return self.h_range = n-1;
            }
            if n < self.trg {
                println!("guess higher");
                return self.l_range = n+1;
            }
        } else {
            match res.as_str() {
                "cls" => util::cls(),
                &_ => println!("number not valid"),
            }
        }
    }
    fn win(&mut self) {
        println!("Congrats! you won!");
        self.br = true;
        Game::replay_prompt();
    }
    fn replay_prompt() {
        let res = util::get_prompt("Play again? ");
        match res.as_str()  {
            "y" | "yes" | "ok" => Game::new().start(),
            &_ => println!("Goodbye!")
        } 
    }
    fn freak_out(&mut self) {
        println!("Ok i've had it. you don't deserve to play the game.");
        println!("you crazy :/");
        self.br = true;
    }
}
