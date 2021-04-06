use clap::{App, Arg};

// Build App for Command Line Interface to parse user input.
pub fn build_app<'a, 'b>() -> App<'a, 'b> {
    App::new("p2p")
        .version("0.1.0")
        .author("Elena Frank")
        .about("CLI for the p2p-network interaction")
        .subcommand(
            App::new("subscribe")
                .about("subscribe to a gossip-sub topic")
                .arg(
                    Arg::with_name("topic")
                        .help("the topic to subscribe to")
                        .short("t")
                        .long("topic")
                        .value_name("topic")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            App::new("unsubscribe")
                .about("unsubscribe from a gossip-sub topic")
                .arg(
                    Arg::with_name("topic")
                        .help("the topic to unsubscribe from")
                        .short("t")
                        .long("topic")
                        .value_name("topic")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            App::new("publish")
                .about("publish data to certain gossip-sub topic")
                .subcommand(
                    App::new("led")
                        .about("send LED configuration")
                        .subcommand(App::new("on").about("LED on"))
                        .subcommand(App::new("off").about("LED off"))
                        .subcommand(
                            App::new("blink").about("blink LED").arg(
                                Arg::with_name("frequency")
                                    .help("the frequency in seconds in which the led should blink")
                                    .short("f")
                                    .long("frequency")
                                    .value_name("frequency")
                                    .takes_value(true)
                                    .required(true),
                            ),
                        ),
                )
                .subcommand(
                    App::new("message").about("send a string message").arg(
                        Arg::with_name("value")
                            .help("the string value of the message that should be send")
                            .short("v")
                            .long("value")
                            .value_name("value")
                            .takes_value(true)
                            .required(true),
                    ),
                )
                .arg(
                    Arg::with_name("topic")
                        .help("the topic to which the data should be published")
                        .short("t")
                        .long("topic")
                        .value_name("topic")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            App::new("get-record")
                .about("query for a kademlia record")
                .arg(
                    Arg::with_name("key")
                        .help("the key of the record")
                        .short("k")
                        .long("key")
                        .value_name("key")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            App::new("put-record")
                .about("publish a record to the kademlia DHT")
                .arg(
                    Arg::with_name("key")
                        .help("the key of the record")
                        .short("k")
                        .long("key")
                        .value_name("key")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("value")
                        .help("the value of the record")
                        .short("v")
                        .long("value")
                        .value_name("value")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(App::new("shutdown").about("shutdown the app"))
}
