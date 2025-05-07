mod ast_src;
mod utils;

use self::ast_src::{AstEnumSrc, AstNodeSrc, AstSrc, Cardinality, Field, KindsSrc};
use crate::codegen::{add_preamble, ensure_file_contents, reformat};
use crate::project_root;
use ast_src::generate_kind_src;
use either::Either;
use itertools::Itertools;
use proc_macro2::{Punct, Spacing};
use quote::{format_ident, quote};
use std::{collections::BTreeSet, fs};
use ungrammar::{Grammar, Rule};
use utils::{clean_token_name, pluralize, to_lower_snake_case, to_upper_snake_case};

pub(crate) fn generate(check: bool) {
    let grammar =
        fs::read_to_string(project_root().join("doc/yul.ungram")).unwrap().parse().unwrap();
    fs::write(project_root().join("doc/yul.grammar"), format!("{:#?}", grammar))
        .expect("write grammar failed");
    let ast = lower(&grammar);
    fs::write(project_root().join("doc/yul.ast"), format!("{:#?}", ast)).expect("write ast failed");
    let kinds_src = generate_kind_src(&ast.nodes, &ast.enums, &grammar);
    fs::write(project_root().join("doc/yul.kinds"), format!("{:#?}", kinds_src))
        .expect("write kinds failed");
    let syntax_kinds = generate_syntax_kinds(kinds_src);
    let syntax_kinds_file =
        project_root().join("crates/qi-compiler/src/yul/parser/syntax_kind/generated.rs");
    ensure_file_contents(
        crate::flags::CodegenType::Grammar,
        syntax_kinds_file.as_path(),
        &syntax_kinds,
        check,
    );
}

