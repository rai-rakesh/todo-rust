use std::env;
use std::fs::File;
use std::io::Write;
use std::io::prelude::*;

struct TodoItem {
    name:String,
    completed:String
}


impl TodoItem {

    fn add_task(name:String,status:String) -> TodoItem {

        return TodoItem{
            name: name,
            completed: status
        };

    }

}

struct TodoList {
    list: Vec<TodoItem>

}

impl TodoList {
    fn new_list() -> TodoList {
        return TodoList{list: Vec::new()};
    }

    fn add_task(&mut self, name:String, status:String) {

        self.list.push(TodoItem::add_task(name,status));
    }

    fn mark_done(&mut self,index: usize)
    {
         self.list[index].completed = "done".to_string();
    }

    fn read_file(&mut self)
    {
        let mut file = File::open("TaskList.txt").expect("Cannot open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Error while reading file");

        if contents != ""
           { 

         contents.lines().for_each(|_line| { let _v: Vec<&str> = _line.split_whitespace().collect();
            self.add_task(_v[3].to_string(),_v[1].to_string())});
        
        }
        else
           { println!("No Tasks!!"); }

        
    }

    #[allow(unused_must_use)]
    fn write_file(&mut self)
    {
        let mut file = File::create("TaskList.txt").expect("Could not create file!");
        let mut i=0;
        for item in &self.list{

            write!(file,"{} {} - {} \n", i,item.completed, item.name);
            i=i+1;
        }
    }

    fn output_list(&mut self)
    {
        
        for item in &self.list{

            println!("[{}] - {} \n", item.completed, item.name);
        }
    }
}


fn main() {
  //  file.write_all(b"nnn").expect("Cannot write file!");
    let args: Vec<String> = env::args().collect();
    //handle for null args[]
    let command: String = args[1].clone();
    let mut todo_list = TodoList::new_list();
 
    if command == "get"
    {
        todo_list.read_file();

        todo_list.output_list();
    }
    else if command == "add"
    {
        let task = args[2].clone();
        todo_list.read_file();
        todo_list.add_task(task.to_string(),"pending".to_string());
        todo_list.write_file();
        todo_list.output_list();

    }
    else if command == "done"
    {
        todo_list.read_file();
        let index:usize = args[2].parse().expect("Unknown Input");
        todo_list.mark_done(index);
        todo_list.write_file();
        todo_list.output_list();
        

    }
}
 