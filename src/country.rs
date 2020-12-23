use std::fmt;
use std::str::FromStr;


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct InvalidCountryCode;

impl std::error::Error for InvalidCountryCode { }

impl fmt::Display for InvalidCountryCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InvalidCountryCode")
    }
}


// Country Code List: ISO 3166-1993 (E)
// 
// Download from: https://github.com/lukes/ISO-3166-Countries-with-Regional-Codes
pub const COUNTRY_CODES_LEN: usize = 252;
pub static COUNTRY_CODES: [(&'static str, &'static str); COUNTRY_CODES_LEN] = [
    ("AD", "Andorra"), 
    ("AE", "United Arab Emirates"), 
    ("AF", "Afghanistan"), 
    ("AG", "Antigua and Barbuda"), 
    ("AI", "Anguilla"), 
    ("AL", "Albania"), 
    ("AM", "Armenia"), 
    ("AO", "Angola"), 
    ("AQ", "Antarctica"), 
    ("AR", "Argentina"), 
    ("AS", "American Samoa"), 
    ("AT", "Austria"), 
    ("AU", "Australia"), 
    ("AW", "Aruba"), 
    ("AX", "Åland Islands"), 
    ("AZ", "Azerbaijan"), 
    ("BA", "Bosnia and Herzegovina"), 
    ("BB", "Barbados"), 
    ("BD", "Bangladesh"), 
    ("BE", "Belgium"), 
    ("BF", "Burkina Faso"), 
    ("BG", "Bulgaria"), 
    ("BH", "Bahrain"), 
    ("BI", "Burundi"), 
    ("BJ", "Benin"), 
    ("BL", "Saint Barthélemy"), 
    ("BM", "Bermuda"), 
    ("BN", "Brunei Darussalam"), 
    ("BO", "Bolivia (Plurinational State of)"), 
    ("BQ", "Bonaire, Sint Eustatius and Saba"), 
    ("BR", "Brazil"), 
    ("BS", "Bahamas"), 
    ("BT", "Bhutan"), 
    ("BV", "Bouvet Island"), 
    ("BW", "Botswana"), 
    ("BY", "Belarus"), 
    ("BZ", "Belize"), 
    ("CA", "Canada"), 
    ("CC", "Cocos (Keeling) Islands"), 
    ("CD", "Congo, Democratic Republic of the"), 
    ("CF", "Central African Republic"), 
    ("CG", "Congo"), 
    ("CH", "Switzerland"), 
    ("CI", "Côte d'Ivoire"), 
    ("CK", "Cook Islands"), 
    ("CL", "Chile"), 
    ("CM", "Cameroon"), 
    ("CN", "China"), 
    ("CO", "Colombia"), 
    ("CR", "Costa Rica"), 
    ("CU", "Cuba"), 
    ("CV", "Cabo Verde"), 
    ("CW", "Curaçao"), 
    ("CX", "Christmas Island"), 
    ("CY", "Cyprus"), 
    ("CZ", "Czechia"), 
    ("DE", "Germany"), 
    ("DJ", "Djibouti"), 
    ("DK", "Denmark"), 
    ("DM", "Dominica"), 
    ("DO", "Dominican Republic"), 
    ("DZ", "Algeria"), 
    ("EC", "Ecuador"), 
    ("EE", "Estonia"), 
    ("EG", "Egypt"), 
    ("EH", "Western Sahara"), 
    ("ER", "Eritrea"), 
    ("ES", "Spain"), 
    ("ET", "Ethiopia"), 
    ("FI", "Finland"), 
    ("FJ", "Fiji"), 
    ("FK", "Falkland Islands (Malvinas)"), 
    ("FM", "Micronesia (Federated States of)"), 
    ("FO", "Faroe Islands"), 
    ("FR", "France"), 
    ("GA", "Gabon"), 
    ("GB", "United Kingdom of Great Britain and Northern Ireland"), 
    ("GD", "Grenada"), 
    ("GE", "Georgia"), 
    ("GF", "French Guiana"), 
    ("GG", "Guernsey"), 
    ("GH", "Ghana"), 
    ("GI", "Gibraltar"), 
    ("GL", "Greenland"), 
    ("GM", "Gambia"), 
    ("GN", "Guinea"), 
    ("GP", "Guadeloupe"), 
    ("GQ", "Equatorial Guinea"), 
    ("GR", "Greece"), 
    ("GS", "South Georgia and the South Sandwich Islands"), 
    ("GT", "Guatemala"), 
    ("GU", "Guam"), 
    ("GW", "Guinea-Bissau"), 
    ("GY", "Guyana"), 
    ("HK", "Hong Kong"), 
    ("HM", "Heard Island and McDonald Islands"), 
    ("HN", "Honduras"), 
    ("HR", "Croatia"), 
    ("HT", "Haiti"), 
    ("HU", "Hungary"), 
    ("ID", "Indonesia"), 
    ("IE", "Ireland"), 
    ("IL", "Israel"), 
    ("IM", "Isle of Man"), 
    ("IN", "India"), 
    ("IO", "British Indian Ocean Territory"), 
    ("IQ", "Iraq"), 
    ("IR", "Iran (Islamic Republic of)"), 
    ("IS", "Iceland"), 
    ("IT", "Italy"), 
    ("JE", "Jersey"), 
    ("JM", "Jamaica"), 
    ("JO", "Jordan"), 
    ("JP", "Japan"), 
    ("KE", "Kenya"), 
    ("KG", "Kyrgyzstan"), 
    ("KH", "Cambodia"), 
    ("KI", "Kiribati"), 
    ("KM", "Comoros"), 
    ("KN", "Saint Kitts and Nevis"), 
    ("KP", "Korea (Democratic People's Republic of)"), 
    ("KR", "Korea, Republic of"), 
    ("KW", "Kuwait"), 
    ("KY", "Cayman Islands"), 
    ("KZ", "Kazakhstan"), 
    ("LA", "Lao People's Democratic Republic"), 
    ("LB", "Lebanon"), 
    ("LC", "Saint Lucia"), 
    ("LI", "Liechtenstein"), 
    ("LK", "Sri Lanka"), 
    ("LR", "Liberia"), 
    ("LS", "Lesotho"), 
    ("LT", "Lithuania"), 
    ("LU", "Luxembourg"), 
    ("LV", "Latvia"), 
    ("LY", "Libya"), 
    ("MA", "Morocco"), 
    ("MC", "Monaco"), 
    ("MD", "Moldova, Republic of"), 
    ("ME", "Montenegro"), 
    ("MF", "Saint Martin (French part)"), 
    ("MG", "Madagascar"), 
    ("MH", "Marshall Islands"), 
    ("MK", "North Macedonia"), 
    ("ML", "Mali"), 
    ("MM", "Myanmar"), 
    ("MN", "Mongolia"), 
    ("MO", "Macao"), 
    ("MP", "Northern Mariana Islands"), 
    ("MQ", "Martinique"), 
    ("MR", "Mauritania"), 
    ("MS", "Montserrat"), 
    ("MT", "Malta"), 
    ("MU", "Mauritius"), 
    ("MV", "Maldives"), 
    ("MW", "Malawi"), 
    ("MX", "Mexico"), 
    ("MY", "Malaysia"), 
    ("MZ", "Mozambique"), 
    ("NA", "Namibia"), 
    ("NC", "New Caledonia"), 
    ("NE", "Niger"), 
    ("NF", "Norfolk Island"), 
    ("NG", "Nigeria"), 
    ("NI", "Nicaragua"), 
    ("NL", "Netherlands"), 
    ("NO", "Norway"), 
    ("NP", "Nepal"), 
    ("NR", "Nauru"), 
    ("NU", "Niue"), 
    ("NZ", "New Zealand"), 
    ("OM", "Oman"), 
    ("PA", "Panama"), 
    ("PE", "Peru"), 
    ("PF", "French Polynesia"), 
    ("PG", "Papua New Guinea"), 
    ("PH", "Philippines"), 
    ("PK", "Pakistan"), 
    ("PL", "Poland"), 
    ("PM", "Saint Pierre and Miquelon"), 
    ("PN", "Pitcairn"), 
    ("PR", "Puerto Rico"), 
    ("PS", "Palestine, State of"), 
    ("PT", "Portugal"), 
    ("PW", "Palau"), 
    ("PY", "Paraguay"), 
    ("QA", "Qatar"), 
    ("RE", "Réunion"), 
    ("RO", "Romania"), 
    ("RS", "Serbia"), 
    ("RU", "Russian Federation"), 
    ("RW", "Rwanda"), 
    ("SA", "Saudi Arabia"), 
    ("SB", "Solomon Islands"), 
    ("SC", "Seychelles"), 
    ("SD", "Sudan"), 
    ("SE", "Sweden"), 
    ("SG", "Singapore"), 
    ("SH", "Saint Helena, Ascension and Tristan da Cunha"), 
    ("SI", "Slovenia"), 
    ("SJ", "Svalbard and Jan Mayen"), 
    ("SK", "Slovakia"), 
    ("SL", "Sierra Leone"), 
    ("SM", "San Marino"), 
    ("SN", "Senegal"), 
    ("SO", "Somalia"), 
    ("SR", "Suriname"), 
    ("SS", "South Sudan"), 
    ("ST", "Sao Tome and Principe"), 
    ("SV", "El Salvador"), 
    ("SX", "Sint Maarten (Dutch part)"), 
    ("SY", "Syrian Arab Republic"), 
    ("SZ", "Eswatini"), 
    ("TC", "Turks and Caicos Islands"), 
    ("TD", "Chad"), 
    ("TF", "French Southern Territories"), 
    ("TG", "Togo"), 
    ("TH", "Thailand"), 
    ("TJ", "Tajikistan"), 
    ("TK", "Tokelau"), 
    ("TL", "Timor-Leste"), 
    ("TM", "Turkmenistan"), 
    ("TN", "Tunisia"), 
    ("TO", "Tonga"), 
    ("TR", "Turkey"), 
    ("TT", "Trinidad and Tobago"), 
    ("TV", "Tuvalu"), 
    ("TW", "Taiwan, Province of China"), 
    ("TZ", "Tanzania, United Republic of"), 
    ("UA", "Ukraine"), 
    ("UG", "Uganda"), 
    ("UM", "United States Minor Outlying Islands"), 
    ("US", "United States of America"), 
    ("UY", "Uruguay"), 
    ("UZ", "Uzbekistan"), 
    ("VA", "Holy See"), 
    ("VC", "Saint Vincent and the Grenadines"), 
    ("VE", "Venezuela (Bolivarian Republic of)"), 
    ("VG", "Virgin Islands (British)"), 
    ("VI", "Virgin Islands (U.S.)"), 
    ("VN", "Viet Nam"), 
    ("VU", "Vanuatu"), 
    ("WF", "Wallis and Futuna"), 
    ("WS", "Samoa"), 
    ("YE", "Yemen"), 
    ("YT", "Mayotte"), 
    ("ZA", "South Africa"), 
    ("ZM", "Zambia"), 
    ("ZW", "Zimbabwe"), 

    // NOTE: 以下 CountryCode 为 IANA 自定义代码，并不在 ISO 编码列表里面。
    ("EU", "EU"),
    ("AP", "AP"),
    ("ZZ", "ZZ"),
    // EU = 249u8,
    // AP = 250u8,
    // ZZ = 251u8,
];

