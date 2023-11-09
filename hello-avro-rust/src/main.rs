use apache_avro::{Codec, Reader, Schema, Writer, from_value, types::Record, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
    favorite_number: i64,
    favorite_color: String,
}

fn main() -> Result<(), Error> {
    let user_schema = r#"{
        "type": "record",
        "name": "User",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "favorite_number",
            "type":   "long"
          },
          {
            "name": "favorite_color",
            "type":  "string"
          }
        ]
    }"#;

    let schema = Schema::parse_str(user_schema).unwrap();
    // println!("{:?}", schema);

    let mut writer = Writer::with_codec(&schema, Vec::new(), Codec::Deflate);

    let user = User {
        name: "John".to_owned(),
        favorite_number: 42,
        favorite_color: "blue".to_owned(),
    };
    writer.append_ser(user)?;

    let mut user2 = Record::new(writer.schema()).unwrap();
    user2.put("name", "Ben");
    user2.put("favorite_number", 7);
    user2.put("favorite_color", "red");
    writer.append(user2)?;

    let input = writer.into_inner()?;

    let reader = Reader::with_schema(&schema, &input[..])?;
    // value is a Result in case the read operation fails
    for value in reader {
        println!("{:?}", from_value::<User>(&value.unwrap()));
    }
    Ok(())
}

