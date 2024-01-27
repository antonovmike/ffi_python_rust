use std::{ffi::CStr, str::Utf8Error};

use sqlite::{State, Value};

#[no_mangle]
pub extern "C" fn print_person(
    name: *const std::os::raw::c_char,
    birth_year: *const std::os::raw::c_char,
) {
    let connection = sqlite::open("test.db").unwrap();

    let c_str_name: &CStr = unsafe { std::ffi::CStr::from_ptr(name) };
    let c_str_year: &CStr = unsafe { std::ffi::CStr::from_ptr(birth_year) };

    let result_name: Result<&str, Utf8Error> = c_str_name.to_str();
    let result_birth_year: Result<&str, Utf8Error> = c_str_year.to_str();
    let int_year = result_birth_year.unwrap().parse::<i64>().unwrap();

    write_db(&connection, result_name, int_year);

    for i in read_db(&connection) {
        println!("Name: {}, Birth Year: {}", i.0, i.1)
    }
}

fn write_db(connection: &sqlite::Connection, result_name: Result<&str, Utf8Error>, year: i64) {
    let query = "
            CREATE TABLE IF NOT EXISTS persons (name TEXT, birth INTEGER);
        ";
    connection.execute(query).unwrap();

    if let Ok(name) = result_name {
        let query = "INSERT INTO persons (name, birth) VALUES (:name, :birth)";
        let mut statement = connection.prepare(query).unwrap();

        statement.bind_iter::<_, (_, Value)>([
            (":name", name.into()), (":birth", year.into()),
        ]).unwrap();
        statement.next().unwrap();
    }
}

fn read_db(connection: &sqlite::Connection) -> Vec<(String, i64)> {
    let query = "
            CREATE TABLE IF NOT EXISTS persons (name TEXT, birth INTEGER);
            INSERT INTO persons VALUES ('Rita Levi-Montalcini', 1909);
            INSERT INTO persons VALUES ('Kurmanjan Datka', 1811);
        ";
    connection.execute(query).unwrap();

    let query = "SELECT * FROM persons WHERE birth > ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, 50)).unwrap();

    let mut collection: Vec<(String, i64)> = vec![];
    while let Ok(State::Row) = statement.next() {
        collection.push((
            statement.read::<String, _>("name").unwrap(),
            statement.read::<i64, _>("birth").unwrap(),
        ));
    }

    collection
}
