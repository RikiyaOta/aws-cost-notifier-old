use aws_sdk_costexplorer as costexplorer;
use aws_sdk_costexplorer::model::{DateInterval, Granularity};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("execute bootstrap#main");
    let runtime_handler = service_fn(hello_world);
    lambda_runtime::run(runtime_handler).await?;
    Ok(())
}

async fn hello_world(_event: LambdaEvent<Value>) -> Result<Value, Error> {
    //let first_name = event["firstName"].as_str().unwrap_or("world");
    //Ok(json!({"message": format!("Hello, {}!", first_name)}))

    let _ = get_cost().await;

    Ok(json!({"message": "Hello, World!"}))
}

async fn get_cost() -> Result<(), costexplorer::Error> {
    let config = aws_config::load_from_env().await;
    let client = costexplorer::Client::new(&config);

    // 試しに API 呼び出してみる

    /*
    ↓こんな感じで取れた:
    GetCostAndUsageOutput {
        next_page_token: None,
        group_definitions: None,
        results_by_time: Some([
            ResultByTime {
                time_period: Some(
                    DateInterval {
                        start: Some("2023-01-01"),
                        end: Some("2023-02-01")
                    }
                ),
                total: Some({
                    "UnblendedCost": MetricValue {
                        amount: Some("123456789.123456789"),
                        unit: Some("USD")
                }}),
                groups: Some([]), estimated: true
            }]),
        dimension_value_attributes: Some([])
    }
     */

    let operation = client
        .get_cost_and_usage()
        .granularity(Granularity::Monthly)
        .time_period(
            DateInterval::builder()
                .start("2023-01-01")
                .end("2023-02-01")
                .build(),
        )
        .metrics("UnblendedCost");

    match operation.send().await {
        Ok(result) => {
            println!("{:?}", result);
        }
        Err(error) => {
            println!("{:?}", error);
        }
    }

    Ok(())
}
