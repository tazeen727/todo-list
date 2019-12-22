#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::databases::rusqlite;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;

#[database("sqlite_tasks")]
struct DbConn(rusqlite::Connection);

#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    description: String,
    done: bool,
}

impl Task {
    fn all(conn: &rusqlite::Connection) -> Result<Vec<Task>, rusqlite::Error> {
        let mut stmt = conn.prepare("SELECT * FROM TASKS")?;

        let task_iter = stmt.query_map(&[], |row| Task {
            id: row.get("TASK_ID"),
            description: row.get("TASK_DESCRIPTION"),
            done: row.get("DONE"),
        })?;

        task_iter.collect()
    }
}

#[get("/tasks")]
fn tasks(conn: DbConn) -> Result<Json<Vec<Task>>, ()> {
    match Task::all(&*conn) {
        Ok(tasks) => Ok(Json(tasks)),
        Err(_) => Err(()),
    }
}

#[post("/task", format = "json", data = "<task>")]
fn register(conn: DbConn, task: Json<Task>) -> Result<Json<Task>, ()> {

}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .mount("/", routes![tasks, task])
        .attach(DbConn::fairing())
        .launch();
}
