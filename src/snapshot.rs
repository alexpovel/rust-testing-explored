use openapi_schema::{
    v3::{OpenApi, RefOrObject},
    Doc,
};

const SCHEMA: &str = include_str!("../static/openapi.json");

fn hide_waldo(doc: &mut Doc) -> &mut Doc {
    match doc {
        Doc::V3(OpenApi { paths, .. }) => {
            if let Some(path) = paths.get_mut("/waldo") {
                if let Some(get) = path.get.as_mut() {
                    match get.responses.remove("200") {
                        Some(value @ RefOrObject::Object(_)) => {
                            get.responses.insert("404".to_string(), value.clone());
                        }
                        _ => unreachable!("Hard-coded to be present & not a reference"),
                    }
                };
            };
        }
        _ => unreachable!("Hard-coded v3"),
    }

    doc
}

#[test]
fn test_hiding_waldo() {
    // Arrange
    let mut doc = openapi_schema::from_str(SCHEMA).expect("hard-coded schema is valid");

    // Act
    hide_waldo(&mut doc);

    // Assert
    let let_me_suffer = false;
    if let_me_suffer {
        use openapi_schema::{
            v3::{
                Info, Media, OpenApi, Operation, PathItem, Paths, RefOrObject, Response, Responses,
                Schema,
            },
            Doc,
        };
        use std::collections::BTreeMap;

        // Get ready for insanity. The create doesn't generally `derive(Default)`, thus
        // `..Default::default()` doesn't work currently, as it can't do partial fields.
        // https://internals.rust-lang.org/t/default-for-a-subset-of-fields/20013
        let expected = Doc::V3(OpenApi {
            openapi: "3.0.3".into(),
            info: Info {
                title: "Some API description".into(),
                version: Some("1.33.7".into()),
                ..Default::default()
            },
            servers: None,
            components: None,
            security: None,
            tags: None,
            external_docs: None,
            paths: Paths::from([(
                "/waldo".to_owned(),
                PathItem {
                    get: Some(Operation {
                        summary: Some("Get Waldo".to_owned()),
                        responses: Responses::from([(
                            "404".to_owned(),
                            RefOrObject::Object(Response {
                                description: "Get Waldo".to_owned(),
                                content: Some(BTreeMap::from([(
                                    "application/json".to_owned(),
                                    Media {
                                        schema: Some(RefOrObject::Object(Schema {
                                            r#type: Some("object".to_owned()),
                                            properties: Some(BTreeMap::from([(
                                                "waldo".to_owned(),
                                                RefOrObject::Object(Schema {
                                                    r#type: Some("string".to_owned()),
                                                    title: Default::default(),
                                                    r#enum: Default::default(),
                                                    multiple_of: Default::default(),
                                                    maximum: Default::default(),
                                                    exclusive_maximum: Default::default(),
                                                    minimum: Default::default(),
                                                    exclusive_minimum: Default::default(),
                                                    max_length: Default::default(),
                                                    min_length: Default::default(),
                                                    pattern: Default::default(),
                                                    max_items: Default::default(),
                                                    min_items: Default::default(),
                                                    unique_items: Default::default(),
                                                    items: Default::default(),
                                                    properties: Default::default(),
                                                    max_properties: Default::default(),
                                                    min_properties: Default::default(),
                                                    required: Default::default(),
                                                    all_of: Default::default(),
                                                    one_of: Default::default(),
                                                    any_of: Default::default(),
                                                    not: Default::default(),
                                                    description: Default::default(),
                                                    format: Default::default(),
                                                    default: Default::default(),
                                                    additional_properties: Default::default(),
                                                    nullable: Default::default(),
                                                    discriminator: Default::default(),
                                                    read_only: Default::default(),
                                                    write_only: Default::default(),
                                                    xml: Default::default(),
                                                    external_docs: Default::default(),
                                                    example: Default::default(),
                                                    deprecated: Default::default(),
                                                }),
                                            )])),
                                            title: Default::default(),
                                            r#enum: Default::default(),
                                            multiple_of: Default::default(),
                                            maximum: Default::default(),
                                            exclusive_maximum: Default::default(),
                                            minimum: Default::default(),
                                            exclusive_minimum: Default::default(),
                                            max_length: Default::default(),
                                            min_length: Default::default(),
                                            pattern: Default::default(),
                                            max_items: Default::default(),
                                            min_items: Default::default(),
                                            unique_items: Default::default(),
                                            items: Default::default(),
                                            max_properties: Default::default(),
                                            min_properties: Default::default(),
                                            required: Default::default(),
                                            all_of: Default::default(),
                                            one_of: Default::default(),
                                            any_of: Default::default(),
                                            not: Default::default(),
                                            description: Default::default(),
                                            format: Default::default(),
                                            default: Default::default(),
                                            additional_properties: Default::default(),
                                            nullable: Default::default(),
                                            discriminator: Default::default(),
                                            read_only: Default::default(),
                                            write_only: Default::default(),
                                            xml: Default::default(),
                                            external_docs: Default::default(),
                                            example: Default::default(),
                                            deprecated: Default::default(),
                                        })),
                                        example: Default::default(),
                                        examples: Default::default(),
                                        encoding: Default::default(),
                                    },
                                )])),
                                headers: Default::default(),
                                links: Default::default(),
                                extensions: Default::default(),
                            }),
                        )]),
                        tags: Default::default(),
                        description: Default::default(),
                        external_docs: Default::default(),
                        operation_id: Default::default(),
                        parameters: Default::default(),
                        request_body: Default::default(),
                        callbacks: Default::default(),
                        deprecated: Default::default(),
                        security: Default::default(),
                        servers: Default::default(),
                        extensions: Default::default(),
                    }),
                    reference: Default::default(),
                    summary: Default::default(),
                    description: Default::default(),
                    put: Default::default(),
                    post: Default::default(),
                    delete: Default::default(),
                    options: Default::default(),
                    head: Default::default(),
                    patch: Default::default(),
                    trace: Default::default(),
                    servers: Default::default(),
                    parameters: Default::default(),
                    extensions: Default::default(),
                },
            )]),
        });

        assert_eq!(expected, doc);
    } else {
        insta::assert_json_snapshot!(doc);
    }
}
