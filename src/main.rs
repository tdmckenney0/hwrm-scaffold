pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::insert_into;
use std::fs;
use std::path::{ Path, PathBuf };

use clap::Parser;

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
    let final_path = db_path.canonicalize().expect("Valid Database URL required!");

    let database_url: String = format!("file:{}", final_path.display());

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// Import "data" directory into a Sqlite database.
pub fn import(connection: &mut SqliteConnection, data_dir: &Path) {
    use crate::schema::weapons::dsl::*;

    let extension = "wepn";
    let files = find_files_with_extension(data_dir, extension);
    let mut weapons_to_insert: Vec<models::weapon::StartWeaponConfig> = Vec::new();

    // Delete all weapons first.
    diesel::delete(weapons)
        .execute(connection)
        .expect("Error clearing weapons!");

    for file in files {
        let weapon_name = file.file_stem().unwrap().to_str().unwrap();

        let contents = fs::read_to_string(&file)
            .expect("Should have been able to read the file");

        let lines = contents.lines();

        for line in lines {
            if line.starts_with("StartWeaponConfig") {
                println!("parsing {}...", weapon_name);

                weapons_to_insert.push(models::weapon::StartWeaponConfig::new(weapon_name.to_string(), line.to_string()));
            }
        }
    }

    insert_into(weapons)
        .values(weapons_to_insert)
        .execute(connection)
        .expect("Could not insert weapons!");
}

fn main() {
    let args = Args::parse();

    let connection = &mut establish_connection(Path::new(&args.db));

    if let Some(dir) = args.import_dir {
        println!("Importing {} to {}...", dir, args.db);

        import(connection, Path::new(&dir));
    }

    if let Some(dir) = args.export_dir {
        println!("Exporting {} to {}...", args.db, dir)
    }
}
