use super::utils::to_upper_snake_case;

#[derive(Copy, Clone, Debug)]
pub(crate) struct KindsSrc {
    pub(crate) punct: &'static [(&'static str, &'static str)],
    pub(crate) keywords: &'static [&'static str],
    pub(crate) contextual_keywords: &'static [&'static str],
    pub(crate) literals: &'static [&'static str],
    pub(crate) tokens: &'static [&'static str],
    pub(crate) nodes: &'static [&'static str],
    pub(crate) _enums: &'static [&'static str],
}

/// The punctuations of the language.
const PUNCT: &[(&str, &str)] = &[
    (":=", "WALRUS"),
    ("->", "ARROW"),
    ("{", "BRACKET_CURLY_LEFT"),
    ("}", "BRACKET_CURLY_RIGHT"),
    ("(", "PARENTHESIS_LEFT"),
    (")", "PARENTHESIS_RIGHT"),
    (",", "COMMA"),
    (":", "COLON"),
];
const TOKENS: &[&str] = &["ERROR", "WHITESPACE", "COMMENT"];

const EOF: &str = "EOF";

const RESERVED: &[&str] = &[];

// keywords that are keywords only in specific parse contexts
const CONTEXTUAL_KEYWORDS: &[&str] = &["data"];

pub(crate) fn generate_kind_src(
    nodes: &[AstNodeSrc],
    enums: &[AstEnumSrc],
    grammar: &ungrammar::Grammar,
) -> KindsSrc {
    let mut contextual_keywords: Vec<&_> = CONTEXTUAL_KEYWORDS.to_vec();

    let mut keywords: Vec<&_> = Vec::new();
    let mut tokens: Vec<&_> = TOKENS.to_vec();
    let mut literals: Vec<&_> = Vec::new();
    let mut used_puncts = vec![false; PUNCT.len()];
    grammar.tokens().for_each(|token| {
        let name = &*grammar[token].name;
        if name == EOF {
            return;
        }
        match name.split_at(1) {
            ("@", lit) if !lit.is_empty() => {
                literals.push(String::leak(to_upper_snake_case(lit)));
            }
            ("#", token) if !token.is_empty() => {
                tokens.push(String::leak(to_upper_snake_case(token)));
            }
            _ if contextual_keywords.contains(&name) => {}
            _ if name.chars().all(char::is_alphabetic) => {
                keywords.push(String::leak(name.to_owned()));
            }
            _ => {
                let idx = PUNCT
                    .iter()
                    .position(|(punct, _)| punct == &name)
                    .unwrap_or_else(|| panic!("Grammar references unknown punctuation {name:?}"));
                used_puncts[idx] = true;
            }
        }
    });
    PUNCT.iter().zip(used_puncts).filter(|(_, used)| !used).for_each(|((punct, _), _)| {
        panic!("Punctuation {punct:?} is not used in grammar");
    });

    keywords.extend(RESERVED.iter().copied());
    keywords.sort();
    keywords.dedup();
    contextual_keywords.sort();
    contextual_keywords.dedup();
    keywords.retain(|&it| !contextual_keywords.contains(&it));

    // we leak things here for simplicity, that way we don't have to deal with lifetimes
    // The execution is a one shot job so thats fine
    let nodes = nodes
        .iter()
        .map(|it| &it.name)
        .map(|it| to_upper_snake_case(it))
        .map(String::leak)
        .map(|it| &*it)
        .collect();
    let nodes = Vec::leak(nodes);
    nodes.sort();
    let enums = enums
        .iter()
        .map(|it| &it.name)
        .map(|it| to_upper_snake_case(it))
        .map(String::leak)
        .map(|it| &*it)
        .collect();
    let enums = Vec::leak(enums);
    enums.sort();
    let keywords = Vec::leak(keywords);
    let contextual_keywords = Vec::leak(contextual_keywords);
    let literals = Vec::leak(literals);
    literals.sort();
    let tokens = Vec::leak(tokens);
    tokens.sort();

    KindsSrc { punct: PUNCT, nodes, _enums: enums, keywords, contextual_keywords, literals, tokens }
}

#[derive(Default, Debug)]
pub(crate) struct AstSrc {
    pub(crate) tokens: Vec<String>,
    pub(crate) nodes: Vec<AstNodeSrc>,
    pub(crate) enums: Vec<AstEnumSrc>,
}

#[derive(Debug)]
pub(crate) struct AstNodeSrc {
    pub(crate) name: String,
    pub(crate) fields: Vec<Field>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Field {
    Token(String),
    Node { name: String, ty: String, cardinality: Cardinality },
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Cardinality {
    Optional,
    Many,
}

#[derive(Debug)]
pub(crate) struct AstEnumSrc {
    pub(crate) name: String,
    pub(crate) variants: Vec<String>,
}
