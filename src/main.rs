mod db;
mod models;
mod repository;

use db::create_database;
use repository::user_repository::UserRepository;

fn main() -> rusqlite::Result<()> {
    let conn = create_database()?;
    let repo = UserRepository::new(&conn);

    repo.create_table()?;

    let u1 = repo.create("Oscar Cervantes")?;
    let u2 = repo.create("Ada Lovelace")?;

    println!("Usuarios creados:");
    println!("  {:?}", u1);
    println!("  {:?}", u2);

    if let Some(user) = repo.find_by_id(u1.id)? {
        println!("Encontrado por ID: {:?}", user);
    }

    println!("Todos los usuarios:");
    for u in repo.find_all()? {
        println!("  {:?}", u);
    }

    repo.update_name(u2.id, "Ada L.")?;
    repo.delete(u1.id)?;

    println!("Después de update/delete:");
    for u in repo.find_all()? {
        println!("  {:?}", u);
    }

    Ok(())
}






/* use rusqlite::Connection;

fn main() {
    let conn = create_database();
    create_table(&conn);

     //insert_user(&conn,"Oscar Cervantes");

    let user = get_user(&conn,1);
    println!("User: {:?}", user); 
}

fn get_user(conn: &Connection, user_id: i32) -> Result<String, rusqlite::Error> {
    let mut stm =conn.prepare("SELECT name FROM users WHERE id = ?1")?;
    let users=stm.query_map([user_id], |row| {
        let name: String = row.get(0)?;
        Ok(name)
    })?;
    for user in users{
        return Ok(user.unwrap());
    }
    Ok("No se encontro el usuario".to_string())
}
fn insert_user(conn: &Connection, user:&str) {
    let result = conn.execute(
        "INSERT INTO users (name) VALUES (?1)",[user],
    );
        
     match result {
        Ok(_)=> {
            println!("Usuario fue creado con exito");
        },
        Err(e) => panic!("Error No se pudo crear un usuario, error : {}", e),
    }
}
//ORM = object relational mapping
fn create_table(conn: &Connection) {
    let result = conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        )",
        [],
    );

     match result {
        Ok(_)=> {
            println!(" Tabla fue creada con exito");
        },
        Err(e) => panic!("Error base de datos no se pudo abrir, error : {}", e),
    }
}

fn create_database() -> Connection {
    let result = Connection::open("users.db");

    match result {
        Ok(_)=> {
            println!("Database fue creada con exito");
            return result.unwrap();
        },

        Err(e) => panic!("Error base de datos no se pudo abrir, error : {}", e),
    }
}
    */