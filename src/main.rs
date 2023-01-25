use serde::{Serialize, Deserialize};
use aws_sdk_costexplorer as costexplorer;
use aws_sdk_costexplorer::model::{
    DateInterval, Granularity, GroupDefinition, GroupDefinitionType,
};
use aws_sdk_costexplorer::types::SdkError;
use chrono::{NaiveDate, Utc};
use chrono_utilities::naive::DateTransitions;
use costexplorer::error::GetCostAndUsageError;
use env_logger as logger;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    logger::init();

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
    let now = Utc::now().naive_utc();
    let start_date = now.date().start_of_month().unwrap();
    let end_date = now.date();

    let _ = get_costs_by_service(start_date, end_date).await;

    Ok(())
}

fn format_date(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

async fn get_costs_by_service(
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<(), SdkError<GetCostAndUsageError>> {
    let config = aws_config::load_from_env().await;
    let client = costexplorer::Client::new(&config);

    let operation = client
        .get_cost_and_usage()
        .granularity(Granularity::Monthly)
        .time_period(
            DateInterval::builder()
                .start(format_date(start_date))
                .end(format_date(end_date))
                .build(),
        )
        .metrics("NetUnblendedCost")
        .group_by(
            GroupDefinition::builder()
                .r#type(GroupDefinitionType::Dimension)
                .key("SERVICE".to_string())
                .build(),
        );

    match operation.send().await {
        Ok(output) => {
            let results_by_time = output.results_by_time().unwrap();
            //let result = results_by_time[0]
            //    .groups()
            //    .unwrap()
            //    .iter()
            //    .map(|group| (group.keys().unwrap(), group.metrics().unwrap()));
//
            //println!("{:?}", result);

            for group in results_by_time[0].groups().unwrap().iter() {
                println!("{:?}", group.keys().unwrap());
                println!("{:?}", group.metrics().unwrap());
            }

            Ok(())
        }
        Err(error) => {
            log::error!("Failed getting cost data. {:?}", error);
            Err(error)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Cost {
    service: String,
    amount: f64,
}