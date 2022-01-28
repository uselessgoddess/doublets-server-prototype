use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
#[graphql(name = "order_by")]
pub enum OrderBy {
    /// in the ascending order, nulls last
    #[graphql(name = "asc")]
    Asc,

    /// in the ascending order, nulls first
    #[graphql(name = "asc_nulls_first")]
    AscNullsFirst,

    /// in the ascending order, nulls last
    #[graphql(name = "asc_nulls_last")]
    AscNullsLast,

    // in the descending order, nulls first
    #[graphql(name = "desc")]
    Desc,
    /// in the descending order, nulls first
    #[graphql(name = "desc_nulls_first")]
    DescNullsFirst,

    /// in the descending order, nulls last
    #[graphql(name = "desc_nulls_last")]
    DescNullsLast,
}
