use crate::oop::{InstOopDesc, Method, MethodId, Oop};
use crate::runtime::{self, Frame};
use std::sync::Arc;

pub struct JavaThread {
    frames: Vec<Frame>,
    args: Option<Vec<Oop>>,
    pc: u32,
    in_safe_point: bool,

    java_thread_obj: Option<InstOopDesc>,
    exception: Option<InstOopDesc>,

    method: Method,
}

pub struct JavaMainThread {
    pub main_class: String,
    pub args: Option<Vec<String>>,
}

impl JavaThread {
    pub fn new(method: &MethodId, args: Option<Vec<Oop>>) -> Self {
        Self {
            frames: Vec::new(),
            args,
            pc: 0,
            in_safe_point: false,

            java_thread_obj: None,
            exception: None,

            method: method.method.clone(),
        }
    }

    pub fn run(&mut self) {
        //todo: impl
    }
}

impl JavaMainThread {
    pub fn run(&self) {
        let main = runtime::require_class3(None, self.main_class.as_bytes()).unwrap();
        let main = main.lock().unwrap();
        let main_method = main
            .get_static_method("([Ljava/lang/String;)V", "main")
            .unwrap();

        let mut args = self.args.as_ref().and_then(|args| {
            Some(
                args.iter()
                    .map(|it| {
                        let r = Arc::new(Vec::from(it.as_bytes()));
                        Oop::Str(r)
                    })
                    .collect(),
            )
        });

        let mut jt = JavaThread::new(main_method, args);
        jt.run();
    }
}
