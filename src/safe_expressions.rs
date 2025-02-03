use std::marker::PhantomData;
use crate::column::Table;
use crate::expressions::{Expression, RawTypes};
pub struct SafeExpr<ExprType, AllowedTables> {
    pub tables: PhantomData<AllowedTables>,
    pub(crate) type_val: PhantomData<ExprType>,
    pub(crate) expr: Expression,
}

impl<ExprType, AllowedTables> SafeExpr<ExprType, AllowedTables>
{
    pub fn new(expr: Expression) -> Self {
        SafeExpr{
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<ExprType>,
            expr,
        }
    }

    // list of functions that accept self and return SafeExpr<U>, where T can different than U

    pub fn to_string(self) -> SafeExpr<String, AllowedTables> {
        SafeExpr{
            type_val: PhantomData::<String>,
            tables: PhantomData::<AllowedTables>,
            expr: self.expr,
        }
    }

    pub fn basic(val: ExprType) -> SafeExpr<ExprType, AllowedTables>
    where
        ExprType: Into<RawTypes>
    {
        SafeExpr {
            tables: PhantomData::<AllowedTables>,
            type_val: PhantomData::<ExprType>,
            expr: Expression::Raw(val.into()),
        }
    }
}

mod tests {
    use crate::create_a_name::Queryable;
    use crate::expressions::Expression;
    use crate::literals::{Literal, Number};
    use crate::safe_expressions::SafeExpr;

    #[test]
    fn get_basic_type() {
        let basic: SafeExpr<_, ()> = SafeExpr::basic(Number::Int(10));
        let check = SafeExpr::<Literal, ()>::new(Expression::Raw(Literal::NumberLit(10.into()).into()).into());

        println!("{}", basic.expr.to_query());

        assert_eq!(basic.expr.to_query(), check.expr.to_query())
    }
}