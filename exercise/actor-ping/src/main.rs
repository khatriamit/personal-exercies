use actix::prelude::*;
struct MyActor {
    count: usize,
}

impl Actor for MyActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "usize")]
struct Ping(usize);

impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _ctx: &mut Self::Context) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}

#[actix_rt::main]
async fn main() {
    let addr = MyActor { count: 10 }.start();

    let res = addr.send(Ping(10)).await;

    println!("RESULT: {}", res.unwrap() == 20);
    System::current().stop();
}
