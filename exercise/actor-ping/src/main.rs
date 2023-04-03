use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
struct Ping;

struct MyActor;
impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Actor is live");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("Actor is stopped");
    }
}

impl Handler<Ping> for MyActor {
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, msg: Ping, ctx: &mut Self::Context) -> Self::Result {
        println!("Ping received");
        Ok(true)
    }
}

#[actix_rt::main]
async fn main() {
    let addr = MyActor.start();

    let result = addr.send(Ping).await;

    match result {
        Ok(res) => println!("Got result: {}", res.unwrap()),
        Err(err) => println!("Got error: {}", err),
    }
}
