#![allow(dead_code)]

mod task;
mod request;

use std::error::Error;
use task::Task;


fn task_demo() -> Result<(), Box<dyn Error>> {
    //let task = Task::new("Task 01");

    //let task = Task::default();

    let task: Option<Task> = None;
    let task = task.unwrap_or_default();

    println!("{task:#?}");

    Ok(())
}

fn demo_non_consuming() -> Result<(), Box<dyn Error>> {
    println!("Non-Consuming Builder:");
    use crate::request::request_builder_non_consuming::RequestBuilder;

    let mut req_builder = RequestBuilder::new();
    req_builder
        .url("https://some-url.com/task/123")
        .method("GET");
    // do some stuff...
    let req = req_builder
        .header("token", "user_uuid.exp.sign")
        .build()?;
    println!("1: {req:#?}");

    // Because the non-consuming builder performed a clone internally,
    // you call the same builder multiple times:
    req_builder.header("Client-Version", "1.2");
    let req = req_builder.build()?;
    println!("2: {req:#?}");

    Ok(())
}

fn demo_consuming() -> Result<(), Box<dyn Error>> {
    println!("Consuming Builder:");
    use crate::request::request_builder_consuming::RequestBuilder;

    let req_builder = RequestBuilder::new()
        .url("https://some-url.com/task/123")
        .method("GET");
    // do some stuff...
    let req_builder = req_builder
        .header("token", "user_uuid.exp.sign");
    // do more stuff...

    // The main point of the consuming builder is that an explicit .clone() is
    // needed if you want to use it multiple times:
    let req = req_builder
        .clone().build()?;
    println!("Consuming 1: {req:#?}");

    // call the same builder multiple times
    let req = req_builder
        .header("Client-Version", "1.2")
        .build()?;
    println!("Consuming 2: {req:#?}");

    Ok(())
}

fn demo_type_state() -> Result<(), Box<dyn Error>> {
    println!("Type State Builder:");
    use crate::request::request_builder_type_state::RequestBuilder;

    let req_builder = RequestBuilder::new()
        //.url("https://some-url.com/task/123")
        .method("GET");

    let req = req_builder
        //.seal()
        .header("Token", "uuid.exp.sign")
        .build()?;

    println!("{req:#?}");

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {

    //task_demo()
    demo_non_consuming()?;
    demo_consuming()?;
    demo_type_state()?;

    Ok(())
}
