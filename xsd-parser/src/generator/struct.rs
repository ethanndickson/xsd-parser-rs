use crate::generator::validator::gen_validate_impl;
use crate::generator::Generator;
use crate::parser::types::Struct;
use std::borrow::Cow;

pub trait StructGenerator {
    fn generate(&self, entity: &Struct, gen: &Generator) -> String {
        format!(
            "{comment}{macros}pub struct {name} {{{fields}}}\n\n{traits}\n{subtypes}\n",
            comment = self.format_comment(entity, gen),
            macros = self.macros(entity, gen),
            name = self.get_type_name(entity, gen),
            fields = self.fields(entity, gen),
            subtypes = self.subtypes(entity, gen),
            traits = self.traits(entity, gen),
        )
    }

    fn fields(&self, entity: &Struct, gen: &Generator) -> String {
        let mod_name = self.mod_name(entity, gen);

        entity.fields.borrow_mut().iter_mut().for_each(|f| {
            if !f.subtypes.is_empty() {
                f.type_name = format!("{}::{}", mod_name, f.type_name)
            }
        });

        let fields = entity
            .fields
            .borrow()
            .iter()
            .map(|f| gen.struct_field_gen().generate(f, gen))
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>()
            .join("\n\n");

        if fields.is_empty() {
            fields
        } else {
            format!("\n{}\n", fields)
        }
    }

    fn subtypes(&self, entity: &Struct, gen: &Generator) -> String {
        let field_subtypes = entity
            .fields
            .borrow()
            .iter()
            .map(|f| gen.base().join_subtypes(f.subtypes.as_ref(), gen))
            .collect::<Vec<String>>()
            .join("");

        let subtypes = gen.base().join_subtypes(entity.subtypes.as_ref(), gen);

        if !field_subtypes.is_empty() || !subtypes.is_empty() {
            format!(
                "\npub mod {name} {{\n{indent}use super::*;{st}\n{fst}\n}}\n",
                name = self.mod_name(entity, gen),
                st = subtypes,
                indent = gen.base().indent(),
                fst = self.shift(&field_subtypes, gen.base().indent().as_str())
            )
        } else {
            format!("{}\n{}", subtypes, field_subtypes)
        }
    }

    fn shift(&self, text: &str, indent: &str) -> String {
        text.replace("\n\n\n", "\n") // TODO: fix this workaround replace
            .split('\n')
            .map(|s| {
                if !s.is_empty() {
                    format!("\n{}{}", indent, s)
                } else {
                    "\n".to_string()
                }
            })
            .fold(indent.to_string(), |acc, x| acc + &x)
    }

    fn get_type_name(&self, entity: &Struct, gen: &Generator) -> String {
        gen.base()
            .format_type_name(entity.name.as_str(), gen)
            .into()
    }

    fn macros(&self, entity: &Struct, gen: &Generator) -> Cow<'static, str> {
        let mut traits = String::new();
        entity.basetypes.borrow().iter().for_each(|t| {
            traits.push_str(&format!("SE{t},"));
        });
        let derives_rename = format!("#[derive(Default, PartialEq, Debug, Clone, YaSerialize, YaDeserialize, {})]\n#[yaserde(rename = \"{}\")]\n", traits, entity.name);
        let tns = gen.target_ns.borrow();
        match tns.as_ref() {
            Some(tn) => match tn.name() {
                Some(name) => format!(
                    "{derives}#[yaserde(prefix = \"{prefix}\", namespace = \"{prefix}: {uri}\")]\n",
                    derives = derives_rename,
                    prefix = name,
                    uri = tn.uri()
                ),
                None => format!(
                    "{derives}#[yaserde(namespace = \"{uri}\")]\n",
                    derives = derives_rename,
                    uri = tn.uri()
                ),
            },
            None => format!("{derives}#[yaserde()]\n", derives = derives_rename),
        }
        .into()
    }

    fn format_comment(&self, entity: &Struct, gen: &Generator) -> String {
        gen.base().format_comment(entity.comment.as_deref(), 0)
    }

    fn mod_name(&self, entity: &Struct, gen: &Generator) -> String {
        gen.base().mod_name(entity.name.as_str())
    }

    fn traits(&self, entity: &Struct, gen: &Generator) -> Cow<'static, str> {
        let mut traits = String::new();

        // Empty validation
        traits.push_str(&gen_validate_impl(
            self.get_type_name(entity, gen).as_str(),
            "",
        ));
        Cow::Owned(traits)
    }
}

pub struct DefaultStructGen;
impl StructGenerator for DefaultStructGen {}
