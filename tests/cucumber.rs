use cucumber::World;

mod steps;
use steps::MothWorld;

#[tokio::main]
async fn main() {
    MothWorld::cucumber()
        .max_concurrent_scenarios(1)
        .run("tests/features")
        .await;
}
