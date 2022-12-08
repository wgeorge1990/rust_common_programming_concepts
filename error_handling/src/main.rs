use std::io;
use std::io::Read;
use std::fs;


fn main() {
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     },
    // };

    handling_with_closures();

    println!("{:?}",read_username_from_file().expect("Something went wrong"));
    // vs using ? as shortcut for propogating errors: way shorter and cleaner method.
    println!("{:?}",shortcut_for_propogating_errors().expect("Something went wrong"));
    // chaining and propogating
    println!("{:?}", chaining_and_propogating().expect("expect"));

    println!("{}",chaining_reduced().expect("expect error, run backtrace to find code break"));
}

fn handling_with_closures() {
    let f = fs::File::open("readusernamefromfile.txt").unwrap_or_else( |error| {
        if error.kind() == io::ErrorKind::NotFound {
            fs::File::create("readusernamefromfile.txt").unwrap_or_else( |error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = fs::File::open("readusernamefromfile.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }

}

fn shortcut_for_propogating_errors() -> Result< String, io::Error> {
    let mut f = fs::File::open("readusernamefromfile.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn chaining_and_propogating() -> Result<String, io::Error> {
    let mut s = String::new();
    fs::File::open("readusernamefromfile.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn chaining_reduced() -> Result<String, io::Error> {
    fs::read_to_string("readusernamefromfile.txt")
}