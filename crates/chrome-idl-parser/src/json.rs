use {
    crate::{
        utils::{create_fully_qualified, generate_js_class},
        ToWasmBindgen,
    },
    proc_macro2::{Ident, Span, TokenStream},
    quote::quote,
    serde::{Deserialize, Serialize},
    std::collections::BTreeMap,
    to_snake_case::ToSnakeCase,
    wasm_bindgen_backend::util::rust_ident,
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
    fn generate_parameters(param: &Parameter) -> TokenStream {
        let typ = if let Some(type_field) = &param.type_field {
            generate_js_class(type_field)
        } else if let Some(ref_field) = &param.ref_field {
            if ref_field.contains('.') {
                create_fully_qualified(&ref_field)
            } else {
                let ident = Ident::new(&ref_field, Span::call_site());

                quote!(#ident)
            }
        } else if param.choices.is_some() {
            quote!(::wasm_bindgen::JsValue)
        } else {
            panic!("Missing type and ref")
        };

        let typ = if param.optional.unwrap_or(false)
            && !param.choices.is_some()
            && param.type_field != Some("any".to_string())
        {
            quote!(Option<#typ>)
        } else {
            typ
        };

        let ident_name = Ident::new(&param.name.to_snake_case(), Span::call_site());

        quote!(#ident_name: #typ)
    }

    fn generate_arguments(param: &Parameter) -> TokenStream {
        let ident_name = Ident::new(&param.name.to_snake_case(), Span::call_site());

        quote!(#ident_name)
    }

    fn generate_functions(&self, internal: &mut Vec<TokenStream>, external: &mut Vec<TokenStream>) {
        for func in self.functions.iter().flat_map(|v| v.iter()) {
            let js_name = self.namespace.clone() + "." + func.name.as_str();
            let ident = rust_ident(&func.name.clone().to_snake_case());

            let ident_callback = rust_ident(&(func.name.clone().to_snake_case() + "_callback"));

            let ext_rust_function_name = rust_ident(
                &(self.namespace.clone().replace('.', "_").to_snake_case()
                    + "_"
                    + func.name.to_snake_case().as_str()),
            );
            let ext_rust_function_name_callback = rust_ident(
                &(self.namespace.clone().replace('.', "_").to_snake_case()
                    + "_"
                    + func.name.to_snake_case().as_str()
                    + "_callback"),
            );

            let params: Vec<TokenStream> = func
                .parameters
                .iter()
                .flat_map(|el| el.iter())
                .map(Self::generate_parameters)
                .collect();
            let args: Vec<_> = func
                .parameters
                .iter()
                .flat_map(|el| el.iter())
                .map(Self::generate_arguments)
                .collect();

            let msg = func.description.clone().unwrap_or_default();

            if let Some(ret) = &func.returns {
                let ret_val = if let Some(type_field) = &ret.type_field {
                    Some(generate_js_class(type_field))
                } else if let Some(ref_field) = &ret.ref_field {
                    if ref_field.contains('.') {
                        Some(create_fully_qualified(&ref_field))
                    } else {
                        let ident = Ident::new(&ref_field, Span::call_site());

                        Some(quote!(#ident))
                    }
                } else {
                    None
                };

                if let Some(ret_val) = ret_val {
                    internal.push(quote!(
                            #[doc = #msg]
                            #[wasm_bindgen(js_name = #js_name)]
                            pub fn #ident(#(#params),*) -> #ret_val;

                    ));
                    external.push(quote!(

                        #[wasm_bindgen]
                        pub fn #ext_rust_function_name(#(#params),*) -> #ret_val {
                            #ident(#(#args),*)
                        }
                    ));
                } else {
                    internal.push(quote!(
                            #[doc = #msg]
                            #[wasm_bindgen(js_name = #js_name)]
                            pub fn #ident(#(#params),*);

                    ));
                    external.push(quote!(

                        #[wasm_bindgen]
                        pub fn #ext_rust_function_name(#(#params),*) {
                            #ident(#(#args),*)
                        }
                    ));
                }
            } else if let Some(ret) = &func.returns_async {
                let ret_val = if let Some(type_field) = &ret.type_field {
                    Some(generate_js_class(type_field))
                } else if let Some(ref_field) = &ret.ref_field {
                    if ref_field.contains('.') {
                        Some(create_fully_qualified(&ref_field))
                    } else {
                        let ident = Ident::new(&ref_field, Span::call_site());

                        Some(quote!(#ident))
                    }
                } else if let Some(params) = &ret.parameters {
                    let param = params.first();

                    if let Some(param) = param {
                        if let Some(type_field) = &param.type_field {
                            Some(generate_js_class(type_field))
                        } else if let Some(ref_field) = &param.ref_field {
                            if ref_field.contains('.') {
                                Some(create_fully_qualified(&ref_field))
                            } else {
                                let ident = Ident::new(&ref_field, Span::call_site());

                                Some(quote!(#ident))
                            }
                        } else if param.choices.is_some() {
                            Some(quote!(::wasm_bindgen::JsValue))
                        } else {
                            panic!("missing ref and type field")
                        }
                    } else {
                        None
                    }
                } else {
                    panic!("missing ref and type field")
                };

                if let Some(ret_val) = ret_val {
                    internal.push(quote!(
                           #[doc = #msg]
                           #[wasm_bindgen(js_name = #js_name, catch)]
                           pub async fn #ident(#(#params),*) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue>;
                    ));
                    external.push(quote!(

                        #[wasm_bindgen]
                        pub async fn #ext_rust_function_name(#(#params),*) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
                            #ident(#(#args),*).await
                        }
                    ));
                } else {
                    internal.push(quote!(
                           #[doc = #msg]
                           #[wasm_bindgen(js_name = #js_name, catch)]
                           pub async fn #ident(#(#params),*) -> Result<(), ::wasm_bindgen::JsValue>;
                    ));
                    external.push(quote!(

                        #[wasm_bindgen]
                        pub async fn #ext_rust_function_name(#(#params),*) -> Result<(), ::wasm_bindgen::JsValue> {
                            #ident(#(#args),*).await
                        }
                    ));
                }
                if params.is_empty() {
                    internal.push(quote!(
                       #[doc = #msg]
                       #[wasm_bindgen(js_name = #js_name)]
                        pub fn #ident_callback(callback: &::js_sys::Function);

                    ));
                    external.push(quote!(

                        #[wasm_bindgen]
                        pub fn #ext_rust_function_name_callback(callback: &::js_sys::Function) {
                            #ident_callback(callback);
                        }
                    ));
                } else {
                    internal.push(quote!(
                       #[doc = #msg]
                       #[wasm_bindgen(js_name = #js_name)]
                        pub fn #ident_callback(#(#params),*, callback: &::js_sys::Function);

                    ));

                    external.push(quote!(

                        #[wasm_bindgen]
                        pub fn #ext_rust_function_name_callback(#(#params),*, callback: &::js_sys::Function) {
                            #ident_callback(#(#args),*, callback);
                        }
                    ));
                }
            }
        }
    }

    fn generate_types(&self, internal: &mut Vec<TokenStream>) {
        if let Some(types) = &self.types {
            for ty in types {
                let type_name = if ty.id.contains('.') {
                    let mut iter = ty.id.chars().skip_while(|el| *el != '.');
                    iter.next();
                    iter.collect::<String>()
                } else {
                    ty.id.clone()
                };

                let ty_ident = rust_ident(&type_name);

                let msg = if let Some(dsc) = &ty.description {
                    dsc.to_owned()
                } else {
                    "".into()
                };

                let js_name = self.namespace.clone() + "." + ty.id.as_str();

                if let Some(t_field) = &ty.type_field {
                    let js_class = generate_js_class(t_field);
                    if t_field == "any" {
                        internal.push(quote! {
                            #[wasm_bindgen(extends = #js_class, js_name = #js_name, typescript_type = #js_name)]
                            #[derive(Debug, Clone, PartialEq)]
                            #[doc = #msg]
                            pub type #ty_ident;
                        });
                    } else {
                        internal.push(quote! {
                        #[wasm_bindgen(extends = #js_class, js_name = #js_name, typescript_type = #js_name)]
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        #[doc = #msg]
                        pub type #ty_ident;
                    });
                    }
                } else {
                    internal.push(quote! {
                        #[wasm_bindgen(js_name = #js_name, typescript_type = #js_name)]
                        #[derive(Debug, Clone, PartialEq)]
                        #[doc = #msg]
                        pub type #ty_ident;
                    });
                };

                if let Some(properties) = &ty.properties {
                    for (key, value) in properties.iter() {
                        let key = if key == "type" {
                            "type_".into()
                        } else {
                            key.to_owned()
                        };
                        let ident = Ident::new(&key, proc_macro2::Span::call_site());

                        let t = ty_ident.clone();

                        let msg = if let Some(dsc) = &value.description {
                            dsc.to_owned()
                        } else {
                            "".into()
                        };

                        let optional = value.optional.unwrap_or(false);

                        let return_type = if let Some(r) = &value.type_field {
                            if r == "function" {
                                // TODO: Fix this
                                continue;
                                // TODO: implement function return values.
                            }

                            if r == "any" {
                                quote!(::wasm_bindgen::JsValue)
                            } else {
                                let gen = generate_js_class(r);
                                if optional {
                                    quote!(Option<#gen>)
                                } else {
                                    gen
                                }
                            }
                        } else if let Some(r) = &value.ref_field {
                            let id = if r.contains('.') {
                                create_fully_qualified(&r)
                            } else {
                                let ident = Ident::new(&r, Span::call_site());

                                quote!(#ident)
                            };

                            let gen = quote!(#id);

                            if optional {
                                quote!(Option<#gen>)
                            } else {
                                gen
                            }
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
        let mut external = vec![];

        self.generate_types(&mut internal);

        self.generate_functions(&mut internal, &mut external);

        let enclosing = quote! {
            #![allow(unused_imports)]
            #![allow(clippy::all)]
            use super::*;
            use wasm_bindgen::prelude::*;
            #[doc = #doc_msg]
            #[wasm_bindgen(js_namespace = chrome)]
            extern "C" {
                #(#internal)*

            }
            #(#external)*
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
    Array(Vec<String>),
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
    pub type_field: Option<String>,
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
    pub optional: Option<bool>,
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
    pub choices: Option<Vec<ValueType>>,
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
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(rename = "$ref")]
    pub ref_field: Option<String>,
    pub parameters: Option<Vec<ReturnParameter>>,
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
    pub choices: Option<Vec<ValueType>>,
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
