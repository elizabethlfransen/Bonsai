#![feature(proc_macro_diagnostic)]

use proc_macro::{Diagnostic, Level, Span, TokenStream};
use quote::quote;
use syn::{Attribute, Ident, Field, Item, ItemEnum, ItemStruct, Meta, Variant, parse_macro_input};

const ALLOW_ATTR: &str = "allow_cli_warning";
const ITEM_COMMAND: &str = "Command";
const ITEM_SUBCOMMAND: &str = "Subcommand";
const ITEM_ARG: &str = "Argument";
const KEY_NO_DOCS: &str = "no_docs";
const KEY_NO_EXAMPLES: &str = "no_examples";

#[proc_macro_attribute]
pub fn check_cli(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);

    match &input {
        Item::Struct(item) => check_cli_struct(item),
        Item::Enum(item) => check_subcommand_enum(item),
        _ => {}
    }
    quote!(#input).into()
}



fn check_cli_struct(item: &ItemStruct) {
    let struct_name = &item.ident;

    if is_root_command(item) && !has_docs(&item.attrs) {
        emit_docs_warning(struct_name, ITEM_COMMAND);
    }
    if is_root_command(item) && !has_examples(&item.attrs) {
        emit_examples_warning(struct_name, ITEM_COMMAND);
    }
    for field in &item.fields {
        check_arg_field(field);
    }
}

fn check_subcommand_enum(item: &ItemEnum) {
    if is_subcommand_enum(item) {
        for variant in &item.variants {
            check_subcommand_variant(variant);
        }
    }
}

fn check_subcommand_variant(variant: &Variant) {
    if !has_docs(&variant.attrs) {
        emit_docs_warning(&variant.ident, ITEM_SUBCOMMAND);
    }
    if !has_examples(&variant.attrs) {
        emit_examples_warning(&variant.ident, ITEM_SUBCOMMAND);
    }
}

fn check_arg_field(field: &Field) {
    if !has_docs(&field.attrs) {
        emit_docs_warning(field.ident.as_ref().unwrap(), ITEM_ARG);
    }
}

fn is_root_command(item: &ItemStruct) -> bool {
    return item
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("command"));
}

fn is_subcommand_enum(item: &ItemEnum) -> bool {
    item.attrs.iter().any(|attr| {
        attr.path().is_ident("derive") && quote!(#attr).to_string().contains("Subcommand")
    })
}

fn has_docs(attributes: &Vec<Attribute>) -> bool {
    attributes
        .iter()
        .any(|attr| is_allow_attr(attr, KEY_NO_DOCS) || attr.path().is_ident("doc"))
}

fn has_examples(attributes: &Vec<Attribute>) -> bool {
    return attributes.iter().any(|attr| {
        is_allow_attr(attr, KEY_NO_EXAMPLES)
        || attr.path().is_ident("command") && quote!(#attr).to_string().contains("examples!")
    });
}

fn is_allow_attr(attr: &Attribute, key: &str) -> bool {
    return if let Meta::List(items) = &attr.meta
        && attr.path().is_ident(ALLOW_ATTR)
    {
        items.tokens.to_string() == key
    } else {
        false
    };
}


fn emit_docs_warning(
    ident: &Ident,
    item_type: &str
) {
    Diagnostic::spanned(
            ident.span().unwrap(),
            Level::Warning,
            format!("{item_type} `{ident}` is missing documentation"),
        )
        .help("Add documentation to resolve this warning")
        .help(format!("Ignore this warning with #[{ALLOW_ATTR}({KEY_NO_DOCS})]"))
        .emit();
}

fn emit_examples_warning(
    ident: &Ident,
    item_type: &str
) {
    Diagnostic::spanned(
            ident.span().unwrap(),
            Level::Warning,
            format!("{item_type} `{ident}` is missing examples"),
        )
        .help("Add examples to resolve this warning")
        .help(format!("Ignore this warning with #[{ALLOW_ATTR}({KEY_NO_EXAMPLES})]"))
        .emit();
}