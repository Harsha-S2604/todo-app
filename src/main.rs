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

fn create_app_directory() -> Result<PathBuf, io::Error> {
	let home_dir = env::var("HOME").expect("Failed to find the home directory");
	
	// use pathbuf to construct a path
	let mut path = PathBuf::from(home_dir);
	path.push(".local");
	path.push("share");
	path.push("todo_app");

	fs::create_dir_all(&path)?;

	Ok(path)
}

fn create_app_file() -> Result<PathBuf, io::Error> {
	let mut app_dir = create_app_directory()?;
	app_dir.push("todo.json");
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
				println!("\n========== TASKS ==========\n");
				todo.list_tasks();
			},
			"5" => {
				break;
			},
			_ => {
				println!("\nBAD CHOICE DUDE PLEASE CHOOSE FROM 1-5!");
			}
		}
	}


}
