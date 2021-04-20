use clap::{App, AppSettings, Arg};

pub fn subscribe_cmd<'a, 'b>() -> App<'a, 'b> {
    App::new("subscribe")
        .about("subscribe to a gossip-sub topic")
        .usage("p2p subscribe --topic <topic>")
        .settings(&[
            AppSettings::DisableHelpSubcommand,
            AppSettings::DisableHelpFlags,
            AppSettings::DisableVersion,
        ])
        .arg(
            Arg::with_name("topic")
                .help("the topic to subscribe to")
                .short("t")
                .long("topic")
                .value_name("topic")
                .takes_value(true)
                .required(true),
        )
}

pub fn unsubscribe_cmd<'a, 'b>() -> App<'a, 'b> {
    App::new("unsubscribe")
        .about("unsubscribe from a gossip-sub topic")
        .usage("p2p unsubscribe --topic <topic>")
        .settings(&[
            AppSettings::DisableHelpSubcommand,
            AppSettings::DisableHelpFlags,
            AppSettings::DisableVersion,
        ])
        .arg(
            Arg::with_name("topic")
                .help("the topic to unsubscribe from")
                .short("t")
                .long("topic")
                .value_name("topic")
                .takes_value(true)
                .required(true),
        )
}

pub fn publish_cmd<'a, 'b>() -> App<'a, 'b> {
    App::new("publish")
    .about("publish data to certain gossip-sub topic")
    .usage("p2p publish --topic <topic> [SUBCOMMAND]")
    .settings(&[AppSettings::DisableHelpSubcommand, AppSettings::DisableHelpFlags, AppSettings::DisableVersion])
    .subcommand(
        App::new("led")
            .about("send LED configuration;\tUSAGE: p2p publish -t <topic> led (on|off|blink -f <frequency>)")
            .subcommand(App::new("on").about("LED on"))
            .subcommand(App::new("off").about("LED off"))
            .subcommand(
                App::new("blink").about("blink LED").arg(
                    Arg::with_name("frequency")
                        .help("the frequency in seconds in which the led should blink")
                        .short("f")
                        .long("freq")
                        .value_name("frequency")
                        .takes_value(true)
                        .required(true),
                ),
            ),
    )
    .subcommand(
        App::new("message")
        .about("send a string message;\tUSAGE: p2p publish -t <topic> message -v <value>")
        .arg(
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
    )
}

pub fn get_record_cmd<'a, 'b>() -> App<'a, 'b> {
    App::new("get-record")
        .about("query for a kademlia record")
        .usage("p2p get-record --key <key>")
        .settings(&[
            AppSettings::DisableHelpSubcommand,
            AppSettings::DisableHelpFlags,
            AppSettings::DisableVersion,
        ])
        .arg(
            Arg::with_name("key")
                .help("the key of the record")
                .short("k")
                .long("key")
                .value_name("key")
                .takes_value(true)
                .required(true),
        )
}

pub fn put_record_cmd<'a, 'b>() -> App<'a, 'b> {
    App::new("put-record")
        .about("publish a record to the kademlia DHT")
        .usage("p2p put-record --key <key> --value <value>")
        .settings(&[
            AppSettings::DisableHelpSubcommand,
            AppSettings::DisableHelpFlags,
            AppSettings::DisableVersion,
        ])
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
        )
}

pub fn connect_cmd<'a, 'b>() -> App<'a, 'b> {
    App::new("connect")
        .about("explicitly connect a new peer")
        .usage("p2p connect -a <multi-address>")
        .settings(&[
            AppSettings::DisableHelpSubcommand,
            AppSettings::DisableHelpFlags,
            AppSettings::DisableVersion,
        ])
        .arg(
            Arg::with_name("address")
                .help("the mutliaddress of the peer")
                .short("a")
                .long("address")
                .value_name("addr")
                .takes_value(true)
                .required(true),
        )
}

// Build App for Command Line Interface to parse user input.
pub fn build_app<'a, 'b>() -> App<'a, 'b> {
    App::new("p2p")
        .version("0.1.0")
        .author("Elena Frank")
        .about("CLI for the p2p-network interaction")
        .subcommand(subscribe_cmd())
        .subcommand(unsubscribe_cmd())
        .subcommand(publish_cmd())
        .subcommand(get_record_cmd())
        .subcommand(put_record_cmd())
        .subcommand(connect_cmd())
        .subcommand(App::new("shutdown").about("shutdown the app"))
        .settings(&[
            AppSettings::DisableHelpSubcommand,
            AppSettings::DisableHelpFlags,
            AppSettings::DisableVersion,
        ])
}
