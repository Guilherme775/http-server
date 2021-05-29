use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self {
            address
        }
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.address);

        match listener {
            Ok(listener) => {
                println!("Listening on {}", &self.address);

                loop {
                    match listener.accept() {
                        Ok((_stream, address)) => {
                            println!("Running on address: {}", address);
                        },
                        Err(..) => println!("Error with connection"),
                    }
                }
            },
            Err(..) => println!("Error running a server"),
        }
    }
}