use crate::TxtRecord;

#[test]
fn insert_get_success() {
    super::setup();
    let mut record = TxtRecord::new();
    record.insert("foo", "bar").unwrap();
    assert_eq!(&record["foo"], "bar");
    assert_eq!(record.get("baz"), None);
}

#[test]
fn remove_success() {
    super::setup();
    let mut record = TxtRecord::new();
    record.insert("foo", "bar").unwrap();
    record.remove("foo").unwrap();
    assert!(record.get("foo").is_none());
}

#[test]
fn to_string_success() {
    super::setup();
    let mut record = TxtRecord::new();
    record.insert("foo", "bar").unwrap();
    assert_eq!(record.to_string(), "\u{14}foo=bar".to_string());
}

#[test]
fn contains_key_success() {
    super::setup();
    let mut record = TxtRecord::new();
    record.insert("foo", "bar").unwrap();
    assert!(record.contains_key("foo"));
    assert!(!record.contains_key("baz"));
}

#[test]
fn len_success() {
    super::setup();
    let mut record = TxtRecord::new();
    record.insert("foo", "bar").unwrap();
    assert_eq!(record.len(), 1);
}

#[test]
fn iter_success() {
    super::setup();

    debug!("iter_success()");

    let mut record = TxtRecord::new();
    record.insert("foo", "bar").unwrap();
    record.insert("baz", "qux").unwrap();
    record.insert("hello", "world").unwrap();

    for (key, value) in record.iter() {
        debug!("({:?}, {:?})", key, value);
    }
}

#[test]
fn keys_success() {
    super::setup();

    debug!("keys_success()");

    let mut record = TxtRecord::new();
    record.insert("foo", "bar").unwrap();
    record.insert("baz", "qux").unwrap();
    record.insert("hello", "world").unwrap();

    for key in record.keys() {
        debug!("{:?}", key);
    }
}

#[test]
fn values_success() {
    super::setup();

    debug!("values_success()");

    let mut record = TxtRecord::new();
    record.insert("foo", "bar").unwrap();
    record.insert("baz", "qux").unwrap();
    record.insert("hello", "world").unwrap();

    for value in record.values() {
        debug!("{:?}", value);
    }
}
