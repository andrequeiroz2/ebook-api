use serde::{Serialize, Deserialize};
use mongodb::bson::{bson, doc, Document};
use std::fmt::Debug;
use mongodb::bson;

#[derive(Clone, Serialize, Deserialize)]
pub enum OperatorsLogical{
    and,
    not,
    nor,
    or,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OperatorsComparison {
    eq,
    gt,
    gte,
    in_,
    lt,
    lte,
    ne,
    nin,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Fields{
    title,
    genre,
    pages,
    author
}

#[serde(untagged)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Value{
    TypeInt(u32),
    TypeStr(String),
    TypeVec(Vec<u32>)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition{
    field: Fields,
    operator_comparison: OperatorsComparison,
    value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestFilter{
    pub operator_logical: String,
    pub condition: Vec<Condition>
}

impl RequestFilter {
    
    pub fn builder(&self) -> Document{
        
        let mut request_filter = RequestFilter{
            operator_logical: self.operator_logical.clone(),
            condition: self.condition.clone(),
        };
        
        self.buinder_filter(&mut request_filter)
    }

    fn buinder_filter(&self, request_filter: &mut RequestFilter)-> Document{
        
        let operator_logical = format!("${}", request_filter.operator_logical);
        let condition = &mut request_filter.condition;
        let mut condition_element: Vec<Document> = Vec::new();

        for cond in condition.iter_mut(){

            let cond_field = format!("{:?}", cond.field);
            // let cond_comparison = format!("${:?}", cond.operator_comparison);
            
            let cond_comparison = if format!("{:?}", cond.operator_comparison) == String::from("in_"){
                format!("${:?}", cond.operator_comparison).replace("_", "")
            }else{
                format!("${:?}", cond.operator_comparison)
            };
            
            let cond_value_teste: bson::Bson = match cond.value.clone() {
                Value::TypeInt(v) => bson!(v),
                Value::TypeStr(v) => bson!(v),
                Value::TypeVec(v) => bson!(v),
            };

            let sentence = doc!{cond_field:{cond_comparison:  cond_value_teste}};
            condition_element.push(sentence)
        }

        let filter = doc!{operator_logical:condition_element};
        
        println!("{}", filter);

        filter

    }
    
}

    