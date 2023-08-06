#[derive(Default, PartialEq, Debug, Clone, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Id(pub String);

impl Validate for Id {}
#[derive(Default, PartialEq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct FooType {
    #[yaserde(attribute, prefix = "tns", rename = "id")]
    pub id: Option<Id>,
}

impl Validate for FooType {}
