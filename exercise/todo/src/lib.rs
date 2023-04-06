mod list {

    pub struct Tasks {
        pub items: String,
    }
}
mod things_todo;
use things_todo::add_activity;
use things_todo::items_completed;
fn lets_add_task() {
    let task = list::Tasks {
        items: String::from("Tasks"),
    };
    things_todo::add_activity(); //relative path
    crate::things_todo::add_activity(); // absolute path because we start at the root directory
    add_activity();
    items_completed::remove_task();
}
