use std::env::args;
use std::fs;
use std::path::Path;
use std::io;
use std::process;

fn main() {
    let arguments: Vec<String> = args().collect();
    let mut path: String = String::new();

    let mut puzzle: Vec<u32> = Vec::new();  

    //Checking path to file is exist in arguments
    if arguments.len() == 2 {
        path = arguments[1].clone();
        println!("{:#?}", arguments[1]);
    }
    else if arguments.len() > 2 {
        println!("Too many arguments write path manually eg(C:/puzzle.txt) : ");

        //read input from keyboard
        io::stdin().read_line(&mut path)
            .expect("Can`t read line");

        //Cutting edge (\r\n)
        path = path.trim_end().to_string();
    }
    else {
        println!("Please write path to file eg(C:/puzzle.txt) :");

        //read input from keyboard
        io::stdin().read_line(&mut path)
            .expect("Can`t read line");

        //Cutting edge \r\n
        path = path.trim_end().to_string();
    }


    //Check that file exist
    if Path::new(&path).exists() {
        //Reading file
        let test: String = fs::read_to_string(path)
            .expect("File not readable");

        //parse file
        for c in test.split("\r\n") {
            puzzle.push(c.parse::<u32>().unwrap());
        }


    } else {
        println!("File does not exist at the provided path: {}", path);
        process::exit(0);
    }



    println!("Result:");
    println!("");


    //If file empty or can`t parse
    if puzzle.len() == 0 {
        println!("Your file empty there no puzzle");
        process::exit(0);
    }
    

    //print first puzzle
    let mut temp = puzzle.remove(0);
    print!("{}", temp / 100000);

    
    //print another puzzles
    loop {
        let first_two_digits = temp % 100;
        print!("{}", temp % 100_000 / 10);


        let mut end: bool = false;

        //if we don't have left over puzzle
        if puzzle.len() == 0 {
            print!("{}", temp % 10);
            break;
        }

        //finding puzzle
        let len = puzzle.len();
        for j in 0..(puzzle.len() - 1) {
            //compare last digits and if its last we stop
            if (puzzle[j] / 10_000) == first_two_digits && j == puzzle.len() {
                end = true;
                break;
            }
            

            //compare last digits
            if (puzzle[j] / 10_000) == first_two_digits {
                temp = puzzle.remove(j);
                break;
            }
        }

        //if puzzle in temp dont change and we not have duplicate we stop
        if len == puzzle.len() && !end {
            print!("{}", temp % 10);
            break;
        }
    }
}
