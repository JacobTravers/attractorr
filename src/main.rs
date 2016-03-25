extern crate hyper;
extern crate select;
extern crate docopt;

mod torrent;

mod search_providers;
use search_providers::SearchProvider;
use search_providers::pirate_bay_search::PirateBaySearch;
use search_providers::kickass_search::KickassSearch;


static USAGE: &'static str = "
Usage:
  torrent-search <searchterm>...
  torrent-search (-h | --help)

Options:
  -h --help     Show this screen.
";

fn main() {
    // parse arguments
    let args = docopt::Docopt::new(USAGE).and_then(|d| d.parse())
        .unwrap_or_else(|e| e.exit());

    let keyword = args.get_vec("<searchterm>").join(" ");

    // create all search providers
    let providers: Vec<Box<SearchProvider>> = vec![
        Box::new(PirateBaySearch::new()),
        Box::new(KickassSearch::new()),
    ];

    // search for torrents
    let mut torrents = vec![];
    for provider in providers.iter() {
        match provider.search(&keyword) {
            Ok(results) => torrents.extend(results),
            Err(err) => println!("Error: {}", err),
        }
    }

    // print out all torrents
    for torrent in torrents.iter() {
        torrent.print();
    }
}
