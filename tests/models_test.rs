#[cfg(test)]
mod model_tests {
    use serde_json::{json, Value};
    use xbrl_api_client::api::models::{PaginationParams, EntityFilterParams, SearchParams};
    use xbrl_api_client::data::taxonomy::{Concept, Dimension, Network, NetworkNode, DimensionMember, Reference, ConceptDimension};
    
    #[test]
    fn test_pagination_params_serialization() {
        // Test with both fields
        let params = PaginationParams {
            page: Some(2),
            page_size: Some(50),
        };
        
        let serialized = serde_json::to_string(&params).unwrap();
        let json: Value = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(json["page"], 2);
        assert_eq!(json["page_size"], 50);
        
        // Test with optional fields omitted
        let params = PaginationParams {
            page: None,
            page_size: Some(25),
        };
        
        let serialized = serde_json::to_string(&params).unwrap();
        let json: Value = serde_json::from_str(&serialized).unwrap();
        
        assert!(json.get("page").is_none() || json["page"].is_null());
        assert_eq!(json["page_size"], 25);
    }
    
    #[test]
    fn test_entity_filter_params_serialization() {
        let params = EntityFilterParams {
            entity_id: Some("ent-123".to_string()),
            entity_name: Some("Example Corp".to_string()),
            cik: Some("0001234567".to_string()),
        };
        
        let serialized = serde_json::to_string(&params).unwrap();
        let json: Value = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(json["entity_id"], "ent-123");
        assert_eq!(json["entity_name"], "Example Corp");
        assert_eq!(json["cik"], "0001234567");
        
        // Test with optional fields omitted
        let params = EntityFilterParams {
            entity_id: None,
            entity_name: Some("Example Corp".to_string()),
            cik: None,
        };
        
        let serialized = serde_json::to_string(&params).unwrap();
        let json: Value = serde_json::from_str(&serialized).unwrap();
        
        assert!(json.get("entity_id").is_none() || json["entity_id"].is_null());
        assert_eq!(json["entity_name"], "Example Corp");
        assert!(json.get("cik").is_none() || json["cik"].is_null());
    }
    
    #[test]
    fn test_search_params_full_serialization() {
        let params = SearchParams {
            taxonomy: "us-gaap".to_string(),
            concept_name: Some("Assets".to_string()),
            entity_id: Some("ent-123".to_string()),
            fiscal_year: Some(2022),
            fiscal_period: Some("Q1".to_string()),
            dimension_name: Some("LegalEntityAxis".to_string()),
            member_name: Some("SubsidiaryMember".to_string()),
            text_search: Some("current assets".to_string()),
            value_greater_than: Some(1000000.0),
            value_less_than: Some(2000000.0),
        };
        
        let serialized = serde_json::to_string(&params).unwrap();
        let json: Value = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(json["taxonomy"], "us-gaap");
        assert_eq!(json["concept_name"], "Assets");
        assert_eq!(json["entity_id"], "ent-123");
        assert_eq!(json["fiscal_year"], 2022);
        assert_eq!(json["fiscal_period"], "Q1");
        assert_eq!(json["dimension_name"], "LegalEntityAxis");
        assert_eq!(json["member_name"], "SubsidiaryMember");
        assert_eq!(json["text_search"], "current assets");
        assert_eq!(json["value_greater_than"], 1000000.0);
        assert_eq!(json["value_less_than"], 2000000.0);
    }
    
    #[test]
    fn test_dimension_serialization() {
        let dimension = Dimension {
            name: "LegalEntityAxis".to_string(),
            label: "Legal Entity [Axis]".to_string(),
            description: Some("Axis for legal entities".to_string()),
            taxonomy: "us-gaap".to_string(),
            members: Some(vec![
                DimensionMember {
                    name: "ParentCompanyMember".to_string(),
                    label: "Parent Company [Member]".to_string(),
                    description: Some("Parent company".to_string()),
                },
                DimensionMember {
                    name: "SubsidiaryMember".to_string(),
                    label: "Subsidiary [Member]".to_string(),
                    description: None,
                },
            ]),
        };
        
        let serialized = serde_json::to_string(&dimension).unwrap();
        let json: Value = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(json["name"], "LegalEntityAxis");
        assert_eq!(json["label"], "Legal Entity [Axis]");
        assert_eq!(json["description"], "Axis for legal entities");
        assert_eq!(json["taxonomy"], "us-gaap");
        
        let members = json["members"].as_array().unwrap();
        assert_eq!(members.len(), 2);
        assert_eq!(members[0]["name"], "ParentCompanyMember");
        assert_eq!(members[1]["name"], "SubsidiaryMember");
        assert_eq!(members[1]["label"], "Subsidiary [Member]");
        assert!(members[1].get("description").is_none() || members[1]["description"].is_null());
    }
    
