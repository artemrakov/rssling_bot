use aws_sdk_dynamodb::{
    model::{AttributeDefinition, KeySchemaElement, KeyType, ScalarAttributeType, ProvisionedThroughput},
    Client,
};

use crate::db::error::Error;

pub async fn create_tables() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    create_users_table(&client).await?;

    Ok(())
}

async fn create_users_table(client: &Client) -> Result<(), Error> {
    let key = String::from("name");
    let table_name = String::from("users");

    let ad = AttributeDefinition::builder()
        .attribute_name(&key)
        .attribute_type(ScalarAttributeType::S)
        .build();

    let ks = KeySchemaElement::builder()
        .attribute_name(&key)
        .key_type(KeyType::Hash)
        .build();

    let pt = ProvisionedThroughput::builder()
        .read_capacity_units(10)
        .write_capacity_units(5)
        .build();

    let create_table_response = client
        .create_table()
        .table_name(&table_name)
        .key_schema(ks)
        .attribute_definitions(ad)
        .provisioned_throughput(pt)
        .send()
        .await;

    match create_table_response {
        Ok(_) => {
            println!("Added table {} with key {}", table_name, key);
            Ok(())
        }
        Err(e) => {
            eprintln!("Got an error creating table:");
            eprintln!("{}", e);
            Err(Error::unhandled(e))
        }
    }
}
