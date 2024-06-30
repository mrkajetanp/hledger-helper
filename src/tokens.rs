pub static DATE_FMT: &str = "%Y-%m-%d";

pub static GROCERIES: &str = "Expenses:Groceries";
pub static ALCOHOL: &str = "Expenses:Alcohol";
pub static EATING_OUT: &str = "Expenses:Eating Out";
pub static TRAVEL: &str = "Expenses:Travel";
pub static OTHER: &str = "Expenses:Other";
pub static MEDICAL: &str = "Expenses:Medical";
pub static SHOPPING: &str = "Expenses:Shopping";

pub static STARLING: &str = "Assets:Bank:Starling Bank";
pub static CHASE: &str = "Assets:Bank:Chase";
pub static CHASE_SAVINGS: &str = "Assets:Bank:Chase:Savings";
pub static CHASE_INTEREST: &str = "Revenue:Interest:Chase";

pub static CL_LTC: &str = "Assets:Crypto:CL:LTC";

pub fn description_to_account(description: &str) -> String {
    match description.to_lowercase().as_str() {
        "iceland" |
        "aldi" |
        "ryman" |
        "general store" |
        "hellofresh" |
        "spar" |
        "waitrose" |
        "sainsbury's" |
        "marks & spencer simply food"
        => GROCERIES,

        "amazon" |
        "amazon prime" |
        "ikea" |
        "poundland" |
        "deichmann" |
        "argos"
        => SHOPPING,

        "mctucky's" |
        "subway" |
        "limoncello cambridge" |
        "steaks" |
        "delaware north" |
        "the euston flyer" |
        "gfc" |
        "uber eats" |
        "u jarka" |
        "panchos burritos l" |
        "chapel cafe"
        => EATING_OUT,

        "on bar manchester" |
        "sikorski memorial house" |
        "brewdog man doghouse" |
        "mitre bar" |
        "stramash" |
        "the bon accord" |
        "inn deep"
        => ALCOHOL,

        "uber" |
        "trainpal" |
        "transport for greater manchester" |
        "tfl - transport for london"
        => TRAVEL,

        "miu" |
        "smart city tailors" |
        "post office" |
        "bp" |
        "creditladder"
        => OTHER,

        "feel good contacts" |
        "boots"
        => MEDICAL,

        "from kajetan puchalski - transfer"
        => STARLING,

        "round up" |
        "to savings"
        => CHASE_SAVINGS,

        "Interest earned"
        => CHASE_INTEREST,

        _ => "[UNCLASSIFIED]",
     }.to_string()
}
