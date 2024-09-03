mod utils;
use crate::utils::{
    derive_builder_attr, serde_as_attr, serde_attr, sqlx_from_row_attr, sqlx_type_attr,
};
use tonic_build::Builder;

/// provide extra attributes to the generated protobuf code easily
pub trait BuilderAttributes {
    /// add type attributes with `#[derive(serde::Serialize, serde::Deserialize)]`
    fn with_serde(self, paths: &[&str], ser: bool, de: bool, extra_attrs: Option<&[&str]>) -> Self;
    fn with_serde_as(self, path: &str, fields: &[(&[&str], &str)]) -> Self;
    /// add type attributes with `#[derive(sqlx::Type)]`
    fn with_sqlx_type(self, paths: &[&str], extra_attrs: Option<&[&str]>) -> Self;
    /// add type attributes with `#[derive(sqlx::FromRow)]`
    fn with_sqlx_from_row(self, paths: &[&str], extra_attrs: Option<&[&str]>) -> Self;
    /// add type attributes with `#[derive(derive_builder::Builder)]`
    fn with_derive_builder(self, paths: &[&str], extra_attrs: Option<&[&str]>) -> Self;
    /// add type attributes with `#[derive(strum::EnumString)]`
    fn with_strum(self, paths: &[&str], extra_attrs: Option<&[&str]>) -> Self;
    /// add type attributes
    fn with_type_attributes(self, paths: &[&str], attributes: &[&str]) -> Self;
    /// add field attributes
    fn with_field_attributes(self, paths: &[&str], attributes: &[&str]) -> Self;
    /// add optional type attributes
    fn with_optional_type_attributes(self, paths: &[&str], attributes: Option<&[&str]>) -> Self;
    /// add optional field attributes
    fn with_optional_field_attributes(self, paths: &[&str], attributes: Option<&[&str]>) -> Self;
}

/// provide extra attributes to the generated protobuf code easily
impl BuilderAttributes for Builder {
    fn with_serde(self, paths: &[&str], ser: bool, de: bool, extra_attrs: Option<&[&str]>) -> Self {
        let attr = serde_attr(ser, de);

        paths.iter().fold(self, |builder, ty| {
            builder
                .type_attribute(ty, attr)
                .with_optional_type_attributes(&[ty], extra_attrs)
        })
    }

    fn with_serde_as(self, path: &str, fields: &[(&[&str], &str)]) -> Self {
        let serde_attr = serde_as_attr();
        let builder = self.type_attribute(path, serde_attr);
        fields.iter().fold(builder, |builder, (paths, attr)| {
            paths.iter().fold(builder, |builder, p| {
                let p = format!("{}.{}", path, p);
                builder.field_attribute(p, attr)
            })
        })
    }

    fn with_sqlx_type(self, paths: &[&str], extra_attrs: Option<&[&str]>) -> Self {
        paths.iter().fold(self, |builder, ty| {
            builder
                .type_attribute(ty, sqlx_type_attr())
                .with_optional_type_attributes(&[ty], extra_attrs)
        })
    }

    fn with_sqlx_from_row(self, paths: &[&str], extra_attrs: Option<&[&str]>) -> Self {
        paths.iter().fold(self, |builder, ty| {
            builder
                .type_attribute(ty, sqlx_from_row_attr())
                .with_optional_type_attributes(&[ty], extra_attrs)
        })
    }

    fn with_derive_builder(self, paths: &[&str], extra_attrs: Option<&[&str]>) -> Self {
        paths.iter().fold(self, |builder, ty| {
            builder
                .type_attribute(ty, derive_builder_attr())
                .with_optional_type_attributes(&[ty], extra_attrs)
        })
    }

    fn with_strum(self, paths: &[&str], extra_attrs: Option<&[&str]>) -> Self {
        paths.iter().fold(self, |builder, ty| {
            builder
                .type_attribute(
                    ty,
                    "#[derive(strum::EnumString, strum::Display, strum::EnumIter)]",
                )
                .with_optional_type_attributes(&[ty], extra_attrs)
        })
    }

    fn with_type_attributes(self, paths: &[&str], attributes: &[&str]) -> Self {
        let attr = attributes.join("\n");

        paths.iter().fold(self, |builder, ty| {
            builder.type_attribute(ty, attr.as_str())
        })
    }

    fn with_field_attributes(self, paths: &[&str], attributes: &[&str]) -> Self {
        let attr = attributes.join("\n");
        paths.iter().fold(self, |builder, ty| {
            builder.field_attribute(ty, attr.as_str())
        })
    }

    fn with_optional_type_attributes(self, paths: &[&str], attributes: Option<&[&str]>) -> Self {
        if let Some(attributes) = attributes {
            self.with_type_attributes(paths, attributes)
        } else {
            self
        }
    }

    fn with_optional_field_attributes(self, paths: &[&str], attributes: Option<&[&str]>) -> Self {
        if let Some(attributes) = attributes {
            self.with_field_attributes(paths, attributes)
        } else {
            self
        }
    }
}
