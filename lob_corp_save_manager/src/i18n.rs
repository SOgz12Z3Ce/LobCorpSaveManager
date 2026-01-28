pub(crate) struct I18nText {
    en: &'static str,
    ko: &'static str,
    zh_hans: &'static str,
    zh_hant: &'static str,
    ja: &'static str,
    ru: &'static str,
    vi: &'static str,
    bg: &'static str,
    es_419: &'static str,
    fr: &'static str,
    pt_br: &'static str,
    pt_pt: &'static str,
}

macro_rules! same_I18nText {
    ($text:literal) => {
        &I18nText {
            en: $text,
            ko: $text,
            zh_hans: $text,
            zh_hant: $text,
            ja: $text,
            ru: $text,
            vi: $text,
            bg: $text,
            es_419: $text,
            fr: $text,
            pt_br: $text,
            pt_pt: $text,
        }
    };
}

impl I18nText {
    pub(crate) const fn lookup_code(id: u32) -> &'static Self {
        match id {
            100000 => &I18nText {
                en: "0-00-00",
                ko: "0-00-00",
                zh_hans: "O-00-00",
                zh_hant: "0-00-00",
                ja: "0-00-00",
                ru: "0-00-00",
                vi: "0-00-00",
                bg: "0-00-00",
                es_419: "0-00-00",
                fr: "0-00-00",
                pt_br: "0-00-00",
                pt_pt: "0-00-00",
            },
            100001 => same_I18nText!("F-01-02"),
            100002 => same_I18nText!("T-04-06"),
            100003 => same_I18nText!("O-04-08"),
            100004 => &I18nText {
                en: "O-01-04",
                ko: "O-01-04",
                zh_hans: "O-01-04",
                zh_hant: "0-01-04",
                ja: "O-01-04",
                ru: "O-01-04",
                vi: "O-01-04",
                bg: "O-01-04",
                es_419: "O-01-04",
                fr: "O-01-04",
                pt_br: "O-01-04",
                pt_pt: "O-01-04",
            },
            100005 => same_I18nText!("O-06-20"),
            100006 => same_I18nText!("O-05-30"),
            100007 => same_I18nText!("T-06-27"),
            100008 => same_I18nText!("O-02-40"),
            100009 => same_I18nText!("O-03-03"),
            100010 => same_I18nText!("0-01-04-W"),
            100011 => same_I18nText!("T-05-41"),
            100012 => same_I18nText!("T-02-43"),
            100013 => same_I18nText!("F-02-44"),
            100014 => same_I18nText!("O-01-45"),
            100015 => same_I18nText!("T-03-46"),
            100016 => same_I18nText!("F-02-49"),
            100017 => same_I18nText!("O-01-15"),
            100018 => same_I18nText!("T-01-54"),
            100019 => &I18nText {
                en: "T-01-31",
                ko: "T-01-31",
                zh_hans: "T-01-31",
                zh_hant: "T-01-55",
                ja: "T-01-31",
                ru: "T-01-31",
                vi: "T-01-31",
                bg: "T-01-31",
                es_419: "T-01-31",
                fr: "T-01-31",
                pt_br: "T-01-31",
                pt_pt: "T-01-31",
            },
            100020 => same_I18nText!("O-02-56"),
            100021 => same_I18nText!("O-01-12"),
            100022 => same_I18nText!("F-01-18"),
            100023 => same_I18nText!("F-04-42"),
            100024 => same_I18nText!("O-05-47"),
            100025 => same_I18nText!("O-05-48-Z"),
            100026 => same_I18nText!("T-04-50"),
            100027 => same_I18nText!("T-05-51"),
            100028 => same_I18nText!("F-05-52"),
            100029 => same_I18nText!("T-04-53"),
            100030 => same_I18nText!("O-05-31-W"),
            100031 => same_I18nText!("O-01-55"),
            100032 => same_I18nText!("F-01-57"),
            100033 => same_I18nText!("F-02-58"),
            100034 => &I18nText {
                en: "Bald-is-awesome!",
                ko: "Bald-is-awesome!",
                zh_hans: "秃头-真是-太棒啦！",
                zh_hant: "光頭超棒!",
                ja: "ものすごいハゲ！",
                ru: "Лысина-это-круто!",
                vi: "Bald-is-awesome!",
                bg: "Bald-is-awesome!",
                es_419: "Bald-is-awesome!",
                fr: "Chauve-est-génial!",
                pt_br: "Ser-Careca-é-Íncrível!",
                pt_pt: "Ser-Careca-É-Incrível!",
            },
            100035 => same_I18nText!("O-02-62"),
            100036 => same_I18nText!("O-03-60"),
            100037 => same_I18nText!("O-05-61"),
            100039 => same_I18nText!("O-01-64"),
            100040 => same_I18nText!("O-04-66"),
            100041 => same_I18nText!("O-01-67"),
            100042 => same_I18nText!("T-01-75"),
            100043 => same_I18nText!("T-01-68"),
            100044 => same_I18nText!("O-04-72"),
            100045 => same_I18nText!("F-02-70"),
            100046 => same_I18nText!("O-02-74"),
            100047 => same_I18nText!("T-02-71"),
            100048 => same_I18nText!("O-01-73"),
            100049 => same_I18nText!("O-05-76"),
            100050 => same_I18nText!("F-01-87"),
            100051 => same_I18nText!("F-05-32"),
            100052 => same_I18nText!("O-01-92"),
            100053 => same_I18nText!("F-04-83"),
            100054 => same_I18nText!("O-04-84"),
            100055 => same_I18nText!("O-03-88"),
            100056 => same_I18nText!("O-03-89"),
            100057 => &I18nText {
                en: "O-02-98",
                ko: "O-02-98",
                zh_hans: "O-02-98",
                zh_hant: "T-04-53",
                ja: "O-02-98",
                ru: "O-02-98",
                vi: "O-02-98",
                bg: "O-02-98",
                es_419: "O-02-98",
                fr: "O-02-98",
                pt_br: "O-02-98",
                pt_pt: "O-02-98",
            },
            100058 => same_I18nText!("O-03-93"),
            100059 => same_I18nText!("O-04-100"),
            100060 => same_I18nText!("T-02-99"),
            100061 => same_I18nText!("O-02-101"),
            100062 => same_I18nText!("D-04-108"),
            100063 => same_I18nText!("D-03-109"),
            100064 => same_I18nText!("D-01-106"),
            100065 => same_I18nText!("D-01-105"),
            100101 => same_I18nText!("O-05-65-H"),
            100102 => same_I18nText!("F-01-37"),
            100103 => same_I18nText!("F-01-69"),
            100104 => same_I18nText!("O-05-102"),
            100105 => same_I18nText!("D-01-110"),
            100106 => same_I18nText!("D-02-107"),
            200001 => same_I18nText!("X"),
            200002 => same_I18nText!("X"),
            200003 => same_I18nText!("X"),
            200004 => same_I18nText!("X"),
            200005 => same_I18nText!("X"),
            200006 => same_I18nText!("X"),
            200007 => same_I18nText!("X"),
            200009 => same_I18nText!("X"),
            200010 => same_I18nText!("X"),
            200013 => same_I18nText!("X"),
            200015 => same_I18nText!("X"),
            200016 => same_I18nText!("X"),
            200018 => same_I18nText!("X"),
            200021 => same_I18nText!("X"),
            200022 => same_I18nText!("X"),
            200023 => same_I18nText!("X"),
            200024 => same_I18nText!("X"),
            200025 => same_I18nText!("X"),
            300001 => same_I18nText!("T-09-77"),
            300002 => same_I18nText!("T-09-78"),
            300003 => same_I18nText!("T-09-09"),
            300004 => same_I18nText!("T-09-79"),
            300005 => same_I18nText!("T-09-80"),
            300006 => same_I18nText!("T-09-82"),
            300007 => same_I18nText!("O-09-81"),
            300101 => same_I18nText!("T-09-85"),
            300102 => same_I18nText!("T-09-86"),
            300103 => same_I18nText!("T-09-90"),
            300104 => same_I18nText!("O-09-91"),
            300105 => same_I18nText!("T-09-94"),
            300106 => same_I18nText!("O-09-95"),
            300107 => same_I18nText!("O-09-96"),
            300108 => same_I18nText!("T-09-97"),
            300109 => same_I18nText!("O-07-103"),
            300110 => same_I18nText!("D-09-104"),
            1000350 => same_I18nText!("???"),
            1000351 => same_I18nText!("???"),
            1000352 => same_I18nText!("???"),
            1000353 => same_I18nText!("???"),
            2000121 => same_I18nText!("???"),
            2000122 => same_I18nText!("???"),
            2000123 => same_I18nText!("???"),
            2000124 => same_I18nText!("???"),
            _ => unreachable!(),
        }
    }
    pub(crate) fn en(&self) -> &'static str {
        self.en
    }
    pub(crate) fn ko(&self) -> &'static str {
        self.ko
    }
    pub(crate) fn zh_hans(&self) -> &'static str {
        self.zh_hans
    }
    pub(crate) fn zh_hant(&self) -> &'static str {
        self.zh_hant
    }
    pub(crate) fn ja(&self) -> &'static str {
        self.ja
    }
    pub(crate) fn ru(&self) -> &'static str {
        self.ru
    }
    pub(crate) fn vi(&self) -> &'static str {
        self.vi
    }
    pub(crate) fn bg(&self) -> &'static str {
        self.bg
    }
    pub(crate) fn es_419(&self) -> &'static str {
        self.es_419
    }
    pub(crate) fn fr(&self) -> &'static str {
        self.fr
    }
    pub(crate) fn pt_br(&self) -> &'static str {
        self.pt_br
    }
    pub(crate) fn pt_pt(&self) -> &'static str {
        self.pt_pt
    }
}
