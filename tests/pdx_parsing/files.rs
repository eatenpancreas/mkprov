

pub const SIMPLE: &str = r#"
width = 6400
height = 2560
"#;

pub const PROBLEMATIC_LEX: &str = r#"
bahama_channel_area = {
	1503 1505 1524 1525
}

#The problem is the equals sign being picked up in comments

#
#central_asian_lakes_area = {
#	1326 1327 1653 1654 1888
#}
"#;

pub const PROBLEMATIC_LEX_2: &str = r#"
bahama_channel_area = {
	1503 1505 1524 1525
}

#The problem is the brackets being picked up as something else

#
#central_asian_lakes_area = {1326 1327 1653 1654 1888}
#central_asian_lakes_area = {
#	1326 1327 1653 1654 1888}
"#;

pub const COMMENTS_ETC: &str = r#"
#Hello world!
width = 6400
stars = "In the #sky"
"#;

pub const OBJECTS: &str = r#"
2024.06.24 = {
    event = "making of the mod"
    height = 2560
    width = 12.142
}
"#;

pub const OBJECTS_CHANGED: &str = r#"2024.06.24 = {
    event = "making of the GHOSTING"
    height = 2560
    width = 12.142
    status = "tired asf"
}
"#;

pub const INCORRECT_1: &str = r#"2024.06.24 =event = "making of the mod"
    height = 2560
}
"#;

pub const INCORRECT_2: &str = r#"
2024.06.24 {
    event = "making of the mod"
    height = 2560
}
"#;

pub const INCORRECT_3: &str = r#"="#;

pub const FULL_UNC_FILE: &str = r#"
#1000 - Dagobah
culture = vroom
religion = rawr
capital = "The Capital"
trade_goods = unknown
hre = no
base_tax = 1
base_production = 1
base_manpower = 1
native_size = 90
native_ferocity = 4
native_hostileness = 12



discovered_by = KON
discovered_by = NDO
discovered_by = LOA
discovered_by = sub_saharan
"#;

pub const FULL_FILE_ADAL: &str = r#"
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