#[repr(transparent)]
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd)]
pub struct Country(u8);

impl Country {
    pub const CN: Self = Self(47);
    pub const HK: Self = Self(94);
    pub const TW: Self = Self(227);
    pub const JP: Self = Self(113);
    pub const KP: Self = Self(120);
    pub const KR: Self = Self(121);
    pub const RU: Self = Self(190);

    pub const SG: Self = Self(197);
    pub const VN: Self = Self(240);

    pub const US: Self = Self(232);
    pub const GB: Self = Self(76);
    pub const FR: Self = Self(74);
    pub const DE: Self = Self(56);
    
    pub const EU: Self = Self(249);
    pub const AP: Self = Self(250);
    pub const ZZ: Self = Self(251);

    
    #[inline]
    pub fn from_index(idx: u8) -> Self {
        assert!((idx as usize) < COUNTRY_CODES_LEN);
        Country(idx)
    }
    
    #[inline]
    pub const unsafe fn from_index_unchecked(idx: u8) -> Self {
        Country(idx)
    }

    #[inline]
    pub fn index(&self) -> u8 {
        self.0
    }

    #[inline]
    pub fn code(&self) -> &'static str {
        COUNTRY_CODES[self.0 as usize].0
    }

    #[inline]
    pub fn full_name(&self) -> &'static str {
        COUNTRY_CODES[self.0 as usize].1
    }
}

