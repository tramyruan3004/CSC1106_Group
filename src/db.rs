use sqlx::{SqlitePool, sqlite::SqliteQueryResult};

pub async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS bugs (
            bug_id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            reported_by TEXT NOT NULL,
            reported_at DATETIME NOT NULL,
            type TEXT,
            severity TEXT NOT NULL,
            progress_status TEXT NOT NULL DEFAULT 'Open',
            project_id TEXT,
            developer_id TEXT
        )"
    ).execute(pool).await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS projects (
            project_id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            created_by TEXT NOT NULL,
            created_at DATETIME NOT NULL,
            users_list TEXT NOT NULL
        );"
    ).execute(pool).await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            user_id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            role TEXT NOT NULL,
            username TEXT NOT NULL,
            hashed_password TEXT NOT NULL
        );"
    ).execute(pool).await?;
    Ok(())
}



// CREATE TABLE IF NOT EXISTS projects (
//             project_id INTEGER PRIMARY KEY AUTOINCREMENT,
//             name TEXT NOT NULL,
//             description TEXT NOT NULL,
//             created_by TEXT NOT NULL,
//             created_at DATETIME NOT NULL,
//             users_list TEXT NOT NULL,
//         );
//         CREATE TABLE IF NOT EXISTS users (
//             user_id TEXT PRIMARY KEY,
//             name TEXT NOT NULL,
//             role TEXT NOT NULL,
//             username TEXT NOT NULL,
//             hashed_password TEXT NOT NULL,
//         );


// pub async fn insert_bug(pool: &SqlitePool, bug: NewBug) -> Result<Bug, sqlx::Error> {
//     let rec = sqlx::query_as::<_, Bug>(
//         "INSERT INTO bugs (title, description, reported_by, severity, status) 
//          VALUES (?, ?, ?, ?, 'Open')
//          RETURNING *"
//     )
//     .bind(bug.title)
//     .bind(bug.description)
//     .bind(bug.reported_by)
//     .bind(bug.severity)
//     .fetch_one(pool)
//     .await?;

//     Ok(rec)
// }

// pub async fn get_all_bugs(pool: &SqlitePool) -> Result<Vec<Bug>, sqlx::Error> {
//     sqlx::query_as::<_, Bug>("SELECT * FROM bugs").fetch_all(pool).await
// }

// pub async fn get_bug_by_id(pool: &SqlitePool, id: i64) -> Result<Bug, sqlx::Error> {
//     sqlx::query_as::<_, Bug>("SELECT * FROM bugs WHERE bug_id = ?")
//         .bind(id)
//         .fetch_one(pool)
//         .await
// }

// pub async fn update_bug(pool: &SqlitePool, id: i64, update: UpdateBug) -> Result<Bug, sqlx::Error> {
//     let current = get_bug_by_id(pool, id).await?;
//     let new_title = update.title.unwrap_or(current.title);
//     let new_desc = update.description.unwrap_or(current.description);
//     let new_severity = update.severity.unwrap_or(current.severity);
//     let new_status = update.status.unwrap_or(current.status);
//     let new_dev = update.developer_id.or(current.developer_id);
//     let new_proj = update.project.or(current.project);

//     sqlx::query_as::<_, Bug>(
//         "UPDATE bugs SET title = ?, description = ?, severity = ?, status = ?, developer_id = ?, project = ? 
//          WHERE bug_id = ? RETURNING *"
//     )
//     .bind(new_title)
//     .bind(new_desc)
//     .bind(new_severity)
//     .bind(new_status)
//     .bind(new_dev)
//     .bind(new_proj)
//     .bind(id)
//     .fetch_one(pool)
//     .await
// }

// pub async fn delete_bug(pool: &SqlitePool, id: i64) -> Result<SqliteQueryResult, sqlx::Error> {
//     sqlx::query("DELETE FROM bugs WHERE bug_id = ?")
//         .bind(id)
//         .execute(pool)
//         .await
// }
