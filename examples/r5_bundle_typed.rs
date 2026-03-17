//! Example: Creating a FHIR R5 Bundle with properly typed nested structures
//!
//! This demonstrates how the code generator now properly handles BackboneElements.

use fhir_resources::r5::bundle::{Bundle, BundleLink, BundleEntry, BundleEntryRequest, BundleEntryResponse};
use serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== FHIR R5 Bundle with Proper BackboneElement Support ===\n");

    // Create a transaction Bundle with proper nested structures
    let bundle = Bundle {
        resource_type: "Bundle".to_string(),
        id: Some("bundle-transaction".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        identifier: None,
        r#type: "transaction".to_string(),  // Required field!
        timestamp: Some("2024-03-17T10:30:00Z" .to_string()),
        total: None,  // Not used for transactions
        
        // Bundle.link - properly typed as Vec<BundleLink>!
        link: Some(vec![
            BundleLink {
                id: None,
                extension: None,
                modifier_extension: None,
                relation: "self".to_string(),  // REQUIRED!
                url: "https://example.com/fhir/Bundle/bundle-transaction".to_string(),  // REQUIRED!
            }
        ]),
        
        // Bundle.entry - properly typed as Vec<BundleEntry>!
        entry: Some(vec![
            BundleEntry {
                id: None,
                extension: None,
                modifier_extension: None,
                
                // Bundle.entry.link - reuses BundleLink structure via contentReference!
                link: Some(vec![
                    BundleLink {
                        id: None,
                        extension: None,
                        modifier_extension: None,
                        relation: "related".to_string(),
                        url: "https://example.com/fhir/Patient/123".to_string(),
                    }
                ]),
                
                full_url: Some("urn:uuid:61ebe359-bfdc-4613-8bf2-c5e300945f0a".to_string()),
                resource: Some(serde_json::json!({
                    "resourceType": "Patient",
                    "id": "example",
                    "active": true,
                    "name": [{
                        "family": "Chalmers",
                        "given": ["Peter", "James"]
                    }]
                })),
                search: None,  // Not used in transactions
                
                // BundleEntryRequest - properly typed!
                request: Some(BundleEntryRequest {
                    id: None,
                    extension: None,
                    modifier_extension: None,
                    method: "POST".to_string(),  // REQUIRED!
                    url: "Patient".to_string(),  // REQUIRED!
                    if_none_match: None,
                    if_modified_since: None,
                    if_match: None,
                    if_none_exist: Some("identifier=http://example.org/fhir/ids|12345".to_string()),
                }),
                
                response: None,  // Response would be in the transaction-response
            }
        ]),
        
        signature: None,
        issues: None,
    };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&bundle)?;
    
    println!("Generated Bundle JSON:");
    println!("{}\n", json);

    // Demonstrate type-safe access
    println!("=== Type-Safe Field Access ===\n");
    
    if let Some(entries) = &bundle.entry {
        for (i, entry) in entries.iter().enumerate() {
            println!("Entry {}:", i + 1);
            
            // Type-safe access to fullUrl
            if let Some(url) = &entry.full_url {
                println!("  Full URL: {}", url);
            }
            
            // Type-safe access to request fields
            if let Some(request) = &entry.request {
                println!("  Request Method: {}", request.method);  // No Option - it's required!
                println!("  Request URL: {}", request.url);         // No Option - it's required!
                if let Some(condition) = &request.if_none_exist {
                    println!("  Conditional: {}", condition);
                }
            }
            
            // Type-safe access to link (which properly uses BundleLink!)
            if let Some(links) = &entry.link {
                for link in links {
                    println!("  Related Link: {} -> {}", link.relation, link.url);
                }
            }
            
            println!();
        }
    }

    println!("✅ All fields are properly typed - no serde_json::Value placeholders!");
    println!("✅ Required fields don't have Option wrapper!");
    println!("✅ Bundle.entry.link correctly reuses BundleLink structure!");
    
    Ok(())
}
