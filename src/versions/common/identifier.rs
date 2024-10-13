use color_eyre::eyre::Result;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{self, Visitor};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    namespace: String,
    path: String,
}

impl Identifier {
    pub fn new(namespace: String, path: String) -> Self {
        Self { namespace: namespace, path: path }
    }

    pub fn parse(src: String) -> Result<Self> {
        let res: Self = serde_plain::from_str(&src)?;
        Ok(
            res
        )
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Join the namespace and path with a colon (:) separator
        let s = format!("{}:{}", self.namespace, self.path);
        serializer.serialize_str(&s) // Serialize directly as a string
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Identifier, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Expecting a string in the form of "namespace:path"
        struct IdentifierVisitor;

        impl<'de> Visitor<'de> for IdentifierVisitor {
            type Value = Identifier;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string in the format 'namespace:path'")
            }

            fn visit_str<E>(self, value: &str) -> Result<Identifier, E>
            where
                E: de::Error,
            {
                // Split the input string by ':'
                let parts: Vec<&str> = value.split(':').collect();
                if parts.len() == 2 {
                    Ok(Identifier {
                        namespace: parts[0].to_string(),
                        path: parts[1].to_string(),
                    })
                } else {
                    Err(de::Error::custom("invalid format for Identifier"))
                }
            }
        }

        deserializer.deserialize_str(IdentifierVisitor)
    }
}
