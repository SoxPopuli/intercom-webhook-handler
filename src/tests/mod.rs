mod conversation_tests;

use std::str::FromStr as _;

use chrono::TimeZone;

use crate::domain::DateTime;

fn s(s: &str) -> String {
    s.to_string()
}

fn dt(dt: &str) -> DateTime {
    chrono::DateTime::from_str(dt).unwrap()
}

#[test]
fn file_name_format() {
    let now = chrono::Utc.with_ymd_and_hms(2024, 01, 02, 10, 30, 00).unwrap();
    let uuid = uuid::uuid!("00000000-0000-0000-0000-ffff00000000");
    let topic_name = "test.topic";

    let expected = "20240102_test_topic_00000000-0000-0000-0000-ffff00000000.json";
    assert_eq!(expected, crate::get_file_name(&now, topic_name, &uuid))
}
