use regex::Regex;

use crate::utils;

pub(crate) struct DateFormatter {

    _date_decomposition_regex_one: Regex,
    _date_decomposition_regex_two: Regex
}

impl DateFormatter {
    
    pub(crate) fn new() -> Result<Self, utils::Error> {
        Ok(
            Self { 
                _date_decomposition_regex_one: Regex::new(
                    r"^((?P<year>\d{4})[-/\.\s]((?P<month_n>\d{1,2})|(?P<month_w>\w{2,12}))[-/\.\s](?P<day>\d{1,2}))")?,
                _date_decomposition_regex_two: Regex::new(
                    r"(^(?P<day>\d{1,2})[-/\.\s]((?P<month_n>\d{1,2})|(?P<month_w>\w{2,12}))[-/\.\s](?P<year>\d{4}))$")?
            }
        )
    }

    pub(crate) fn standardize(&self, original: &str)  -> String {

        //println!("{}", original);

        let mut cap = self._date_decomposition_regex_one.captures(original);

        if let None = cap {
            cap = self._date_decomposition_regex_two.captures(original);
        }

        let mut year= "";
        let mut month = String::new();
        let mut day = "";

        match cap {

            Some(groups) => {
                
                //println!("{:?}-{:?}-{:?}-{:?}", groups.name("year"), groups.name("month_w"), groups.name("month_n"), groups.name("day"));

                if let Some(y) = groups.name("year") {
                    year = &original[y.start()..y.end()];
                }

                if let Some(mw) = groups.name("month_w") {
                    self.to_month_num(&original[mw.start()..mw.end()], &mut month);
                }

                if let Some(mn) = groups.name("month_n") {
                    month = String::from(&original[mn.start()..mn.end()]);
                }

                if let Some(d) = groups.name("day") {
                    day = &original[d.start()..d.end()];
                }

                format!("{}-{}-{}", year, month, day)
            }
            None => { String::from("")}
        }
    }

    fn to_month_num(&self, from :&str, to: &mut String) {
        

        match from {

            "January"   => {to.push_str("01");}
            "February"  => {to.push_str("02");}
            "March"     => {to.push_str("03");}
            "April"     => {to.push_str("04");}
            "May"       => {to.push_str("05");}
            "June"      => {to.push_str("06");}
            "July"      => {to.push_str("07");}
            "August"    => {to.push_str("08");}
            "September" => {to.push_str("09");}
            "October"   => {to.push_str("10");}
            "November"  => {to.push_str("11");}
            "December"  => {to.push_str("12");}
            _           => {}
        };
    }
}