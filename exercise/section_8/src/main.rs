use std::{fs::rename, fs::File, io::Error, io::ErrorKind};
fn main() {
    // panic!("Panicked here!!!")

    let vec = vec![1];
    // vec[10];

    let file = File::open("error.tx");
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("error.txt") {
                Ok(file_created) => file_created,
                Err(err) => panic!("cannot create the file!"),
            },
            _ => panic!("It was some other error"),
        },
    };

    let file2 = File::open("error.txt").expect("error: file not found error");

    let test = open_file();
    test.unwrap();
    let rename_m = rename_file();
    rename_m.unwrap();
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn open_file() -> Result<File, Error> {
    let file = File::open("error.txt")?;
    Ok(file)
}

fn rename_file() -> Result<(), Error> {
    let file = rename("error3.txt", "error.txt")?;
    Ok(file)
}
