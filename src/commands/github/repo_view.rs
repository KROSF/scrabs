use graphql_client::GraphQLQuery;

type URI = String;
type DateTime = String;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "assets/graphql/schemas/github.json",
    query_path = "assets/graphql/queries/repository.gql",
    response_derives = "Debug"
)]
pub struct RepoView;
