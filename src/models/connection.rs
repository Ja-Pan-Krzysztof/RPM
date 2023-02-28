use sqlite3::Connection;


fn connect() -> sqlite3::Result<Connection> {
    let conn = Connection::open("database.db");

    conn
}

pub fn create_table() {
    match connect().unwrap().execute("CREATE TABLE IF NOT EXISTS passwords (id	INTEGER NOT NULL, user TEXT, site TEXT, password	TEXT NOT NULL, PRIMARY KEY(id AUTOINCREMENT));") {
        Ok(_) => {},
        Err(error) => println!("{}", error),
    };

}

pub fn add_new_password(user: String, site: String, password: String) {
    match connect().unwrap().execute(&format!("INSERT INTO passwords(user, site, password) VALUES('{}', '{}', '{}')", user, site, password)) {
        Ok(_) => println!("[Ok] Add new password"),
        Err(error) => println!("{}", error),
    };
}
