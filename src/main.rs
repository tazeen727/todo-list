#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::databases::rusqlite;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;

use rusqlite::Result;

#[database("sqlite_tasks")]
struct DbConn(rusqlite::Connection);

#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    description: String,
    done: bool,
}

impl Task {
    fn all(conn: &rusqlite::Connection) -> Result<Vec<Task>> {
        let mut stmt = conn.prepare("SELECT * FROM TASKS")?;

        let task_iter = stmt.query_map(&[], |row| Task {
            id: row.get("TASK_ID"),
            description: row.get("TASK_DESCRIPTION"),
            done: row.get("DONE"),
        })?;

        task_iter.collect()
    }

    fn find(
        conn: &rusqlite::Connection,
        id: u32,
    ) -> Result<Option<Task>> {
        let mut stmt = conn.prepare("SELECT * FROM TASKS WHERE TASK_ID = ?")?;

        let mut task_iter = stmt.query_map(&[&id], Self::mapper)?;

        task_iter.next().transpose()
    }

    fn insert(
        conn: &rusqlite::Connection,
        task: &Task,
    ) -> Result<()> {
        let mut stmt = conn.prepare("
            INSERT INTO TASKS
            VALUES (?, ?, ?)
        ")?;

        stmt.execute(&[&task.id, &task.description, &task.done])?;
        Ok(())
    }

    fn update(
        conn: &rusqlite::Connection,
        task: &Task,
    ) -> Result<()> {
        let mut stmt = conn.prepare("
            UPDATE TASKS
            SET TASK_DESCRIPTION = ?, DONE = ?
            WHERE TASK_ID = ?
        ")?;

        stmt.execute(&[&task.description, &task.done, &task.id])?;
        Ok(())
    }

    fn mapper(row: &rusqlite::Row) -> Task {
        Task {
            id: row.get("TASK_ID"),
            description: row.get("TASK_DESCRIPTION"),
            done: row.get("DONE"),
        }
    }
}

#[get("/tasks")]
fn tasks(conn: DbConn) -> Option<Json<Vec<Task>>> {
    Task::all(&*conn).ok().map(Json)
}

#[post("/task", format = "application/json", data = "<task>")]
fn upsert_task(conn: DbConn, task: Json<Task>) -> Option<Json<Task>> {
    Task::find(&*conn, task.id)
        .and_then(|task2| {
            match task2 {
                Some(_) => {
                    Task::update(&*conn, &*task)?;
                    Ok(task)
                },
                None => {
                    Task::insert(&*conn, &*task)?;
                    Ok(task)
                }
            }
        }).ok()
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .mount("/", routes![tasks, upsert_task])
        .attach(DbConn::fairing())
        .launch();
}
