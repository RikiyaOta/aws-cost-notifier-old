use lambda_runtime::{LambdaEvent, Error, service_fn};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error>{
    println!("execute bootstrap#main");
    let runtime_handler = service_fn(hello_world);
    lambda_runtime::run(runtime_handler).await?;
    Ok(())
}

async fn hello_world(_event: LambdaEvent<Value>) -> Result<Value, Error> {
    //let first_name = event["firstName"].as_str().unwrap_or("world");
    //Ok(json!({"message": format!("Hello, {}!", first_name)}))
    Ok(json!({"message": "Hello, World!"}))
}