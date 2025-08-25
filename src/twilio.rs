use serde::Deserialize;
use serde_inline_default::serde_inline_default;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct PhoneNumberResponse {
    pub call_forwarding: Option<String>,
    pub caller_name: Option<String>,
    pub calling_country_code: Option<String>,
    pub country_code: Option<String>,
    pub identity_match: Option<IdentityMatch>,
    pub line_status: Option<String>,
    pub line_type_intelligence: Option<String>,
    pub national_format: Option<String>,
    pub phone_number: Option<String>,
    pub phone_number_quality_score: Option<String>,
    pub pre_fill: Option<String>,
    pub reassigned_number: Option<String>,
    pub sim_swap: Option<SimSwap>,
    pub sms_pumping_risk: Option<String>,
    pub url: Option<String>,
    pub valid: Option<bool>,
}

#[serde_inline_default]
#[derive(Deserialize, Debug)]
pub struct IdentityMatch {
    pub first_name_match: Option<String>,
    pub last_name_match: Option<String>,
    pub address_lines_match: Option<String>,
    pub city_match: Option<String>,
    pub state_match: Option<String>,
    pub postal_code_match: Option<String>,
    pub address_country_match: Option<String>,
    pub national_id_match: Option<String>,
    pub date_of_birth_match: Option<String>,
    pub summary_score: Option<u32>,
    #[serde_inline_default(0)]
    pub error_code: u32,
    pub error_message: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SimSwap {
    pub carrier_name: Option<String>,
    pub error_code: Option<i32>,
    pub last_sim_swap: Option<LastSimSwapData>,
    pub mobile_country_code: Option<String>,
    pub mobile_network_code: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct LastSimSwapData {
    pub last_sim_swap_date: Option<String>,
    pub swapped_period: Option<String>,
    pub swapped_in_period: Option<String>,
}
