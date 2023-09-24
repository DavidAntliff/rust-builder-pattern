mod task;
mod web;

use std::error::Error;
use task::Task;
use web::Request;
use crate::web::RequestBuilder;

fn task_demo() -> Result<(), Box<dyn Error>> {
    //let task = Task::new("Task 01");

    //let task = Task::default();

    let task: Option<Task> = None;
    let task = task.unwrap_or_default();

    println!("{task:#?}");

    Ok(())
}

fn web_demo() -> Result<(), Box<dyn Error>> {
    let req = RequestBuilder::new()
        .url("https://some-url.com/task/123")
        .method("GET")
        .header("token", "user_uuid.exp.sign")
        .build()?;
    println!("{req:#?}");
    Ok(())
}

fn web_demo_non_consuming() -> Result<(), Box<dyn Error>> {
    let mut req_builder = RequestBuilder::new();
    req_builder
        .url("https://some-url.com/task/123")
        .method("GET");
    // do some stuff...
    let req = req_builder
        .header("token", "user_uuid.exp.sign")
        .build()?;
    println!("1: {req:#?}");

    // call the same builder multiple times
    req_builder.header("Client-Version", "1.2");
    let req = req_builder.build()?;
    println!("2: {req:#?}");

    Ok(())
}

fn web_demo_consuming() -> Result<(), Box<dyn Error>> {
    let req_builder = RequestBuilder::new()
        .url_consuming("https://some-url.com/task/123")
        .method_consuming("GET");
    // do some stuff...
    let req_builder = req_builder
        .header_consuming("token", "user_uuid.exp.sign");
    // do more stuff...
    let req = req_builder
        .clone().build_consuming()?;
    println!("Consuming 1: {req:#?}");

    // call the same builder multiple times
    let req = req_builder
        .header_consuming("Client-Version", "1.2")
        .build_consuming()?;
    println!("Consuming 2: {req:#?}");

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {

    //task_demo()
    //web_demo()?;
    web_demo_non_consuming()?;
    web_demo_consuming()?;

    Ok(())
}
