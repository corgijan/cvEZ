use diesel::{Connection, RunQueryDsl, SelectableHelper, SqliteConnection};
use diesel::associations::HasTable;
use diesel::query_dsl::limit_dsl::LimitDsl;
use crate::models::sbom;
use crate::models::cve;
use crate::models::post::{Post, NewPost};
use crate::schema::posts;

mod models;
mod schema;

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
            dbg!(&a);
            for a in a.iter(){
                for a in a.external_references.iter(){
                    for a in a.reference.iter(){
                        for a in a.iter() {
                            if a.r#type.is_some() && a.r#type.as_ref().unwrap() == "git" {
                                repo1 = Some(strip_git_repo(a.url.as_ref().unwrap()));
                            }
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
    let database_url = "sqlite://database.sqlite3";
    let mut con = SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let new_post = NewPost { title: "Title", body: "ImaABody BODY", published: true };

    //diesel::insert_into(crate::schema::posts::table).values(&new_post).get_results::<Post>(&mut con).expect("Error inserting post");


    use self::schema::posts::dsl::*;
    let results: Vec<Post> = posts
        .load(&mut con)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}

fn strip_git_repo(repo: &str) -> String {
    let  repo = repo.replace("git://", "").replace("git+","").replace(".git","").replace("https://","").replace("http://","").replace("ssh://","").replace("git@","");
    return repo;
}

