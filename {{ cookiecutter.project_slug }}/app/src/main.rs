use lambda_http::{run, http::{StatusCode}, service_fn, Body, Error, Request, RequestExt, Response};
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Answer {
    problem: String,
    answer: AnswerValue,
}

enum AnswerValue {
    Str(String)
    Int(i64)
}

fn solve() -> AnswerValue {
    // do something real here
match 1 {
        AnswerValue::Str(a) => AnswerValue::Str(a)
        AnswerValue::Int(a) => AnswerValue::Int(a)
    }
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let answer = Answer::new("problem${{ cookiecutter.problem_number }}", solve())

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(StatusCode::Ok)
        .header("content-type", "application/json")
        .body(json!(answer))
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
