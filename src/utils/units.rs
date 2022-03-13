#[derive(Debug, PartialEq)]
/// Unit to define length unit.
/// Only supports Metric and Imperial
pub enum WeightType {
    Metric,
    Imperial,
}

#[derive(Debug, PartialEq)]
pub enum TemperatureType {
    Celsius,
    Fahrenheit,
    Kelvin,
}

#[derive(Debug)]
pub enum CurrencyTypes {
    AED,
    AFN,
    ALL,
    AMD,
    ANG,
    AOA,
    ARS,
    AUD,
    AWG,
    AZN,
    BAM,
    BBD,
    BDT,
    BGN,
    BHD,
    BIF,
    BMD,
    BND,
    BOB,
    BRL,
    BSD,
    BTC,
    BTN,
    BWP,
    BYN,
    BYR,
    BZD,
    CAD,
    CDF,
    CHF,
    CLF,
    CLP,
    CNY,
    COP,
    CRC,
    CUC,
    CUP,
    CVE,
    CZK,
    DJF,
    DKK,
    DOP,
    DZD,
    EGP,
    ETB,
    EUR,
    FJD,
    GBP,
    GEL,
    GHS,
    GMD,
    GNF,
    GTQ,
    GYD,
    HKD,
    HNL,
    HRK,
    HTG,
    HUF,
    IDR,
    ILS,
    INR,
    IQD,
    IRR,
    ISK,
    JMD,
    JOD,
    JPY,
    KES,
    KGS,
    KHR,
    KMF,
    KRW,
    KWD,
    KYD,
    KZT,
    LAK,
    LBP,
    LKR,
    LRD,
    LSL,
    LTL,
    LVL,
    LYD,
    MAD,
    MDL,
    MGA,
    MKD,
    MMK,
    MOP,
    MRO,
    MUR,
    MVR,
    MWK,
    MXN,
    MYR,
    MZN,
    NAD,
    NGN,
    NIO,
    NOK,
    NPR,
    NZD,
    OMR,
    PAB,
    PEN,
    PGK,
    PHP,
    PKR,
    PLN,
    PYG,
    QAR,
    RON,
    RSD,
    RUB,
    RWF,
    SAR,
    SBD,
    SCR,
    SDG,
    SEK,
    SGD,
    SHP,
    SLL,
    SOS,
    SRD,
    STD,
    SVC,
    SZL,
    THB,
    TJS,
    TMT,
    TND,
    TOP,
    TRY,
    TTD,
    TWD,
    TZS,
    UAH,
    UGX,
    USD,
    UYU,
    UZS,
    VND,
    XAF,
    XCD,
    XDR,
    XOF,
    XPF,
    YER,
    ZAR,
    ZMK,
    ZMW,
    ZWL,
}

#[cfg(not(tarpaulin_include))]
impl std::fmt::Display for WeightType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WeightType::Metric => {
                write!(f, "Metric")
            }
            WeightType::Imperial => {
                write!(f, "Imperial")
            }
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl std::fmt::Display for TemperatureType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TemperatureType::Celsius => {
                write!(f, "Celsius")
            }
            TemperatureType::Fahrenheit => {
                write!(f, "Fahrenheit")
            }
            TemperatureType::Kelvin => {
                write!(f, "Kelvin")
            }
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl std::fmt::Display for CurrencyTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            _ => {
                write!(f, "{:?}", self)
            }
        }
    }
}
