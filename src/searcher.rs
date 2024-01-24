use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Lines},
    path::Path, time::Instant,
};

use serde::Deserialize;
use tantivy::{
    schema::{Schema, FAST, STORED, TEXT},
    Index,
};
use tracing::{error, info, warn};

#[derive(Debug)]
pub struct Searcher {
    schema: Option<Schema>,
    index: Option<Index>,
}

impl Searcher {
    pub fn new() -> Self {
        (Searcher {
            schema: None,
            index: None,
        })
        .build_schema()
        .index()
    }

    #[tracing::instrument()]
    fn build_schema(mut self) -> Self {
        info!("Start building schema");
        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("curie", STORED);
        schema_builder.add_text_field("names", STORED);
        schema_builder.add_text_field("types", STORED);
        schema_builder.add_text_field("preferred_name", TEXT | STORED | FAST);
        schema_builder.add_u64_field("shortest_name_length", STORED);
        self.schema = Some(schema_builder.build());
        self
    }

    fn index(mut self) -> Self {
        let timer = Instant::now();
      
        self.index = Some(Index::create_in_ram(
            self.schema
                .clone()
                .expect("Tried to index before building schema"),
        ));

        let mut count: usize = 0;

        for reader in Searcher::get_synonym_files() {
            for node in Searcher::get_nodes_from_lines(reader) {

              // INDEX HERE
              count += 1;

            }
        }

        info!("{}", count);

        info!("Indexing took {:.2?}", timer.elapsed());
        self
    }

    /// Helper to open the synonym directory and return an iterator of BufReaders
    /// over the valid synonym files.
    fn get_synonym_files() -> impl Iterator<Item = BufReader<File>> {
        let synonym_dir_path = Path::new("./robokop-filtered-synonyms");
        let synonym_dir = fs::read_dir(synonym_dir_path).expect("Error opening synonym directory.");
        let files = synonym_dir
        .filter_map(| entry | {
          match entry {
            Err(err) => { error!("Error obtaining a file in the synonym directory, skipping: {}", err); None },
            Ok(e) => Some(e.path()) 
          }
        })
        .filter_map(| path | {
          if !path.is_file() {
            warn!("Found a path ({:?}) in the synonyms directory that isn't a file, skipping.", path);
            return None;
          }
          let Ok(file) = fs::File::open(&path) else {
            error!("Issue opening a file ({:?}) in the synonyms directory, skipping", path);
            return None;
          };
          Some(io::BufReader::new(file))
        });
        files
    }

    /// Helper to get an iterator of valid `BabelSchema` nodes from a `BufReader<File>`.
    /// Skips invalid nodes and prints an error log giving the reason.
    fn get_nodes_from_lines(reader: BufReader<File>) -> impl Iterator<Item = BabelSchema> {
        reader.lines()
            .filter_map(| line_result: Result<String, io::Error> | {
              match line_result {
                Err(err) => { error!("Couldn't read string from a line, skipping: {}", err); None },
                Ok(line) => Some(line)
              }
            })
            .filter_map(| line: String | {
              match serde_json::from_str::<BabelSchema>(&line) {
                Err(err) => { error!("Unable to parse a node from this line:\n{}\nError: {}", &line, err); None },
                Ok(node) => Some(node)
              }
            })
    }
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct BabelSchema {
    curie: String,
    names: Vec<String>,
    types: Vec<String>,
    preferred_name: Option<String>,
    shortest_name_length: Option<u64>,
}
