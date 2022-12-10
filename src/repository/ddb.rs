use mongodb::{
    error::Error,
    results::InsertOneResult,
    sync::{Collection, Cursor},
};

use log::error;

use crate::model::task::Task;

#[derive(Clone)]
pub struct TaskService {
    collection: Collection<Task>,
}

impl TaskService {
    pub fn new(collection: Collection<Task>) -> TaskService {
        TaskService { collection }
    }

    pub fn create(&self, task: Task) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(task, None)
    }

    pub fn get(&self, task_id: String) -> Option<Task> {
        let tokens: Vec<String> = task_id.split("_").map(|x| String::from(x)).collect();
        let user_uuid: &str = &tokens[0];
        let task_uuid: &str = &tokens[1];

        let mut res: Cursor<Task> = self
            .collection
            .find(
                bson::doc! {"user_uuid": user_uuid, "task_uuid": task_uuid},
                None,
            )
            .unwrap();

        let mut task: Option<Task> = None;
        while res.advance().unwrap() {
            let task_option = res.deserialize_current();
            task = match task_option {
                Ok(item) => Some(item),
                Err(error) => {
                    error!("{:?}", error);
                    None
                }
            }
        }

        task
        // self.collection.find_one(bson::doc! { self.task_global_id}, None)
    }
}
