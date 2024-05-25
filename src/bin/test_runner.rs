use clap::{Arg, Command, ArgAction};
use std::collections::HashSet;
use mini_robot::version;
use mini_robot::tests::test_suite::TestSuite;
use mini_robot::tests::test_order::TestOrder;

fn main() {
    let matches = Command::new(version::NAME)
        .version(version::VERSION)
        .author(version::AUTHORS)
        .about(version::ABOUT)
        .arg(Arg::new("suite")
            .short('s')
            .long("suite")
            .value_name("SUITE")
            .help("Specify the test suite(s) to run, separated by commas")
            .action(ArgAction::Set))
        .arg(Arg::new("module")
            .short('m')
            .long("module")
            .value_name("MODULE")
            .help("Specify the test module(s) to run, separated by commas")
            .action(ArgAction::Set))
        .arg(Arg::new("tc")
            .short('t')
            .long("tc")
            .value_name("TEST_CASE")
            .help("Specify the test case(s) to run, separated by commas")
            .action(ArgAction::Set))
        .arg(Arg::new("parallel")
            .short('p')
            .long("parallel")
            .value_name("PARALLEL")
            .help("Number of tests to run in parallel")
            .action(ArgAction::Set))
        .arg(Arg::new("order")
            .short('o')
            .long("order")
            .value_name("ORDER")
            .help("Order of test execution: sequential or parallel")
            .action(ArgAction::Set))
        .get_matches();

    let suites: HashSet<String> = matches.get_one::<String>("suite")
        .map(|s| s.split(',').map(|s| s.to_string()).collect())
        .unwrap_or_default();

    let modules: HashSet<String> = matches.get_one::<String>("module")
        .map(|m| m.split(',').map(|m| m.to_string()).collect())
        .unwrap_or_default();

    let test_cases: HashSet<String> = matches.get_one::<String>("tc")
        .map(|t| t.split(',').map(|t| t.to_string()).collect())
        .unwrap_or_default();

    let parallel = matches.get_one::<String>("parallel")
        .and_then(|p| p.parse::<usize>().ok());

    let order = matches.get_one::<String>("order")
        .and_then(|o| match o.to_lowercase().as_str() {
            "parallel" => Some(TestOrder::Parallel),
            "sequential" => Some(TestOrder::Sequential),
            _ => None,
        });

    if suites.is_empty() {
        println!("No test suite specified. Use --suite to specify one or more test suites.");
        return;
    }

    for suite in &suites {
        if let Some(test_suite) = TestSuite::new(suite, parallel, order) {
            test_suite.run_filtered(&modules, &test_cases);
        } else {
            println!("Test suite '{}' not found or no test cases discovered.", suite);
        }
    }
}
