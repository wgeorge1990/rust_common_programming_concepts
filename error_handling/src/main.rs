use std::io;
use std::io::Read;
use std::fs::File;


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
}

fn handling_with_closures() {
    let f = File::open("readusernamefromfile.txt").unwrap_or_else( |error| {
        if error.kind() == io::ErrorKind::NotFound {
            File::create("readusernamefromfile.txt").unwrap_or_else( |error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("readusernamefromfile.txt");

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
    let mut f = File::open("readusernamefromfile.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
