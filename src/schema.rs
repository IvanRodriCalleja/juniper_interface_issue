use juniper::{graphql_interface, ID, graphql_object, RootNode, FieldResult, FieldError, graphql_value, EmptyMutation, EmptySubscription};


#[graphql_interface(for = [Faction, Ship])]
pub trait Node {
    fn id(&self) -> ID;
}

pub struct Faction {
    pub id: String,
    pub name: String,
}

#[graphql_interface]
impl Node for Faction {
    fn id(&self) -> ID {
        ID::new(format!("{}-{}", &self.id, "FactionNode"))
    }
}

#[graphql_object(impl = NodeValue)]
impl Faction {
    fn name(&self) -> &str {
        &self.name
    }
}


pub struct Ship {
    pub id: String,
    pub name: String,
}

#[graphql_interface]
impl Node for Ship {
    fn id(&self) -> ID {
        ID::new(format!("{}-{}", &self.id, "ShipNode"))
    }
}

#[graphql_object(impl = NodeValue)]
impl Ship {
    fn id(&self) -> ID {
        ID::new(format!("{}-{}", &self.id, "Ship"))
    }

    fn name(&self) -> &str {
        &self.name
    }
}

pub struct Query;

#[graphql_object]
impl Query {

    fn node(id: ID) -> FieldResult<NodeValue> {
        let bind = id.to_string();
        let segments = bind.split("-").collect::<Vec<&str>>();

        let (id, struct_name) = (ID::new(segments[0]), segments[1].to_string());

        match struct_name.as_str() {
            "ShipNode" => Ok(NodeValue::Ship(Ship {
                id: "ship_id".to_string(),
                name: "ship name".to_string()
            })),
            "FactionNode" => Ok(NodeValue::Faction(Faction {
                id: "faction_id".to_string(),
                name: "faction name".to_string()
            })),
            _ => Err(FieldError::new(
                "Node tpe not found",
                graphql_value!({
                    "type": "NOT_FOUND"
                }),
            ))
        }
    }

    fn ship() -> FieldResult<Ship> {
        Ok(Ship {
            id: "ship_id".to_string(),
            name: "ship name".to_string()
        })
    }

    fn faction() -> FieldResult<Faction> {
        Ok(Faction {
            id: "faction_id".to_string(),
            name: "faction name".to_string()
        })
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}