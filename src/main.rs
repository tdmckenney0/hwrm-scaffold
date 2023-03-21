pub mod schema;
pub mod models;

use std::fs;
use std::path::{ Path, PathBuf };

use clap::Parser;

use diesel::prelude::*;
use diesel_migrations::{ embed_migrations, EmbeddedMigrations, MigrationHarness };

use models::wepn_file:: { NewWeaponFileCollection, NewWeaponFile };

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the Homeworld RM 'data/' directory for export. If both this and `--import-dir` are the same, the import will be performed first.
    #[arg(long, short)]
    export_dir: Option<String>,

    /// Path to the Homeworld RM 'data/' directory for import. If both this and `--export-dir` are the same, the import will be performed first.
    #[arg(long, short)]
    import_dir: Option<String>,

    /// Path to Database file; If no directories provided, the database will be migrated only.
    #[arg(long, short)]
    db: String
}

/// Find files with a specific extension.
pub fn find_files_with_extension(root_path: &Path, extension: &str) -> Vec<PathBuf> {
    let mut files = vec![];
    if let Ok(entries) = fs::read_dir(root_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().unwrap_or_default() == extension {
                    files.push(path);
                } else if path.is_dir() {
                    let mut sub_files = find_files_with_extension(&path, extension);
                    files.append(&mut sub_files);
                }
            }
        }
    }
    files
}

/// Establish Connection to Sqlite Database.
pub fn establish_connection(db_path: &Path) -> SqliteConnection {
    let database_url: String = format!("file:{}", db_path.display());

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// Import "data" directory into a Sqlite database.
pub fn import(connection: &mut SqliteConnection, data_dir: &Path) {
    let extension = "wepn";
    let files = find_files_with_extension(data_dir, extension);
    let mut weapon_files: Vec<NewWeaponFile> = Vec::new();

    for file in files {
        weapon_files.push(NewWeaponFile::from_path(file.as_path()));
    }

    let collection = NewWeaponFileCollection::from_vec(weapon_files);

    collection.insert(connection);
}

fn main() {
    let args = Args::parse();

    let connection = &mut establish_connection(Path::new(&args.db));

    connection.run_pending_migrations(MIGRATIONS).expect("Database could not be migrated!");

    if let Some(dir) = args.import_dir {
        println!("Importing {} to {}...", dir, args.db);

        import(connection, Path::new(&dir));
    }

    if let Some(dir) = args.export_dir {
        println!("Exporting {} to {}...", args.db, dir)
    }
}
