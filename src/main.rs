use aws_sdk_costexplorer as costexplorer;
use aws_sdk_costexplorer::model::{DateInterval, Granularity};
use chrono::{NaiveDate, Utc};
use chrono_utilities::naive::DateTransitions;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("execute bootstrap#main");
    let runtime_handler = service_fn(handler);
    lambda_runtime::run(runtime_handler).await?;
    Ok(())
}

async fn handler(_event: LambdaEvent<Value>) -> Result<Value, Error> {
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

    let now = Utc::now().naive_utc();
    let start_date = now.date().start_of_month().unwrap();
    let end_date = now.date();

    let operation = client
        .get_cost_and_usage()
        .granularity(Granularity::Monthly)
        .time_period(
            DateInterval::builder()
                .start(format_date(start_date))
                .end(format_date(end_date))
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

fn format_date(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}
