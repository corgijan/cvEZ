mod sbom;
mod cve;
use crate::sbom::SBom;

fn main() {
    let cve = include_str!("../cve.json");
    let cve: cve::CVERecord = serde_json::from_str(&cve).unwrap();
    let sbom = include_str!("../sbom.xml");
    let sbom: sbom::SBom = serde_xml_rs::from_str(&sbom).unwrap();

    let mut repo1 = None;
    let mut repo2 = None;

    // check if affected.repo is in sbom
    for affected in sbom.components {
        for a in affected.component.iter(){
            for a in a.iter(){
                for a in a.external_references.iter(){
                    for a in a.reference.iter(){
                        for a in a.iter() {
                            if a.url.is_some() {
                                repo1 = Some(strip_git_repo(a.url.as_ref().unwrap()));
                            }
                        }
                    }
                }
            }
        }
    }

    for a in cve.containers.iter(){
        for a in a.cna.iter(){
            for a in a.affected.iter(){
                dbg!(a);
                for a in a.iter(){
                    if a.repo.is_some(){
                        repo2 = Some(strip_git_repo(a.repo.as_ref().unwrap()));
                    }
                }
            }
        }
    }
    dbg!(repo1);
    dbg!(repo2);


    println!("Hello, world!");
}

fn strip_git_repo(repo: &str) -> String {
    let  repo = repo.replace("git://", "").replace("git+","").replace(".git","").replace("https://","").replace("http://","").replace("ssh://","").replace("git@","");
    return repo;
}

