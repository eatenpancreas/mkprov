use crate::{Location};
use crate::lexer::{Token, Lexer};
use crate::parser::Parser;

const SIMPLE: &str = r#"
width = 6400
height = 2560
"#;

#[test]
fn simple_lex() {
    let idents: Vec<Token> = Lexer::new(SIMPLE).collect();
    assert_eq!(idents, vec![
        Token { location: Location(1), content: "width".to_string() },
        Token { location: Location(7), content: "=".to_string() },
        Token { location: Location(9), content: "6400".to_string() },
        Token { location: Location(14), content: "height".to_string() },
        Token { location: Location(21), content: "=".to_string() },
        Token { location: Location(23), content: "2560".to_string() }
    ])
}
#[test]
fn simple_parse() {
    let parser = Parser::new(Lexer::new(SIMPLE).collect());
    println!("{:?}", parser.parse());
}

const COMMENTS_ETC: &str = r#"
#Hello world!
width = 6400
stars = "In the #sky"
"#;

#[test]
fn comments_etc_lex() {
    let idents: Vec<Token> = Lexer::new(COMMENTS_ETC).collect();
    assert_eq!(idents, vec![
        Token { location: Location(15), content: "width".to_string() },
        Token { location: Location(21), content: "=".to_string() },
        Token { location: Location(23), content: "6400".to_string() },
        Token { location: Location(28), content: "stars".to_string() },
        Token { location: Location(34), content: "=".to_string() },
        Token { location: Location(37), content: "In the #sky".to_string() }
    ])
}

const OBJECTS: &str = r#"
2024.06.24 = {
    event = "making of the mod"
    height = 2560
}
"#;
#[test]
fn objects_lex() {
    let idents: Vec<Token> = Lexer::new(OBJECTS).collect();
    println!("{idents:?}");
}

const FULL_FILE_ADAL: &str = r#"
government = monarchy
add_government_reform = autocracy_reform
technology_group = east_african
unit_type = sub_saharan
religion = sunni
primary_culture = somali
add_accepted_culture = harari
capital = 1211	# Deker, Harer
religious_school = shafii_school
add_historical_rival = ETH

1415.1.1 = {
	monarch = {
		name = "Sabr"
		dynasty = "Walashma"
		dip = 0
		mil = 0
		adm = 1
	}
}

1422.1.1 = {
	monarch = {
		name = "Mansur"
		dynasty = "Walashma"
		adm = 2
		dip = 1
		mil = 0
	}
}

1424.1.1 = {
	monarch = {
		name = "Jamal"
		dynasty = "Walashma"
		adm = 2
		dip = 0
		mil = 2
	}
}

1433.1.1 = {
	monarch = {
		name = "Badlay"
		dynasty = "Walashma"
		adm = 2
		dip = 2
		mil = 1
	}
}

1445.1.1 = {
	monarch = {
		name = "Mohammed"
		dynasty = "Walashma"
		adm = 1
		dip = 1
		mil = 1
	}
}

1472.1.1 = {
	monarch = {
		name = "Shams"
		dynasty = "Walashma"
		adm = 0
		dip = 0
		mil = 1
	}
}

1488.1.1 = {
	monarch = {
		name = "Mohamed"
		dynasty = "Walashma"
		adm = 0
		dip = 0
		mil = 1
	}
}

1518.1.1 = {
	monarch = {
		name = "Mahfuz"
		dynasty = "Karanle"
		adm = 0
		dip = 0
		mil = 0
	}
}

1519.1.1 = {
	monarch = {
		name = "'Abu"
		dynasty = "Walashma"
		adm = 0
		dip = 0
		mil = 0
	}
}

1527.1.1 = {
	monarch = {
		name = "Ahmad Gran"
		dynasty = "Karanle"
		adm = 1
		dip = 2
		mil = 5
	}
}

1543.2.21 = {
	monarch = {
		name = "Bat'iah"
		dynasty = "Karanle"
		adm = 1
		dip = 2
		mil = 1
		female = yes
	}
}

1552.1.1 = {
	monarch = {
		name = "Nur"
		dynasty = "Karanle"
		adm = 0
		dip = 1
		mil = 3
	}
}
"#;

#[test]
fn full_file_adal_lex() {
    for ident in Lexer::new(FULL_FILE_ADAL) {
        println!("{ident:?}");
    }
}