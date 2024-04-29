
mod utils;
mod engine;

fn main() {

  let conf = match utils::args::parse() {
    Ok(conf) => conf, 
    Err(err) => {
      println!("Fatal error : {}", err);
      std::process::exit(1);
    }
  };
  if conf.is_helping {
    utils::args::display_helping();
    std::process::exit(0);
  }
  if conf.is_debugging {
    println!("conf = {:?}", conf);
  }

  let mut env = engine::Environment::from_args(&conf);

  // Prise en charge du STDIN ou path input à faire 
  let mut doc = engine::Document::from_str("
    Ceci est un texte{# 
      avec un 
      commentaire 
      multilignes
    #}{{ mavar }}!");
  
  match doc.parse_parts() {
    Ok(r) => if conf.is_debugging {
      if r {
        println!("Parse parts = part(s) found (n = {})", doc.stack_len());
      } else {
        println!("Parse parts = nothing to do");
      }
    }
    Err(err) => {
      println!("Fatal error during parsing : {:?}", err);
      std::process::exit(1);
    }
  }
  if conf.is_debugging {
    println!("{}", doc.debug_stack())
  }

  match doc.resolve(&mut env) {
    Ok(changed) => {
      if conf.is_debugging {
        println!("Resolved, source changed = {:?}", changed);
      }
      if changed {
        doc = doc.transform(); 
      }
    }
    Err(err) => {
      println!("Error during resolving = {:?}", err);
      std::process::exit(1);
    }
  }
  
  // prise en charge de l'output en fichier et non en console à faire 
  match conf.output {
    Some(_) => unimplemented!(),
    None => println!("{}", doc.source)
  }

  std::process::exit(0);
}
