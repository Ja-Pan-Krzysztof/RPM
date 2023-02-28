use models::connection::create_table;


mod password_generator;
mod parser;
mod models;


fn main() {
    create_table();

    parser::arg_parser::options();
}
