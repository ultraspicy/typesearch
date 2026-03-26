//! Index schema types and builders

use crate::field::{FieldMetadata, FieldType};
use serde_json::{json, Value};
use std::collections::HashMap;

/// Represents a complete index schema
#[derive(Debug, Clone)]
pub struct IndexSchema {
    /// Index name
    pub name: String,

    /// Field mappings
    pub fields: Vec<FieldMapping>,

    /// Index settings (shards, replicas, etc.)
    pub settings: IndexSettings,
}

/// Settings for an index
#[derive(Debug, Clone)]
pub struct IndexSettings {
    /// Number of primary shards
    pub number_of_shards: u32,

    /// Number of replica shards
    pub number_of_replicas: u32,

    /// Custom analyzers
    pub analyzers: HashMap<String, AnalyzerDefinition>,

    /// Custom normalizers
    pub normalizers: HashMap<String, NormalizerDefinition>,
}

impl Default for IndexSettings {
    fn default() -> Self {
        Self {
            number_of_shards: 5,
            number_of_replicas: 1,
            analyzers: HashMap::new(),
            normalizers: HashMap::new(),
        }
    }
}

/// Mapping for a single field
#[derive(Debug, Clone)]
pub struct FieldMapping {
    /// Field name
    pub name: String,

    /// Field metadata
    pub metadata: FieldMetadata,

    /// Sub-fields (for multi-field mappings)
    pub fields: Vec<SubField>,
}

/// Sub-field for multi-field mappings
#[derive(Debug, Clone)]
pub struct SubField {
    /// Sub-field name (e.g., "prefix", "exact")
    pub name: String,

    /// Field type
    pub field_type: FieldType,

    /// Analyzer to use
    pub analyzer: Option<String>,
}

/// Analyzer definition
#[derive(Debug, Clone)]
pub struct AnalyzerDefinition {
    /// Tokenizer to use
    pub tokenizer: String,

    /// Token filters
    pub filters: Vec<String>,

    /// Character filters
    pub char_filters: Vec<String>,
}

/// Normalizer definition
#[derive(Debug, Clone)]
pub struct NormalizerDefinition {
    /// Filters to apply
    pub filters: Vec<String>,
}

impl IndexSchema {
    /// Convert schema to OpenSearch/Elasticsearch mapping JSON
    pub fn to_mapping(&self) -> Value {
        let mut properties = json!({});

        for field in &self.fields {
            let mut field_def = json!({
                "type": field.metadata.field_type.to_string(),
            });

            if field.metadata.options.stored {
                field_def["store"] = json!(true);
            }

            if field.metadata.options.sortable {
                field_def["doc_values"] = json!(true);
            }

            if field.metadata.options.case_insensitive {
                field_def["normalizer"] = json!("lowercase_normalizer");
            }

            // Add sub-fields if any
            if !field.fields.is_empty() {
                let mut subfields = json!({});
                for subfield in &field.fields {
                    subfields[&subfield.name] = json!({
                        "type": subfield.field_type.to_string(),
                    });
                    if let Some(analyzer) = &subfield.analyzer {
                        subfields[&subfield.name]["analyzer"] = json!(analyzer);
                    }
                }
                field_def["fields"] = subfields;
            }

            properties[&field.name] = field_def;
        }

        json!({
            "mappings": {
                "properties": properties
            },
            "settings": self.settings_json()
        })
    }

    /// Convert settings to JSON
    fn settings_json(&self) -> Value {
        let mut settings = json!({
            "number_of_shards": self.settings.number_of_shards,
            "number_of_replicas": self.settings.number_of_replicas,
        });

        if !self.settings.analyzers.is_empty() || !self.settings.normalizers.is_empty() {
            let mut analysis = json!({});

            if !self.settings.analyzers.is_empty() {
                let mut analyzers = json!({});
                for (name, def) in &self.settings.analyzers {
                    analyzers[name] = json!({
                        "type": "custom",
                        "tokenizer": def.tokenizer,
                        "filter": def.filters,
                    });
                }
                analysis["analyzer"] = analyzers;
            }

            if !self.settings.normalizers.is_empty() {
                let mut normalizers = json!({});
                for (name, def) in &self.settings.normalizers {
                    normalizers[name] = json!({
                        "type": "custom",
                        "filter": def.filters,
                    });
                }
                analysis["normalizer"] = normalizers;
            }

            settings["analysis"] = analysis;
        }

        settings
    }
}
