
mod utils;
mod engine;

fn main() {
  let conf = match utils::args::parse() {
    Ok(conf) => conf, 
    Err(e) => panic!("Fatal error : {}", e) 
  };
  if conf.is_helping {
    utils::args::display_helping();
    std::process::exit(0);
  }
  // println!("{:?}", conf);
  // println!(
  //   "{:?}", 
  //   utils::json::load("
  //     {\"ok\": null}
  //   ")
  // );

  let mut doc = engine::Document::from_str("
    Ceci est un texte{# 
      avec un 
      commentaire 
      multilignes
    #}{{ fdsfds }}!");
  match doc.parse_parts() {
    Ok(true) => println!("{:?}", doc.debug_stack()),
    Ok(false) => print!("rien à faire"),
    Err(err) => {
      println!("Fatal error during parsing : {:?}", err);
      std::process::exit(1);
    }
  }
  println!("resolve = {:?}",doc.resolve());
  doc = doc.transform(); 
  println!("transform = {:?}", doc);
}
