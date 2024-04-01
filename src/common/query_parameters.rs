/// Query parameters for OpenAI API requets.
#[derive(Debug, Default, Clone)]
pub struct OpenAIQueryParameters {
    /// A cursor for use in pagination. after is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with obj_foo,
    /// your subsequent call can include after=obj_foo in order to fetch the next page of the list.
    pub after: Option<String>,

    /// A cursor for use in pagination. before is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with obj_foo,
    /// your subsequent call can include before=obj_foo in order to fetch the previous page of the list.
    pub before: Option<String>,

    /// A limit on the number of objects to be returned.
    pub limit: Option<u32>,

    /// Sort order by the created_at timestamp of the objects. asc for ascending order and desc for descending order.
    pub order: Option<String>,

    /// Only return files with the given purpose.
    pub purpose: Option<String>,
}

impl OpenAIQueryParameters {
    /// Create a new query parameters object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the after ID for pagination.
    pub fn after<S: Into<String>>(&mut self, after: S) -> &mut Self {
        self.after = Some(after.into());
        self
    }

    /// Sets the before ID for pagination.
    pub fn before<S: Into<String>>(&mut self, before: S) -> &mut Self {
        self.before = Some(before.into());
        self
    }

    /// Set a limit on the number of objects to be returned.
    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// Set the sort order by the created_at timestamp of the objects. asc for ascending order and desc for descending order.
    pub fn order<S: Into<String>>(&mut self, order: S) -> &mut Self {
        self.order = Some(order.into());
        self
    }

    /// Only return files with the given purpose.
    pub fn purpose<S: Into<String>>(&mut self, purpose: S) -> &mut Self {
        self.purpose = Some(purpose.into());
        self
    }

    /// Convert the query parameters to a header map.
    pub fn to_query(self) -> Vec<(String, String)> {
        let mut query = vec![];

        if let Some(after) = self.after {
            query.push(("after".to_string(), after));
        }

        if let Some(before) = self.before {
            query.push(("before".to_string(), before));
        }

        if let Some(limit) = self.limit {
            query.push(("limit".to_string(), limit.to_string()));
        }

        if let Some(order) = self.order {
            query.push(("order".to_string(), order));
        }

        if let Some(purpose) = self.purpose {
            query.push(("purpose".to_string(), purpose));
        }

        query
    }
}
