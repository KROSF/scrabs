use anyhow::{anyhow, Result};
use graphql_client::{GraphQLQuery, Response};
use reqwest::blocking::Client;

pub struct RepoView;
pub mod repo_view {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &'static str = "RepoView";
    pub const QUERY : & 'static str = "query RepoView($owner: String!, $name: String!) {\n  repository(owner: $owner, name: $name) {\n    homepageUrl\n    owner {\n      login\n      avatarUrl\n      __typename\n    }\n    url\n    createdAt\n    languages(orderBy: { field: SIZE, direction: DESC }, first: 1) {\n      edges {\n        node {\n          name\n        }\n      }\n    }\n    stargazers {\n      totalCount\n    }\n  }\n}\n" ;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[doc = "An ISO-8601 encoded UTC date string."]
    type DateTime = String;
    #[doc = "An RFC 3986, RFC 3987, and RFC 6570 (level 4) compliant URI string."]
    type URI = String;
    #[derive(Deserialize)]
    #[serde(tag = "__typename")]
    pub enum RepoViewRepositoryOwnerOn {
        User,
        Organization,
    }
    #[derive(Deserialize)]
    pub struct RepoViewRepositoryOwner {
        #[doc = "The username used to login."]
        pub login: String,
        #[doc = "A URL pointing to the owner's public avatar."]
        #[serde(rename = "avatarUrl")]
        pub avatar_url: URI,
        #[serde(flatten)]
        pub on: RepoViewRepositoryOwnerOn,
    }
    #[derive(Deserialize)]
    #[doc = "Represents a given language found in repositories."]
    pub struct RepoViewRepositoryLanguagesEdgesNode {
        #[doc = "The name of the current language."]
        pub name: String,
    }
    #[derive(Deserialize)]
    #[doc = "Represents the language of a repository."]
    pub struct RepoViewRepositoryLanguagesEdges {
        pub node: RepoViewRepositoryLanguagesEdgesNode,
    }
    #[derive(Deserialize)]
    #[doc = "A list of languages associated with the parent."]
    pub struct RepoViewRepositoryLanguages {
        #[doc = "A list of edges."]
        pub edges: Option<Vec<Option<RepoViewRepositoryLanguagesEdges>>>,
    }
    #[derive(Deserialize)]
    #[doc = "The connection type for User."]
    pub struct RepoViewRepositoryStargazers {
        #[doc = "Identifies the total count of items in the connection."]
        #[serde(rename = "totalCount")]
        pub total_count: Int,
    }
    #[derive(Deserialize)]
    #[doc = "A repository contains the content for a project."]
    pub struct RepoViewRepository {
        #[doc = "The repository's URL."]
        #[serde(rename = "homepageUrl")]
        pub homepage_url: Option<URI>,
        #[doc = "The User owner of the repository."]
        pub owner: RepoViewRepositoryOwner,
        #[doc = "The HTTP URL for this repository"]
        pub url: URI,
        #[doc = "Identifies the date and time when the object was created."]
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[doc = "A list containing a breakdown of the language composition of the repository."]
        pub languages: Option<RepoViewRepositoryLanguages>,
        #[doc = "A list of users who have starred this starrable."]
        pub stargazers: RepoViewRepositoryStargazers,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub owner: String,
        pub name: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[doc = "Lookup a given repository by the owner and repository name."]
        pub repository: Option<RepoViewRepository>,
    }
}
impl graphql_client::GraphQLQuery for RepoView {
    type Variables = repo_view::Variables;
    type ResponseData = repo_view::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: repo_view::QUERY,
            operation_name: repo_view::OPERATION_NAME,
        }
    }
}

fn parse_repo_name(repo_name: &str) -> Result<(&str, &str)> {
    let mut parts = repo_name.split('/');

    match (parts.next(), parts.next()) {
        (Some(owner), Some(name)) => Ok((owner, name)),
        _ => Err(anyhow!(
            "wrong format for the repository name param (we expect something like krosf/scrabs)"
        )),
    }
}

pub fn repository(repo_name: &str) -> Result<Response<repo_view::ResponseData>> {
    let (owner, name) = match parse_repo_name(&repo_name) {
        Ok(repo) => repo,
        Err(_) => {
            return Err(anyhow!(""));
        }
    };

    let q = RepoView::build_query(repo_view::Variables {
        owner: owner.to_string(),
        name: name.to_string(),
    });

    let client = Client::new();

    let res = client
        .post("https://api.github.com/graphql")
        .bearer_auth(dotenv::var("GITHUB_TOKEN").expect("GitHub Toke"))
        .json(&q)
        .send()?;

    match res.json::<Response<repo_view::ResponseData>>() {
        Ok(res) => Ok(res),
        Err(e) => Err(anyhow!(e.to_string())),
    }
}
