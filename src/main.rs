use std::fs;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::Index;
use tantivy::ReloadPolicy;
use serde::Deserialize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build schema
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("curie", STORED);
    schema_builder.add_text_field("names", STORED);
    schema_builder.add_text_field("types", STORED);
    schema_builder.add_text_field("preferred_name", TEXT | STORED | FAST);
    schema_builder.add_u64_field("shortest_name_length", STORED);
    let schema = schema_builder.build();

    // Index documents
    let index_dir = Path::new("./index");
    fs::remove_dir_all(index_dir)?;
    fs::create_dir(index_dir)?;
    let index = Index::create_in_dir(&index_dir, schema.clone())?;
    let mut index_writer = index.writer(50_000_000)?;

    let mut count: usize = 0;
    let mut missing: usize = 0;
    let babel_files_dir = Path::new("./robokop-filtered-synonyms");
    for entry in fs::read_dir(babel_files_dir)? {
        let path = entry?.path();
        if path.is_file() {
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                let line_str = line?;
                let Ok(node): Result<BabelSchema, _> = serde_json::from_str(&line_str) else {
                    // println!("Missing field from line:\n{}", &line_str);
                    missing += 1;
                    continue;
                };

                
                let curie = schema.get_field("curie").unwrap();
                let names = schema.get_field("names").unwrap();
                let types = schema.get_field("types").unwrap();
                let preferred_name = schema.get_field("preferred_name").unwrap();
                let shortest_name_length = schema.get_field("shortest_name_length").unwrap();

                let mut doc = Document::default();
                doc.add_text(curie, node.curie);
                node.names.into_iter().for_each(| name | {
                    doc.add_text(names, name)
                });
                node.types.into_iter().for_each(| t | {
                    doc.add_text(types, t)
                });
                doc.add_text(preferred_name, node.preferred_name);
                doc.add_u64(shortest_name_length, node.shortest_name_length);
                index_writer.add_document(doc)?;

                count += 1;
                println!("Count: {},\t Missing: {}", count, missing);
            }
        }
        index_writer.commit()?;
    }

    // let index = Index::open_in_dir(index_dir)?;
    
    // Search
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;
    
    println!("Type 'q' to quit.");
    loop {
        print!("\nSearch: ");
        io::stdout().flush()?;

        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_ok() {
            let input = buf.trim();
            if input == "q" { break; }

            let t0 = Instant::now();
            let searcher = reader.searcher();
            let preferred_name = schema.get_field("preferred_name").unwrap();
            let query_parser = QueryParser::for_index(&index, vec![preferred_name]);
            let query = query_parser.parse_query(input)?;

            let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
            for (_score, doc_address) in top_docs {
                let retrieved_doc = searcher.doc(doc_address)?;
                println!("{}", schema.to_json(&retrieved_doc));
            }

            println!("{:?}", t0.elapsed());
        }
    }
    
    Ok(())
}

#[derive(Deserialize)]
struct BabelSchema {
    curie: String,
    names: Vec<String>,
    types: Vec<String>,
    preferred_name: String,
    shortest_name_length: u64,
}