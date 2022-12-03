use std::collections::HashMap;

use dozer_core::{
    dag::{executor_local::DEFAULT_PORT_HANDLE, node::Processor},
    storage::transactions::SharedTransaction,
};
use dozer_types::{
    ordered_float::OrderedFloat,
    types::{Field, FieldDefinition, FieldType, Operation, Record, Schema},
};

use crate::pipeline::aggregation::tests::aggregation_tests_utils::init_processor;

#[test]
fn test_simple_aggregation() {
    let (mut processor, tx) = init_processor(
        "SELECT Country, SUM(Salary) \
        FROM Users \
        WHERE Salary >= 1 GROUP BY Country",
    )
    .unwrap();

    let schema = Schema::empty()
        .field(
            FieldDefinition::new(String::from("ID"), FieldType::Int, false),
            false,
            false,
        )
        .field(
            FieldDefinition::new(String::from("Country"), FieldType::String, false),
            false,
            false,
        )
        .field(
            FieldDefinition::new(String::from("Salary"), FieldType::Float, false),
            false,
            false,
        )
        .field(
            FieldDefinition::new(String::from("SUM(Salary)"), FieldType::Float, false),
            false,
            false,
        )
        .clone();

    let _output_schema = processor
        .update_schema(
            DEFAULT_PORT_HANDLE,
            &HashMap::from([(DEFAULT_PORT_HANDLE, schema)]),
        )
        .unwrap();

    let inp = Operation::Insert {
        new: Record::new(
            None,
            vec![
                Field::Int(0),
                Field::String("Italy".to_string()),
                Field::Float(OrderedFloat(100.0)),
                Field::Float(OrderedFloat(100.0)),
            ],
        ),
    };

    let out = processor
        .aggregate(
            &mut SharedTransaction::new(&tx),
            &processor.db.clone().unwrap(),
            inp,
        )
        .unwrap_or_else(|_e| panic!("Error executing aggregate"));

    let exp = vec![Operation::Insert {
        new: Record::new(
            None,
            vec![
                Field::String("Italy".to_string()),
                Field::Float(OrderedFloat(100.0)),
            ],
        ),
    }];
    assert_eq!(out, exp);
}
