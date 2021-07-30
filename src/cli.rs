//Run the command line interface for this app
use std::env::consts;

const VERSION: &str = env!("CARGO_PKG_VERSION");
static mut OS: String = String::new();


// Greet and get OS information
pub fn run() {

    set_os();
    greet();
    unsafe {
        println!("Running on OS: {}", OS.as_str());
    }
}

fn set_os() {
    unsafe {
        OS.push_str(consts::OS);
    }
}


fn greet() {

    art::draw_robot();

    println!("Welcome to the command line interface to download viper database files from the internet");
    println!("Currently running vesion {}", VERSION);

}

mod art {


    pub fn draw_robot() {

        let greet = r#"                           ___
                          |_|_|
                          |_|_|              _____
                          |_|_|     ____    |*_*_*|
                 _______   _\__\___/ __ \____|_|_   _______
                / ____  |=|      \  <_+>  /      |=|  ____ \
                ~|    |\|=|======\\______//======|=|/|    |~
                 |_   |    \      |      |      /    |    |
                  \==-|     \     |  2D  |     /     |----|~~/
                  |   |      |    |      |    |      |____/~/
                  |   |       \____\____/____/      /    / /
                  |   |         {----------}       /____/ /
                  |___|        /~~~~~~~~~~~~\     |_/~|_|/
                   \_/        |/~~~~~||~~~~~\|     /__|\
                   | |         |    ||||    |     (/|| \)
                   | |        /     |  |     \       \\
 _____  _          |_|        |     |  |     |     |
| ____|(_) _____   _____      |     |  |     |
|  _|  | |/ _ \ \ / / _ \     |     |  |     |
| |___ | | (_) \ V / (_) |    |_____|  |_____|
|_____|/ |\___/ \_/ \___/     |     |  |     |
     |__/                     (_____)  (_____)
__     ______  ____           |     |  |     |
\ \   / /  _ \| __ )          |     |  |     |
 \ \ / /| | | |  _ \          |/~~~\|  |/~~~\|
  \ V / | |_| | |_) |         /|___|\  /|___|\
   \_/  |____/|____/         <_______><_______>
 ____                      _                 _
|  _ \  _____      ___ __ | | ___   __ _  __| | ___ _ __
| | | |/ _ \ \ /\ / / '_ \| |/ _ \ / _` |/ _` |/ _ \ '__|
| |_| | (_) \ V  V /| | | | | (_) | (_| | (_| |  __/ |
|____/ \___/ \_/\_/ |_| |_|_|\___/ \__,_|\__,_|\___|_|}"#;

        println!("{}", greet);

    }



}
