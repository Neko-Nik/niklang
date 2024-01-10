mod interpreter;
mod lexer;
mod expressions;


fn main() {
    // If there are arguments passed, run them
    // Currently only file path are supported or a project folder path
    // If folder path is passed, run "main.nikl"
    // If file path is passed, run it
    // If no file is found, print error

    // Get arguments
    let args: Vec<String> = std::env::args().collect();

    // If one argument is passed
    if args.len() == 2 {
        // Get argument
        let arg: &String = &args[1];
        // Interpret the file
        interpreter::run_file::run_file(arg);
        
    } else if args.len() > 2 {
        // TODO: If the args is of length 3, check if the second argument is "-o" and the third argument is a file path
        // If so, compile the project and output the binary to the file path
        // Print error
        println!("Error: Too many arguments");
    } else {
        // If no arguments are passed, start the REPL (Read-Eval-Print-Loop)
        interpreter::repl::start_repl();
    }
}
