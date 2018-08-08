extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_ec2;
extern crate rusoto_sts;

use std::default::Default;

use rusoto_core::request::HttpClient;
use rusoto_core::{DefaultCredentialsProvider, Region};

use rusoto_ec2::{Ec2, Ec2Client};
use rusoto_sts::{StsAssumeRoleSessionCredentialsProvider, StsClient};

fn main() {
    let _ = env_logger::try_init();

    let credentials = DefaultCredentialsProvider::new().unwrap();
    let sts = StsClient::new(Region::EuWest1);
    // let sts = StsClient::new_with(HttpClient::new().unwrap(), credentials, Region::EuWest1);

    let provider = StsAssumeRoleSessionCredentialsProvider::new(
        sts,
        "arn:aws:iam::something:role/something".to_owned(),
        "default".to_owned(),
        None,
        None,
        None,
        None,
    );

    let client = Ec2Client::new_with(HttpClient::new().unwrap(), provider, Region::UsEast1);

    let sir_input = Default::default();
    println!("[*] requesting...");
    let x = client.describe_spot_instance_requests(sir_input);

    // println!("{:?}", x);
}
