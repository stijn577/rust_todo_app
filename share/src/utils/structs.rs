use js_sys::wasm_bindgen::JsValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Date {}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TaskTypes {
    Test,
    Undefined,
    None,
}
pub trait TaskTrait {}

// pub trait TaskTrait<T>: Serialize + Default + Clone {}

// impl<T> TaskTrait<T> for Undefined {
//     // Implement the methods
// }

// impl<T> TaskTrait<T> for Undefined {
//     // Implement the methods
// }

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Undefined;

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Test {
    subject: String,
    chapters: Vec<String>,
}

// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub enum TaskKind {
//     None,
//     Test(Test),
// }

// impl Default for TaskKind {
//     fn default() -> Self {
//         Self::None
//     }
// }

// impl std::ops::Deref for TaskKind {
//     type Target = Test;

//     fn deref(&self) -> &Self::Target {
//         todo!()
//     }
// }

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct Task<T> {
    title: String,
    description: String,
    image: String,
    created: Date,
    due: Option<Date>,
    kind: Box<T>,
}

impl<T> Task<T> {
    pub fn new(
        title: String,
        description: String,
        image: String,
        created: Date,
        due: Option<Date>,
        kind: Box<T>,
    ) -> Self {
        Self {
            title,
            description,
            image,
            created,
            due,
            kind,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TaskBuilder<T = Undefined>
where
    T: Default,
{
    title: String,
    description: String,
    image_path: String,
    created: Date,
    due: Option<Date>,
    kind: Box<T>,
}

impl<T> TaskBuilder<T>
where
    T: Default,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_title(&mut self, title: impl Into<String>) -> &mut Self {
        self.title = title.into();
        self
    }
    pub fn add_description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description = description.into();
        self
    }
    pub fn add_image(&mut self, path: impl Into<String>) -> &mut Self {
        self.image_path = path.into();
        self
    }
    pub fn add_created(&mut self, date: Date) -> &mut Self {
        self.created = date;
        self
    }
    pub fn add_due(&mut self, date: Date) -> &mut Self {
        self.due = Some(date);
        self
    }
    pub fn add_type(&mut self, kind: T) -> &mut Self {
        self.kind = Box::new(kind);
        self
    }

    pub fn build(self) -> Task<T> {
        Task::new(
            self.title,
            self.description,
            self.image_path,
            self.created,
            self.due,
            self.kind,
        )
    }
}

impl TaskBuilder<Test> {
    pub fn add_subject(&mut self, subject: impl Into<String>) -> &mut Self {
        self.kind.subject = subject.into();
        self
    }

    pub fn add_chapter(&mut self, chapter: impl Into<String>) -> &mut Self {
        self.kind.chapters.push(chapter.into());
        self
    }
}

// impl From<Task> for js_sys::wasm_bindgen::JsValue {
//     fn from(value: Task) -> Self {
//         JsValue::from_str(format!("{:?}", value).as_str())
//     }
// }

// impl From<Vec<Task>> for js_sys::wasm_bindgen::JsValue {
// fn from(value: Task) -> Self {
// JsValue::from_str(format!("{:?}", value).as_str())
// }
// }
