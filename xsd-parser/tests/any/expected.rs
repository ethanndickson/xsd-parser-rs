#[derive(Default, PartialEq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct FooType {
    #[yaserde(prefix = "tns", rename = "Name")]
    pub name: String,
}
