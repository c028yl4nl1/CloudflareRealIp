use colored::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;
use std::{collections::HashSet, fs, process::exit};
use tld_extract::TLDExtract;
fn main() {
    config_threads(100);

    
    let all_domain = get_domain_extract_and_get_file();
    if let None = all_domain{
        eprintln!("{}", "Host no is valid".bright_red());
        eprintln!("Use ./bin host wordlist.txt");
        exit(1);
    }
    all_domain.unwrap().into_par_iter().for_each(|host| {
        let v = dns_unlock(&host);
        if let Some(ip) =  v {
            println!("valid: {} > {} ", host.bright_green(), ip);
        }
        else {
            eprintln!("invalid: {}", host.bright_red());
        }
    }
    );
    

   
   // inter.into_par_iter().for_each(|z| {});
}

fn config_threads(num_threads: usize) -> Result<(), Box<dyn std::error::Error>> {
    let _thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()?;

    Ok(())
}

fn get_domain_extract_and_get_file() -> Option<HashSet<String>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("{}", "Incorret use".bright_green());

        eprintln!("Use ./bin host wordlist.txt");
        exit(1);
    }

    let (host, file_txt) = (args.get(1).unwrap(), args.get(2).unwrap());

    // try extract host

    let url = extract_domain(&host)?;

    let url = tld_host_extract(url)?;

    if let Ok(databuffer_fille) = fs::read_to_string(file_txt) {
        let mut hash = HashSet::new();
        for line in databuffer_fille.lines() {
            let insert = format!("{}.{}", line, url);
            hash.insert(insert);
        }

        return Some(hash);
    } else {
        eprintln!("{}", "File no exist".bright_red());
    }

    None
}

fn extract_domain(url: &str) -> Option<String> {
    let re = Regex::new(r"(?:https?:\/\/)?((?:[\w-]+\.)+[\w-]+)").unwrap();

    if let Some(captures) = re.captures(url) {
        Some(captures[1].to_string())
    } else {
        None
    }
}

fn tld_host_extract<T: AsRef<str>>(host: T) -> Option<String> {
    let source = tld_extract::Source::Snapshot;
    let suffix = tld_extract::SuffixList::new(source, false, None);
    let mut extract = TLDExtract::new(suffix, true).unwrap();
    let extract_response = extract.extract(host.as_ref()).ok()?;
    extract_response.registered_domain
}


use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::Resolver;

fn dns_unlock(host: &str) -> Option<String> {
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    
    
    let v = resolver.lookup_ip(host);
    if let Ok(lookup) = v{
        if let Some (record_dns) =  lookup.as_lookup().records().get(1){
            if let Some(ip) = record_dns.clone().into_parts().rdata{
               let ip =  ip.as_a()?;
               let h = ip.to_string();
               println!("{}",h);
               return Some(h);
               
            }

        }
       
      
    }
    None
    
}
