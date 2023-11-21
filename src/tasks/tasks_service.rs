use tasks_types::{TaskListWrapper, DataBaseTask, ApiRequestTask};
use std::{fs::{File}, path::Path, io::{BufReader, self, Write}};
use uuid::Uuid;
use chrono::{Utc};
use serde_json::Error as SerdeJsonError;

use super::tasks_types::{self, ApiUpdateTask};

fn read_tasks(file_path: &Path) -> Result<TaskListWrapper, serde_json::Error> {
  let file = File::open(file_path).expect("Unable to open file");
  let reader = BufReader::new(file);
  serde_json::from_reader(reader)
}

fn write_tasks(file_path: &Path, data: &str) -> io::Result<()> {
  let mut file = File::create(file_path)?;
  file.write_all(data.as_bytes())?;
  Ok(())
}

fn serialize_tasks(task_list_wrapper: &TaskListWrapper) -> Result<String, serde_json::Error> {
  serde_json::to_string_pretty(task_list_wrapper)
}

fn get_database() -> Result<TaskListWrapper, SerdeJsonError> {
  let json_file_path = Path::new("./database.json");
  let tasks_wrapper = read_tasks(json_file_path);

  tasks_wrapper
}

fn update_database(tasks_wrapper: TaskListWrapper) {
  match serialize_tasks(&tasks_wrapper) {
    Ok(serialized) => {
        // Write the serialized data to a file
        if let Err(e) = write_tasks(Path::new("./database.json"), &serialized) {
            eprintln!("Error writing file: {}", e);
        }
    },
    Err(e) => eprintln!("Error serializing tasks: {}", e),
  }
}

pub fn get_tasks() -> Result<Vec<DataBaseTask>, SerdeJsonError> {
  let tasks_wrapper = get_database()?;

  Ok(tasks_wrapper.tasks)
}

pub fn add_task(body: ApiRequestTask) -> Result<DataBaseTask, SerdeJsonError> {
  match get_database() {
    Ok(mut tasks_wrapper) => {
      let new_task = DataBaseTask {
          title: body.title, // If it's already Option<String>, use it directly
          description: Some(body.description.unwrap_or_else(|| "".to_string())),
          completed: Some(body.completed.unwrap_or(false)),
          priority: Some(body.priority.unwrap_or_else(|| "".to_string())),
          created_at: Utc::now(),
          updated_at: Utc::now(),
          id: Uuid::new_v4(),
      };

      tasks_wrapper.tasks.push(new_task.clone());
      update_database(tasks_wrapper); // Handle error if update_database returns Result

      Ok(new_task)
  },
  Err(e) => Err(e),
  }
}

pub fn update_task(body: ApiUpdateTask) -> Result<bool, SerdeJsonError> {
  match get_database() {
    Ok(mut tasks_wrapper) => {
      let task_index = tasks_wrapper.tasks
          .iter()
          .position(|task| task.id == body.id);

      if let Some(index) = task_index {
          let item = &tasks_wrapper.tasks[index];

          let updated_task = DataBaseTask {
              title: body.task.title,
              description: Some(body.task.description.unwrap_or_else(|| "".to_string())),
              completed: Some(body.task.completed.unwrap_or(false)),
              priority: Some(body.task.priority.unwrap_or_else(|| "".to_string())),
              created_at: item.created_at,
              updated_at: Utc::now(),
              id: item.id,
          };
          println!("{:?}", updated_task.clone());
          tasks_wrapper.tasks[index] = updated_task; // Direct assignment
          update_database(tasks_wrapper); // Assuming this returns a Result
          return Ok(true)
      } else {
          println!("Element with id: \"{}\" does not exist.", body.id);
          return Ok(false) // Consider using an Err to indicate failure
      }
  },
  Err(e) => Err(e),
  }
}

pub fn remove_task(task_id: Uuid) -> Result<bool, SerdeJsonError> {
  match get_database() {
    Ok(mut tasks_wrapper) => {
      if !tasks_wrapper.tasks.is_empty() {
          tasks_wrapper.tasks.retain(|x| x.id != task_id);
          update_database(tasks_wrapper); // Handle error if update_database returns Result

          return Ok(true)
      }

      println!("Element with id: \"{}\" is not exist.", task_id);
      Ok(false)
  },
  Err(e) => Err(e),
  }
}

// pub fn make_tasks_actions(args: Args) {
//   let json_file_path = Path::new("./database.json");
//   let mut tasks_wrapper = match read_tasks(json_file_path) {
//       Ok(data) => data,
//       Err(e) => {
//           eprintln!("Error reading file: {}", e);
//           return;
//       },
//   };
//   let todo_list = &mut tasks_wrapper.tasks;

