use mine::result::MineResult;

#[tokio::main]
async fn main() -> MineResult<()> {
    mine::run().await
}
