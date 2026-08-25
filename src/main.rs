use sky_cli_tasks::TaskStore;
use std::env;
use std::path::PathBuf;
use std::process;

fn usage() -> &'static str {
    "Usage:\n  sky-tasks add <title>\n  sky-tasks list\n  sky-tasks complete <id>\n  sky-tasks remove <id>\n\nStorage path: SKY_TASKS_FILE (default: ./tasks.json)"
}

fn parse_id(raw: Option<String>) -> Result<u64, String> {
    raw.ok_or_else(|| "missing task id".to_string())?
        .parse::<u64>()
        .map_err(|_| "task id must be a positive integer".to_string())
        .and_then(|id| {
            if id == 0 {
                Err("task id must be greater than zero".to_string())
            } else {
                Ok(id)
            }
        })
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "help".to_string());
    let path = env::var("SKY_TASKS_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("tasks.json"));
    let mut store = TaskStore::load(&path)?;

    match command.as_str() {
        "add" => {
            let title = args.collect::<Vec<_>>().join(" ");
            let task = store.add(&title)?;
            store.save(&path)?;
            println!("added #{}: {}", task.id, task.title);
        }
        "list" => {
            for task in store.list() {
                let marker = if task.completed { "x" } else { " " };
                println!("[{}] #{} {}", marker, task.id, task.title);
            }
        }
        "complete" => {
            let id = parse_id(args.next())?;
            if args.next().is_some() {
                return Err("complete accepts exactly one task id".to_string());
            }
            let task = store.complete(id)?;
            store.save(&path)?;
            println!("completed #{}: {}", task.id, task.title);
        }
        "remove" => {
            let id = parse_id(args.next())?;
            if args.next().is_some() {
                return Err("remove accepts exactly one task id".to_string());
            }
            let task = store.remove(id)?;
            store.save(&path)?;
            println!("removed #{}: {}", task.id, task.title);
        }
        "help" | "--help" | "-h" => println!("{}", usage()),
        other => return Err(format!("unknown command: {other}\n\n{}", usage())),
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        process::exit(2);
    }
}
