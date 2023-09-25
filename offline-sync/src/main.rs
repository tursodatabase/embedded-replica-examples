use libsql::Database;
use libsql_replication::{Frames, TempSnapshot};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // the path to a SQLite file.
    let db = Arc::new(Mutex::new(Database::open_with_local_sync("test.db").await.unwrap()));
    let conn = db.lock().unwrap().connect().unwrap();

    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: {} <snapshots path>", args[0]);
        return Ok(());
    }
    // The snapshots path is where you would dump new snapshots into the airgapped system
    // through any method of your choosing.
    let snapshots_path = args.get(1).unwrap();

    // The sync loop. You would implement this logic. In here, we are syncing every second
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3));

        let mut snapshot_paths: Vec<_> = std::fs::read_dir(snapshots_path).unwrap().map(|r| r.unwrap()).collect();
        snapshot_paths.sort_by_key(|dir| dir.path());

        let mut applied_total = 0;
        for snapshot_path in snapshot_paths {
            let db = db.clone();
            let snapshot_path = snapshot_path.path();
            let snapshot = TempSnapshot::from_snapshot_file(snapshot_path.as_ref())?;
            let applied = tokio::task::spawn_blocking(move || {
                db.lock().unwrap().sync_frames(Frames::Snapshot(snapshot))
            })
            .await??;
            applied_total += applied;
        }

        println!("Applied {} frames", applied_total);

        let mut rows = match conn.query("SELECT u.name, k.expired FROM users u JOIN keycards as k ON u.user_id = k.user_id", ()).await {
            Ok(x) => x,
            Err(_) => { println!("Empty database"); continue; },
        };

        while let Ok(Some(row)) = rows.next() {
            println!(
                "User {} keycard is expired: {}",
                row.get::<String>(0).unwrap(),
                if row.get::<i32>(1).unwrap() == 1 { "yes" } else { "no" }
            );
        }
    }
}