fn generate_syntax_kinds(grammar: KindsSrc) -> String {
    let (single_byte_tokens_values, single_byte_tokens): (Vec<_>, Vec<_>) = grammar
        .punct
        .iter()
        .filter(|(token, _name)| token.len() == 1)
        .map(|(token, name)| (token.chars().next().unwrap(), format_ident!("{}", name)))
        .unzip();

    let punctuation_values = grammar.punct.iter().map(|(token, _name)| {
        if "{}[]()".contains(token) {
            let c = token.chars().next().unwrap();
            quote! { #c }
            // underscore is an identifier in the proc-macro api
        } else if *token == "_" {
            quote! { _ }
        } else {
            let cs = token.chars().map(|c| Punct::new(c, Spacing::Joint));
            quote! { #(#cs)* }
        }
    });
    let punctuation =
        grammar.punct.iter().map(|(_token, name)| format_ident!("{}", name)).collect::<Vec<_>>();
    let punctuation_texts = grammar.punct.iter().map(|&(text, _name)| text);

    let fmt_kw_as_variant = |&name| match name {
        name => format_ident!("{}_KW", to_upper_snake_case(name)),
    };
    let strict_keywords = grammar.keywords;
    let strict_keywords_variants =
        strict_keywords.iter().map(fmt_kw_as_variant).collect::<Vec<_>>();
    let strict_keywords_tokens = strict_keywords.iter().map(|it| format_ident!("{it}"));

    let contextual_keywords = grammar.contextual_keywords;
    let contextual_keywords_variants =
        contextual_keywords.iter().map(fmt_kw_as_variant).collect::<Vec<_>>();
    let contextual_keywords_tokens = contextual_keywords.iter().map(|it| format_ident!("{it}"));
    let contextual_keywords_str_match_arm = grammar.contextual_keywords.iter().map(|kw| {
        quote! { #kw }
    });
    let contextual_keywords_variants_match_arm = grammar
        .contextual_keywords
        .iter()
        .map(|kw_s| {
            let kw = fmt_kw_as_variant(kw_s);
            quote! { #kw }
        })
        .collect::<Vec<_>>();

    let non_strict_keyword_variants =
        contextual_keywords_variants.iter().sorted().dedup().collect::<Vec<_>>();

    let literals =
        grammar.literals.iter().map(|name| format_ident!("{}", name)).collect::<Vec<_>>();

    let tokens = grammar.tokens.iter().map(|name| format_ident!("{}", name)).collect::<Vec<_>>();

    let nodes = grammar.nodes.iter().map(|name| format_ident!("{}", name)).collect::<Vec<_>>();

    let ast = quote! {
        #![allow(bad_style)]

        /// The kind of syntax node, e.g. `IDENT`.
        #[derive(Debug)]
        #[repr(u16)]
        pub enum SyntaxKind {
            // Technical SyntaxKinds: they appear temporally during parsing,
            // but never end up in the final tree
            #[doc(hidden)]
            TOMBSTONE,
            #[doc(hidden)]
            EOF,
            #(#punctuation,)*
            #(#strict_keywords_variants,)*
            #(#non_strict_keyword_variants,)*
            #(#literals,)*
            #(#tokens,)*
            #(#nodes,)*

            // Technical kind so that we can cast from u16 safely
            #[doc(hidden)]
            __LAST,
        }
        use self::SyntaxKind::*;

        impl SyntaxKind {
            #[allow(unreachable_patterns)]
            pub const fn text(self) -> &'static str {
                match self {
                    TOMBSTONE | EOF | __LAST
                    #( | #literals )*
                    #( | #nodes )*
                    #( | #tokens )* => panic!("no text for these `SyntaxKind`s"),
                    #( #punctuation => #punctuation_texts ,)*
                    #( #strict_keywords_variants => #strict_keywords ,)*
                    #( #contextual_keywords_variants => #contextual_keywords ,)*
                }
            }

            /// Strict keywords are identifiers that are always considered keywords.
            pub fn is_strict_keyword(self) -> bool {
                matches!(self, #(#strict_keywords_variants)|*)
                || match self {
                    _ => false,
                }
            }

            /// Weak keywords are identifiers that are considered keywords only in certain contexts.
            pub fn is_contextual_keyword(self) -> bool {
                match self {
                    #(#contextual_keywords_variants_match_arm => true,)*
                    _ => false,
                }
            }

            pub fn is_keyword(self) -> bool {
                matches!(self, #(#strict_keywords_variants)|*)
                || match self {
                    #(#contextual_keywords_variants_match_arm => true,)*
                    _ => false,
                }
            }

            pub fn is_punct(self) -> bool {
                matches!(self, #(#punctuation)|*)
            }

            pub fn is_literal(self) -> bool {
                matches!(self, #(#literals)|*)
            }

            pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
                let kw = match ident {
                    #(#strict_keywords => #strict_keywords_variants,)*
                    _ => return None,
                };
                Some(kw)
            }

            pub fn from_contextual_keyword(ident: &str) -> Option<SyntaxKind> {
                let kw = match ident {
                    #(#contextual_keywords_str_match_arm => #contextual_keywords_variants,)*
                    _ => return None,
                };
                Some(kw)
            }

            pub fn from_char(c: char) -> Option<SyntaxKind> {
                let tok = match c {
                    #(#single_byte_tokens_values => #single_byte_tokens,)*
                    _ => return None,
                };
                Some(tok)
            }
        }

        #[macro_export]
        macro_rules! T_ {
            #([#punctuation_values] => { $crate::SyntaxKind::#punctuation };)*
            #([#strict_keywords_tokens] => { $crate::SyntaxKind::#strict_keywords_variants };)*
            #([#contextual_keywords_tokens] => { $crate::SyntaxKind::#contextual_keywords_variants };)*
            [decimal_number] => { $crate::SyntaxKind::DECIMAL_NUMBER };
            [hex_number] => { $crate::SyntaxKind::HEX_NUMBER };
            [string_literal] => { $crate::SyntaxKind::STRING_LITERAL };
            [hex_literal] => { $crate::SyntaxKind::HEX_LITERAL };
            [ident] => { $crate::SyntaxKind::IDENT };
        }

        impl ::core::marker::Copy for SyntaxKind {}
        impl ::core::clone::Clone for SyntaxKind {
            #[inline]
            fn clone(&self) -> Self {
                *self
            }
        }
        impl ::core::cmp::PartialEq for SyntaxKind {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                (*self as u16) == (*other as u16)
            }
        }
        impl ::core::cmp::Eq for SyntaxKind {}
        impl ::core::cmp::PartialOrd for SyntaxKind {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> core::option::Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl ::core::cmp::Ord for SyntaxKind {
            #[inline]
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                (*self as u16).cmp(&(*other as u16))
            }
        }
        impl ::core::hash::Hash for SyntaxKind {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                ::core::mem::discriminant(self).hash(state);
            }
        }
    };

    add_preamble(crate::flags::CodegenType::Grammar, reformat(ast.to_string()))
}

impl AstNodeSrc {
    fn remove_field(&mut self, to_remove: Vec<usize>) {
        to_remove.into_iter().rev().for_each(|idx| {
            self.fields.remove(idx);
        });
    }
}

impl Field {
    fn is_many(&self) -> bool {
        matches!(self, Field::Node { cardinality: Cardinality::Many, .. })
    }
    fn token_kind(&self) -> Option<proc_macro2::TokenStream> {
        match self {
            Field::Token(token) => {
                let token: proc_macro2::TokenStream = token.parse().unwrap();
                Some(quote! { T![#token] })
            }
            _ => None,
        }
    }
    fn method_name(&self) -> String {
        match self {
            Field::Token(name) => {
                let name = match name.as_str() {
                    ":=" => "walrus",
                    "->" => "arrow",
                    "{" => "bracket_curly_left",
                    "}" => "bracket_curly_right",
                    "(" => "parenthesis_left",
                    ")" => "parenthesis_right",
                    "," => "comma",
                    ":" => "colon",
                    _ => name,
                };
                format!("{name}_token",)
            }
            Field::Node { name, .. } => {
                if name == "type" {
                    String::from("ty")
                } else {
                    name.to_owned()
                }
            }
        }
    }
    fn ty(&self) -> proc_macro2::Ident {
        match self {
            Field::Token(_) => format_ident!("SyntaxToken"),
            Field::Node { ty, .. } => format_ident!("{}", ty),
        }
    }
}

fn lower(grammar: &Grammar) -> AstSrc {
    let mut res = AstSrc {
        tokens: "Whitespace Comment StringLiteral HexNumber DecimalNumber HexLiteral Ident"
            .split_ascii_whitespace()
            .map(|it| it.to_owned())
            .collect::<Vec<_>>(),
        ..Default::default()
    };

    let nodes = grammar.iter().collect::<Vec<_>>();

    for &node in &nodes {
        let name = grammar[node].name.clone();
        let rule = &grammar[node].rule;
        match lower_enum(grammar, rule) {
            Some(variants) => {
                let enum_src = AstEnumSrc { name, variants };
                res.enums.push(enum_src);
            }
            None => {
                let mut fields = Vec::new();
                lower_rule(&mut fields, grammar, None, rule);
                res.nodes.push(AstNodeSrc { name, fields });
            }
        }
    }

    deduplicate_fields(&mut res);
    extract_enums(&mut res);
    res.nodes.sort_by_key(|it| it.name.clone());
    res.enums.sort_by_key(|it| it.name.clone());
    res.tokens.sort();
    res.nodes.iter_mut().for_each(|it| {
        it.fields.sort_by_key(|it| match it {
            Field::Token(name) => (true, name.clone()),
            Field::Node { name, .. } => (false, name.clone()),
        });
    });
    res.enums.iter_mut().for_each(|it| {
        it.variants.sort();
    });
    res
}

fn lower_enum(grammar: &Grammar, rule: &Rule) -> Option<Vec<String>> {
    let alternatives = match rule {
        Rule::Alt(it) => it,
        _ => return None,
    };
    let mut variants = Vec::new();
    for alternative in alternatives {
        match alternative {
            Rule::Node(it) => variants.push(grammar[*it].name.clone()),
            Rule::Token(it) if grammar[*it].name == ";" => (),
            _ => return None,
        }
    }
    Some(variants)
}

fn lower_rule(acc: &mut Vec<Field>, grammar: &Grammar, label: Option<&String>, rule: &Rule) {
    if lower_separated_list(acc, grammar, label, rule) {
        return;
    }

    match rule {
        Rule::Node(node) => {
            let ty = grammar[*node].name.clone();
            let name = label.cloned().unwrap_or_else(|| to_lower_snake_case(&ty));
            let field = Field::Node { name, ty, cardinality: Cardinality::Optional };
            acc.push(field);
        }
        Rule::Token(token) => {
            assert!(label.is_none());
            let mut name = clean_token_name(&grammar[*token].name);
            if "[]{}()".contains(&name) {
                name = format!("'{name}'");
            }
            let field = Field::Token(name);
            acc.push(field);
        }
        Rule::Rep(inner) => {
            if let Rule::Node(node) = &**inner {
                let ty = grammar[*node].name.clone();
                let name = label.cloned().unwrap_or_else(|| pluralize(&to_lower_snake_case(&ty)));
                let field = Field::Node { name, ty, cardinality: Cardinality::Many };
                acc.push(field);
                return;
            }
            panic!("unhandled rule: {rule:?}")
        }
        Rule::Labeled { label: l, rule } => {
            assert!(label.is_none());
            let manually_implemented =
                matches!(l.as_str(), "then_branch" | "condition" | "args" | "body");
            if manually_implemented {
                return;
            }
            lower_rule(acc, grammar, Some(l), rule);
        }
        Rule::Seq(rules) | Rule::Alt(rules) => {
            for rule in rules {
                lower_rule(acc, grammar, label, rule)
            }
        }
        Rule::Opt(rule) => lower_rule(acc, grammar, label, rule),
    }
}

// (T (',' T)* ','?)
fn lower_separated_list(
    acc: &mut Vec<Field>,
    grammar: &Grammar,
    label: Option<&String>,
    rule: &Rule,
) -> bool {
    let rule = match rule {
        Rule::Seq(it) => it,
        _ => return false,
    };

    let (nt, repeat, trailing_sep) = match rule.as_slice() {
        [Rule::Node(node), Rule::Rep(repeat), Rule::Opt(trailing_sep)] => {
            (Either::Left(node), repeat, Some(trailing_sep))
        }
        [Rule::Node(node), Rule::Rep(repeat)] => (Either::Left(node), repeat, None),
        [Rule::Token(token), Rule::Rep(repeat), Rule::Opt(trailing_sep)] => {
            (Either::Right(token), repeat, Some(trailing_sep))
        }
        [Rule::Token(token), Rule::Rep(repeat)] => (Either::Right(token), repeat, None),
        _ => return false,
    };
    let repeat = match &**repeat {
        Rule::Seq(it) => it,
        _ => return false,
    };
    if !matches!(
        repeat.as_slice(),
        [comma, nt_]
            if trailing_sep.is_none_or(|it| comma == &**it) && match (nt, nt_) {
                (Either::Left(node), Rule::Node(nt_)) => node == nt_,
                (Either::Right(token), Rule::Token(nt_)) => token == nt_,
                _ => false,
            }
    ) {
        return false;
    }
    match nt {
        Either::Right(token) => {
            let name = clean_token_name(&grammar[*token].name);
            let field = Field::Token(name);
            acc.push(field);
        }
        Either::Left(node) => {
            let ty = grammar[*node].name.clone();
            let name = label.cloned().unwrap_or_else(|| pluralize(&to_lower_snake_case(&ty)));
            let field = Field::Node { name, ty, cardinality: Cardinality::Many };
            acc.push(field);
        }
    }
    true
}

fn deduplicate_fields(ast: &mut AstSrc) {
    for node in &mut ast.nodes {
        let mut i = 0;
        'outer: while i < node.fields.len() {
            for j in 0..i {
                let f1 = &node.fields[i];
                let f2 = &node.fields[j];
                if f1 == f2 {
                    node.fields.remove(i);
                    continue 'outer;
                }
            }
            i += 1;
        }
    }
}

fn extract_enums(ast: &mut AstSrc) {
    for node in &mut ast.nodes {
        for enm in &ast.enums {
            let mut to_remove = Vec::new();
            for (i, field) in node.fields.iter().enumerate() {
                let ty = field.ty().to_string();
                if enm.variants.iter().any(|it| it == &ty) {
                    to_remove.push(i);
                }
            }
            if to_remove.len() == enm.variants.len() {
                node.remove_field(to_remove);
                let ty = enm.name.clone();
                let name = to_lower_snake_case(&ty);
                node.fields.push(Field::Node { name, ty, cardinality: Cardinality::Optional });
            }
        }
    }
}
