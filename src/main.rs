use std::env;
use std::path::PathBuf;
use std::fs;
use std::io;
use serde::{Serialize, Deserialize};
use std::io::Write;
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="UPPERCASE")]
enum Task_Status {
	NOT_STARTED,
	IN_PROGRESS,
	COMPLETED,
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
	id: u64,
	task_name: String,
	status: Task_Status,
}

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
	tasks: Vec<Task>,
	next_id: u64,
}

fn get_app_directory() -> Result<PathBuf, io::Error> {
    
    let home_dir = env::var("HOME").expect("Failed to find the home directory");
    let mut path = PathBuf::from(home_dir);

    path.push(".local");
    path.push("share");
    path.push("todo_app");
    fs::create_dir_all(&path)?;
    
    Ok(path)
}

fn get_file_path() -> Result<PathBuf, io::Error> {
    let mut path = get_app_directory()?;
    path.push("todo.json");
    Ok(path)
}

fn create_app_file() -> Result<PathBuf, io::Error> {
	let mut app_dir = get_file_path()?;
	if !app_dir.exists() {
		let mut file = fs::File::create(&app_dir)?;
		file.write_all("{\"tasks\": [], \"next_id\": 1}".as_bytes())?;
	}

	Ok(app_dir)
}

fn read_tasks_from_file(path: PathBuf) -> Result<Todo, io::Error> {
	let data = fs::read_to_string(path)?;
	let todo: Todo = serde_json::from_str(&data)?;
	Ok(todo)
}

impl Todo {
	fn new() -> Result<Todo, io::Error> {
		let todo_file_path = create_app_file()?;
		let todo = read_tasks_from_file(todo_file_path)?;

		let mut todo = Todo {
			tasks: todo.tasks,
			next_id: todo.next_id,
		};

		Ok(todo)
	}

	fn list_tasks(&self) {
        println!("\n========== TASKS ==========\n");
		for task in &self.tasks {
			println!("{:#?}. {:#?} -> {:#?}", task.id, task.task_name, task.status);
		}
	}

	fn add_task(&mut self, task_name: String) {
		let task: Task = Task {
			id: self.next_id,
			task_name,
			status: Task_Status::NOT_STARTED,
		};

		self.tasks.push(task);
		self.next_id += 1;
	}

    fn delete_task(&mut self, id: u64) {
        if let Some(idx) = self.tasks.iter().position(|task| task.id == id) {
            self.tasks.swap_remove(idx);
            println!("Task number {id} deleted");
        } else {
            println!("no such tasks");
        }
    }

    fn sync(&self) -> Result<(), io::Error>{
        println!("\nSyncing...\n");
        let path = get_file_path()?;
        let todo_json = serde_json::to_string_pretty(&self)?;
        let mut file = fs::File::create(&path)?;
        file.write_all(todo_json.as_bytes())?;
        Ok(())
    }
}

fn main() {
	let result_todo = Todo::new();
	
	let mut todo = match result_todo {
		Ok(todo) => {
			println!("\nWelcome to TODO Tasks!");
			todo
		},

		Err(e) => {
			panic!("(ERROR):: {:#?}", e)
		}
	};
	
	loop {
		println!("\nwhat would you like to do?");
		println!("1. Add Task");
		println!("2. List Tasks");
		println!("3. Update Task Status");
		println!("4. Delete Tasks");
		println!("5. Quit(or ctrl + c)");

		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();

		match input.trim() {
			"1" => {
				let mut task_name = String::new();
				println!("\nPlease enter the task name");
				io::stdin().read_line(&mut task_name).unwrap();
				let task_name = task_name.trim().to_string();
				if task_name.len() == 0 {
					eprintln!("(ERROR):: Failed to add task, invalid task name");
					continue;
				}
				todo.add_task(task_name);
			},
			"2" => {
				todo.list_tasks();
			},
            "4" => {
                todo.list_tasks();
                println!("\nPlease enter the task id to delete\n");
                let mut task_id = String::from("");
                io::stdin().read_line(&mut task_id).unwrap();
                let task_id = task_id.trim();
                let task_id: u64 = task_id.parse().unwrap();
                todo.delete_task(task_id); 
            },
			"5" => {
                if let Ok(r) = todo.sync() {
                    break;
                } else {
                    eprintln!("(ERROR)::Failed to sync the DB.");
                    println!("\nWould you like to still exit?(y/n)");
                    println!("WARNING: All the new data will lost");
                    
                    let mut input = String::from("");
                    io::stdin().read_line(&mut input).expect("Failed to read the input");
                    if input == "y" {
                        break;
                    }

                    continue;
                }
				break;
			},
			_ => {
				println!("\nBAD CHOICE DUDE PLEASE CHOOSE FROM 1-5!");
			}
		}
	}


}