    #[test]
    fn test_network_serialization() {
        let network = Network {
            id: "net-123".to_string(),
            name: "Statement of Financial Position".to_string(),
            short_name: Some("Balance Sheet".to_string()),
            description: Some("Statement of Financial Position".to_string()),
            taxonomy: "us-gaap".to_string(),
            role: "http://www.example.com/role/StatementOfFinancialPosition".to_string(),
            nodes: Some(vec![
                NetworkNode {
                    concept_name: "Assets".to_string(),
                    concept_label: "Assets".to_string(),
                    parent: None,
                    order: Some(1.0),
                    level: Some(1),
                    preferred_label: None,
                    children: Some(vec![
                        NetworkNode {
                            concept_name: "CurrentAssets".to_string(),
                            concept_label: "Current Assets".to_string(),
                            parent: Some("Assets".to_string()),
                            order: Some(1.1),
                            level: Some(2),
                            preferred_label: None,
                            children: None,
                        },
                        NetworkNode {
                            concept_name: "NoncurrentAssets".to_string(),
                            concept_label: "Noncurrent Assets".to_string(),
                            parent: Some("Assets".to_string()),
                            order: Some(1.2),
                            level: Some(2),
                            preferred_label: None,
                            children: None,
                        },
                    ]),
                },
                NetworkNode {
                    concept_name: "Liabilities".to_string(),
                    concept_label: "Liabilities".to_string(),
                    parent: None,
                    order: Some(2.0),
                    level: Some(1),
                    preferred_label: None,
                    children: None,
                },
            ]),
        };
        
        let serialized = serde_json::to_string(&network).unwrap();
        let json: Value = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(json["id"], "net-123");
        assert_eq!(json["name"], "Statement of Financial Position");
        assert_eq!(json["short_name"], "Balance Sheet");
        assert_eq!(json["taxonomy"], "us-gaap");
        
        let nodes = json["nodes"].as_array().unwrap();
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0]["concept_name"], "Assets");
        
        let children = nodes[0]["children"].as_array().unwrap();
        assert_eq!(children.len(), 2);
        assert_eq!(children[0]["concept_name"], "CurrentAssets");
        assert_eq!(children[0]["parent"], "Assets");
        assert_eq!(children[1]["concept_name"], "NoncurrentAssets");
    }
    
    #[test]
    fn test_concept_serialization() {
        let concept = Concept {
            name: "Assets".to_string(),
            label: "Assets".to_string(),
            description: Some("Total assets of an entity".to_string()),
            taxonomy: "us-gaap".to_string(),
            type_name: "monetaryItemType".to_string(),
            period_type: Some("instant".to_string()),
            balance: Some("debit".to_string()),
            standard_label: Some("Assets, Total".to_string()),
            documentation: Some("Sum of the carrying amounts as of the balance sheet date...".to_string()),
            is_abstract: Some(false),
            is_nillable: Some(true),
            references: Some(vec![
                Reference {
                    name: "Publisher".to_string(),
                    value: "FASB".to_string(),
                },
                Reference {
                    name: "Standard".to_string(), 
                    value: "ASC".to_string(),
                },
            ]),
            dimensions: Some(vec![
                ConceptDimension {
                    dimension_name: "LegalEntityAxis".to_string(),
                    members: vec![
                        "ParentCompanyMember".to_string(),
                        "SubsidiaryMember".to_string(),
                    ],
                },
            ]),
        };
        
        let serialized = serde_json::to_string(&concept).unwrap();
        let json: Value = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(json["name"], "Assets");
        assert_eq!(json["label"], "Assets");
        assert_eq!(json["taxonomy"], "us-gaap");
        assert_eq!(json["type_name"], "monetaryItemType");
        assert_eq!(json["period_type"], "instant");
        assert_eq!(json["balance"], "debit");
        
        let references = json["references"].as_array().unwrap();
        assert_eq!(references.len(), 2);
        assert_eq!(references[0]["name"], "Publisher");
        assert_eq!(references[0]["value"], "FASB");
        
        let dimensions = json["dimensions"].as_array().unwrap();
        assert_eq!(dimensions.len(), 1);
        assert_eq!(dimensions[0]["dimension_name"], "LegalEntityAxis");
        
        let members = dimensions[0]["members"].as_array().unwrap();
        assert_eq!(members.len(), 2);
        assert_eq!(members[0], "ParentCompanyMember");
        assert_eq!(members[1], "SubsidiaryMember");
    }
}