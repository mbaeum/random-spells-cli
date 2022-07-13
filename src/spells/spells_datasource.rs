use crate::datasources::graphql_datasource::*;
use crate::spells::spell_model::SpellModel;
use crate::spells::spell_queries::{spells_query, SpellsQuery};
use futures::executor::block_on;

#[derive(Debug)]
pub enum SpellsDataSourceError {
    GraphQLError(GraphQLAPIError),
    NoSpellsFound,
}

pub trait SpellsDataSource {
    fn get_all_spells(&self) -> Result<Vec<SpellModel>, SpellsDataSourceError>;
}

#[derive(Debug)]
pub struct SpellsGraphQLDataSource {
    api: GraphQLAPI,
}

impl SpellsGraphQLDataSource {
    pub fn new(api_url: String) -> Self {
        Self {
            api: GraphQLAPI::new(api_url),
        }
    }

    fn make_variables(&self) -> spells_query::Variables {
        spells_query::Variables { limit: Some(10) }
    }

    async fn get_all_raw_spells(
        &self,
    ) -> Result<spells_query::ResponseData, SpellsDataSourceError> {
        let variables = self.make_variables();
        let response_data = self
            .api
            .get_response_data::<spells_query::Variables, spells_query::ResponseData, SpellsQuery>(
                variables,
            )
            .await;
        match response_data {
            Ok(data) => Ok(data),
            Err(err) => Err(SpellsDataSourceError::GraphQLError(err)),
        }
    }
}

impl SpellsDataSource for SpellsGraphQLDataSource {
    fn get_all_spells(&self) -> Result<Vec<SpellModel>, SpellsDataSourceError> {
        let data = block_on(self.get_all_raw_spells());
        let spells = data?.spells;
        Ok(spells
            .into_iter()
            .map(|spell| {
                SpellModel::new(spell.name, spell.level, spell.desc, spell.url, spell.index)
            })
            .collect::<Vec<SpellModel>>())
    }
}