impl Into<u8> for Country {
    fn into(self) -> u8 {
        self.0
    }
}

impl FromStr for Country {
    type Err = InvalidCountryCode;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        COUNTRY_CODES
            .binary_search_by(|&(code, _)| code.cmp(s))
            .map_err(|_| InvalidCountryCode)
            .map(|idx| Self(idx as u8))
    }
}

impl fmt::Debug for Country {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{:?}", self.code())
    }
}


#[test]
fn test_country_consts() {
    assert_eq!("CN".parse::<Country>(), Ok(Country::CN));
    assert_eq!("HK".parse::<Country>(), Ok(Country::HK));
    assert_eq!("TW".parse::<Country>(), Ok(Country::TW));
    assert_eq!("JP".parse::<Country>(), Ok(Country::JP));
    assert_eq!("KP".parse::<Country>(), Ok(Country::KP));
    assert_eq!("KR".parse::<Country>(), Ok(Country::KR));
    assert_eq!("RU".parse::<Country>(), Ok(Country::RU));
    
    assert_eq!("SG".parse::<Country>(), Ok(Country::SG));
    assert_eq!("VN".parse::<Country>(), Ok(Country::VN));

    assert_eq!("US".parse::<Country>(), Ok(Country::US));
    assert_eq!("GB".parse::<Country>(), Ok(Country::GB));
    assert_eq!("FR".parse::<Country>(), Ok(Country::FR));
    assert_eq!("DE".parse::<Country>(), Ok(Country::DE));
}