//   match TaskCommand::from_str(&args.command) {
//       Some(TaskCommand::Commands) => {
//           println!("\nSupported commands:\n");
//           println!("{:?} - list of commands.\n", TaskCommand::as_str(&TaskCommand::Commands));
//           println!("{:?} - list of existed tasks.\n", TaskCommand::as_str(&TaskCommand::List));
//           println!("{:?} - add a task to the list.\n", TaskCommand::as_str(&TaskCommand::Add));
//           println!("{:?} - edit a task of the list by \"--id\".\n", TaskCommand::as_str(&TaskCommand::Edit));
//           println!("{:?} - remove a task from the list.\n", TaskCommand::as_str(&TaskCommand::Remove));
//           println!("{:?} - filter items by \"--category\" or \"--status\".\n", TaskCommand::as_str(&TaskCommand::Filter));
//       },
//       Some(TaskCommand::List) => {
//           if todo_list.len() > 0 {
//               for element in todo_list {
//                   println!("{:?}", element);
//               }
//           } else {
//               println!("Nothing. Add at least 1 to see it here");
//           }
//       },
//       Some(TaskCommand::Add) => {
//           todo_list.push(Task {
//               title: args.title.clone().unwrap_or_else(|| "".to_string()),
//               description: args.description.unwrap_or_else(|| "".to_string()),
//               priority: args.priority.unwrap_or_else(|| "".to_string()),
//               completed: args.status.unwrap_or_else(|| "no".to_string()) == "yes".to_string(),
//               created_at: Utc::now(),
//               updated_at: Utc::now(),
//               id: Uuid::new_v4()
//           });

//           println!("Element with title: \"{}\" has been added to list.", args.title.as_ref().unwrap());
//       },
//       Some(TaskCommand::Remove) => {
//           if let Some(id) = &args.id {
//               if !todo_list.is_empty() {
//                   todo_list.retain(|x| x.id != *id);

//                   println!("Element with id: \"{}\" is not exist.", id);
//               }
//           }

//           println!("Enter \"--id\" to remove element.");
//       },
//       Some(TaskCommand::Edit) => {
//           if let Some(id) = &args.id {
//               if !todo_list.is_empty() {
//                   let element = todo_list
//                       .iter()
//                       .enumerate()
//                       .find(|&(_idx, val)| val.id == *id);

//                   match element {
//                       Some((index, item)) => {
//                           let status = args.status.unwrap_or_else(|| "no".to_string());
//                           let status_yes: String = "yes".to_string();
//                           let status_no: String = "no".to_string();

//                           let updated_task = Task {
//                               title: args.title.unwrap_or_else(|| item.title.clone()),
//                               description: args.description.unwrap_or_else(|| item.description.clone()),
//                               priority: args.priority.unwrap_or_else(|| item.priority.clone()),
//                               completed: if status == status_yes || status == status_no { status == status_yes } else { item.completed },
//                               created_at: item.created_at,
//                               updated_at: Utc::now(),
//                               id: item.id.clone()
//                           };

//                           todo_list.splice(index..index + 1, std::iter::once(updated_task));

//                           println!("Element with id: \"{}\" has been changed.", id);
//                       },
//                       None => println!("Element with id: \"{}\" is not exist.", id),
//                   };
//               }
//           }
//       },
//       Some(TaskCommand::Filter) => {
//           if let Some(priority) = &args.priority {
//               let filtered_value: Vec<&Task> = todo_list.iter().filter(|&x| x.priority == *priority).collect();

//               if !filtered_value.is_empty() {
//                   println!("\nTasks:\n");

//                   for (index, element) in filtered_value.iter().enumerate() {
//                       println!("{}. {:?}\n", index + 1, element);
//                   }
//               }
//           }

//           if let Some(status) = &args.status {
//               let is_completed: bool = "yes".to_string() == *status;

//               let filtered_value: Vec<&Task> = todo_list.iter().filter(|&x| x.completed == is_completed).collect();

//               if !filtered_value.is_empty() {
//                   println!("\nTasks:\n");

//                   for (index, element) in filtered_value.iter().enumerate() {
//                       println!("{}. {:?}\n", index + 1, element);
//                   }
//               }
//           }
//       }
//       _ => {
//           println!("Try another command");
//       }
//   }

//   match serialize_tasks(&tasks_wrapper) {
//       Ok(serialized) => {
//           // Write the serialized data to a file
//           if let Err(e) = write_tasks(json_file_path, &serialized) {
//               eprintln!("Error writing file: {}", e);
//           }
//       },
//       Err(e) => eprintln!("Error serializing tasks: {}", e),
//   }
// }