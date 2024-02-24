use {
    crate::{utils::generate_js_class, ToWasmBindgen},
    proc_macro2::{Ident, TokenStream},
    quote::quote,
    serde::{Deserialize, Serialize},
    std::collections::BTreeMap,
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChromeApi {
    pub namespace: String,
    pub description: Option<String>,
    pub properties: Option<BTreeMap<String, Feature>>,
    pub types: Option<Vec<Type>>,
    pub functions: Option<Vec<Function>>,
    pub events: Option<Vec<Event>>,
}

impl ChromeApi {
    fn generate_types(&self, internal: &mut Vec<TokenStream>) {
        if let Some(types) = &self.types {
            for ty in types {
                let ty_ident = proc_macro2::Ident::new(&ty.id, proc_macro2::Span::call_site());

                let msg = if let Some(dsc) = &ty.description {
                    dsc.to_owned()
                } else {
                    "".into()
                };

                let js_class = generate_js_class(&ty.type_field);

                let js_name = ty.id.clone();

                internal.push(quote! {
                    #[wasm_bindgen(extends = #js_class, js_name = #js_name, typescript_type = #js_name)]
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    #[doc = #msg]
                    pub type #ty_ident;
                });

                if let Some(properties) = &ty.properties {
                    for (key, value) in properties.iter() {
                        let key = if key == "type" {
                            "type_".into()
                        } else {
                            key.to_owned()
                        };
                        let ident = Ident::new(&key, proc_macro2::Span::call_site());

                        let t = Ident::new(&ty.id, proc_macro2::Span::call_site());

                        let msg = if let Some(dsc) = &value.description {
                            dsc.to_owned()
                        } else {
                            "".into()
                        };

                        let optional = value.optional.unwrap_or(false);

                        let return_type = if let Some(r) = &value.type_field {
                            if r == "function" {
                                continue;
                            }
                            let gen = generate_js_class(r);
                            if optional {
                                quote!(Option<#gen>)
                            } else {
                                gen
                            }
                        } else if let Some(r) = &value.ref_field {
                            let gen = quote!(i32);

                            if optional {
                                quote!(Option<#gen>)
                            } else {
                                gen
                            }

                            // TODO: Fix this.
                        } else if value.choices.is_some() {
                            quote!(wasm_bindgen::JsValue)
                        } else {
                            panic!("no type, ref, or choices");
                        };

                        internal.push(quote! {
                            #[wasm_bindgen(method, getter, js_class = #ty_ident)]
                            #[doc = #msg]
                            pub fn #ident(this: &#t) -> #return_type;


                        });
                    }
                }
            }
        }
    }
}

impl ToWasmBindgen for ChromeApi {
    fn to_wasm_bindgen(&self) -> proc_macro2::TokenStream {
        let doc_msg = if let Some(description) = &self.description {
            description.to_owned()
        } else {
            "".into()
        };

        let mut internal = vec![];

        self.generate_types(&mut internal);

        let enclosing = quote! {
            #![allow(unused_imports)]
            #![allow(clippy::all)]
            use super::*;
            use wasm_bindgen::prelude::*;
            #[doc = #doc_msg]
            #[wasm_bindgen]
            extern "C" {
                #(#internal)*

            }

        };

        enclosing
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Feature {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(rename = "$ref")]
    pub ref_field: Option<String>,
    pub description: Option<String>,
    pub value: Option<Value>,
    pub deprecated: Option<String>,
    pub platforms: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Named((String, ValueType)),
    Value(i64),
    Object(BTreeMap<String, String>),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueType {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(rename = "$ref")]
    pub ref_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "enum")]
    pub enum_field: Option<Vec<EnumField>>,
    pub properties: Option<BTreeMap<String, TypeProperties>>,
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeProperties {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(rename = "$ref")]
    pub ref_field: Option<String>,
    pub choices: Option<Vec<ValueType>>,
    pub optional: Option<bool>,
    pub items: Option<SimpleType>,
    pub description: Option<String>,
    pub nodoc: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleType {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(rename = "$ref")]
    pub ref_field: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnumField {
    String(String),
    WithDescription(WithDescription),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithDescription {
    pub name: String,
    pub description: String,
    pub nodoc: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Function {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub nocompile: Option<bool>,
    pub deprecated: Option<String>,
    pub description: Option<String>,
    pub platforms: Option<Vec<String>>,
    pub parameters: Option<Vec<Parameter>>,
    #[serde(rename = "returns_async")]
    pub returns_async: Option<ReturnsAsync>,
    pub returns: Option<Returns>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Returns {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(rename = "$ref")]
    pub ref_field: Option<String>,
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub properties: Option<BTreeMap<String, ParameterProperties>>,
    #[serde(rename = "$ref")]
    pub ref_field: Option<String>,
    pub optional: Option<bool>,
    pub minimum: Option<i64>,
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProperties {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(rename = "$ref")]
    pub ref_field: Option<String>,
    pub preserve_null: Option<bool>,
    pub optional: Option<bool>,

    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReturnsAsync {
    pub name: String,
    pub parameters: Vec<ReturnParameter>,
    pub optional: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReturnParameter {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(rename = "$ref")]
    pub ref_field: Option<String>,
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub description: Option<String>,
    pub parameters: Option<Vec<EventParameter>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventParameter {
    pub name: String,
    #[serde(rename = "$ref")]
    pub ref_field: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}
