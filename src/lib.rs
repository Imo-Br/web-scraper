pub mod site;
use regex::Regex;

use std::fmt::{self, Display, Formatter};

// region:  ---TagNames

const A: &str = "a";
const ABBR: &str = "abbr";
const ACRONYM: &str = "acronym";
const ADDRESS: &str = "address";
const APPLET: &str = "applet";
const AREA: &str = "area";
const ARTICLE: &str = "article";
const ASIDE: &str = "aside";
const AUDIO: &str = "audio";
const B: &str = "b";
const BASE: &str = "base";
const BASEFONT: &str = "basefont";
const BDI: &str = "bdi";
const BDO: &str = "bdo";
const BIG: &str = "big";
const BLOCKQUOTE: &str = "blockquote";
const BODY: &str = "body";
const BR: &str = "br";
const BUTTON: &str = "button";
const CANVAS: &str = "canvas";
const CAPTION: &str = "caption";
const CENTER: &str = "center";
const CITE: &str = "cite";
const CODE: &str = "code";
const COL: &str = "col";
const COLGROUP: &str = "colgroup";
const DATALIST: &str = "datalist";
const DD: &str = "dd";
const DEL: &str = "del";
const DETAILS: &str = "details";
const DFN: &str = "dfn";
const DIR: &str = "dir";
const DIV: &str = "div";
const DL: &str = "dl";
const DT: &str = "dt";
const EM: &str = "em";
const EMBED: &str = "embed";
const FIELDSET: &str = "fieldset";
const FIGCAPTION: &str = "figcaption";
const FIGURE: &str = "figure";
const FONT: &str = "font";
const FOOTER: &str = "footer";
const FORM: &str = "form";
const FRAME: &str = "frame";
const FRAMESET: &str = "frameset";
const H1: &str = "h1";
const H2: &str = "h2";
const H3: &str = "h3";
const H4: &str = "h4";
const H5: &str = "h5";
const H6: &str = "h6";
const HEAD: &str = "head";
const HEADER: &str = "header";
const HR: &str = "hr";
const HTML: &str = "html";
const I: &str = "i";
const IFRAME: &str = "iframe";
const IMG: &str = "img";
const INPUT: &str = "input";
const INS: &str = "ins";
const KBD: &str = "kbd";
const KEYGEN: &str = "keygen";
const LABEL: &str = "label";
const LEGEND: &str = "legend";
const LI: &str = "li";
const LINK: &str = "link";
const MAIN: &str = "main";
const MAP: &str = "map";
const MARK: &str = "mark";
const MENU: &str = "menu";
const MENUITEM: &str = "menuitem";
const META: &str = "meta";
const METER: &str = "meter";
const NAV: &str = "nav";
const NOFRAMES: &str = "noframes";
const NOSCRIPT: &str = "noscript";
const OBJECT: &str = "object";
const OL: &str = "ol";
const OPTGROUP: &str = "optgroup";
const OPTION: &str = "option";
const OUTPUT: &str = "output";
const P: &str = "p";
const PARAM: &str = "param";
const PRE: &str = "pre";
const PROGRESS: &str = "progress";
const Q: &str = "q";

const S: &str = "s";
const SAMP: &str = "samp";
const SCRIPT: &str = "script";
const SECTION: &str = "section";
const SELECT: &str = "select";
const SMALL: &str = "small";
const SOURCE: &str = "source";
const SPAN: &str = "span";
const STRIKE: &str = "strike";
const STRONG: &str = "strong";
const STYLE: &str = "style";
const SUB: &str = "sub";
const SUMMARY: &str = "summary";
const SUP: &str = "sup";
const TABLE: &str = "table";
const TBODY: &str = "tbody";
const TD: &str = "td";
const TEXTAREA: &str = "textarea";
const TFOOT: &str = "tfoot";
const TH: &str = "th";
const THEAD: &str = "thead";
const TIME: &str = "time";
const TITLE: &str = "title";
const TR: &str = "tr";
const TRACK: &str = "track";
const TT: &str = "tt";
const U: &str = "u";
const UL: &str = "ul";
const VAR: &str = "var";
const VIDEO: &str = "video";

// endregion:  ---TagNames

// region:  ---Make html tag struct
///You can use this struct to get the elements inside a tag, by the tag name
/// # Example
/// ```rust
/// let tag = HtmlTag::HTML
/// // Parse the <html> tags and collect the results into a vector of strings
/// let new_vector = tag.parse_tags(&buf);
/// ```
pub struct HtmlTag(pub &'static str);
impl Display for HtmlTag {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl HtmlTag {
    ///   This represents the html tag: a
    pub const A: HtmlTag = HtmlTag(A);
    ///   This represents the html tag:  abbr  
    pub const ABBR: HtmlTag = HtmlTag(ABBR);
    ///   This represents the html tag:  acronym  
    pub const ACRONYM: HtmlTag = HtmlTag(ACRONYM);
    ///   This represents the html tag:  address  
    pub const ADDRESS: HtmlTag = HtmlTag(ADDRESS);
    ///   This represents the html tag:  applet  
    pub const APPLET: HtmlTag = HtmlTag(APPLET);
    ///   This represents the html tag:  area  
    pub const AREA: HtmlTag = HtmlTag(AREA);
    ///   This represents the html tag:  article  
    pub const ARTICLE: HtmlTag = HtmlTag(ARTICLE);
    ///   This represents the html tag:  aside  
    pub const ASIDE: HtmlTag = HtmlTag(ASIDE);
    ///   This represents the html tag:  audio  
    pub const AUDIO: HtmlTag = HtmlTag(AUDIO);
    ///   This represents the html tag:  b  
    pub const B: HtmlTag = HtmlTag(B);
    ///   This represents the html tag:  base  
    pub const BASE: HtmlTag = HtmlTag(BASE);
    ///   This represents the html tag:  basefont  
    pub const BASEFONT: HtmlTag = HtmlTag(BASEFONT);
    ///   This represents the html tag:  bdi  
    pub const BDI: HtmlTag = HtmlTag(BDI);
    ///   This represents the html tag:  bdo  
    pub const BDO: HtmlTag = HtmlTag(BDO);
    ///   This represents the html tag:  big  
    pub const BIG: HtmlTag = HtmlTag(BIG);
    ///   This represents the html tag:  blockquote  
    pub const BLOCKQUOTE: HtmlTag = HtmlTag(BLOCKQUOTE);
    ///   This represents the html tag:  body  
    pub const BODY: HtmlTag = HtmlTag(BODY);
    ///   This represents the html tag:  br  
    pub const BR: HtmlTag = HtmlTag(BR);
    ///   This represents the html tag:  button  
    pub const BUTTON: HtmlTag = HtmlTag(BUTTON);
    ///   This represents the html tag:  canvas  
    pub const CANVAS: HtmlTag = HtmlTag(CANVAS);
    ///   This represents the html tag:  caption  
    pub const CAPTION: HtmlTag = HtmlTag(CAPTION);
    ///   This represents the html tag:  center  
    pub const CENTER: HtmlTag = HtmlTag(CENTER);
    ///   This represents the html tag:  cite  
    pub const CITE: HtmlTag = HtmlTag(CITE);
    ///   This represents the html tag:  code  
    pub const CODE: HtmlTag = HtmlTag(CODE);
    ///   This represents the html tag:  col  
    pub const COL: HtmlTag = HtmlTag(COL);
    ///   This represents the html tag:  colgroup  
    pub const COLGROUP: HtmlTag = HtmlTag(COLGROUP);
    ///   This represents the html tag:  datalist  
    pub const DATALIST: HtmlTag = HtmlTag(DATALIST);
    ///   This represents the html tag:  dd  
    pub const DD: HtmlTag = HtmlTag(DD);
    ///   This represents the html tag:  del  
    pub const DEL: HtmlTag = HtmlTag(DEL);
    ///   This represents the html tag:  details  
    pub const DETAILS: HtmlTag = HtmlTag(DETAILS);
    ///   This represents the html tag:  dfn  
    pub const DFN: HtmlTag = HtmlTag(DFN);
    ///   This represents the html tag:  dir  
    pub const DIR: HtmlTag = HtmlTag(DIR);
    ///   This represents the html tag:  div  
    pub const DIV: HtmlTag = HtmlTag(DIV);
    ///   This represents the html tag:  dl  
    pub const DL: HtmlTag = HtmlTag(DL);
    ///   This represents the html tag:  dt  
    pub const DT: HtmlTag = HtmlTag(DT);
    ///   This represents the html tag:  em  
    pub const EM: HtmlTag = HtmlTag(EM);
    ///   This represents the html tag:  embed  
    pub const EMBED: HtmlTag = HtmlTag(EMBED);
    ///   This represents the html tag:  fieldset  
    pub const FIELDSET: HtmlTag = HtmlTag(FIELDSET);
    ///   This represents the html tag:  figcaption
    pub const FIGCAPTION: HtmlTag = HtmlTag(FIGCAPTION);
    ///   This represents the html tag:  figure
    pub const FIGURE: HtmlTag = HtmlTag(FIGURE);
    ///   This represents the html tag:  font
    pub const FONT: HtmlTag = HtmlTag(FONT);
    ///   This represents the html tag:  footer
    pub const FOOTER: HtmlTag = HtmlTag(FOOTER);
    ///   This represents the html tag:  form
    pub const FORM: HtmlTag = HtmlTag(FORM);
    ///   This represents the html tag:  frame
    pub const FRAME: HtmlTag = HtmlTag(FRAME);
    ///   This represents the html tag:  frameset
    pub const FRAMESET: HtmlTag = HtmlTag(FRAMESET);
    ///   This represents the html tag:  h1
    pub const H1: HtmlTag = HtmlTag(H1);
    ///   This represents the html tag:  h2
    pub const H2: HtmlTag = HtmlTag(H2);
    ///   This represents the html tag:  h3
    pub const H3: HtmlTag = HtmlTag(H3);
    ///   This represents the html tag:  h4
    pub const H4: HtmlTag = HtmlTag(H4);
    ///   This represents the html tag:  h5
    pub const H5: HtmlTag = HtmlTag(H5);
    ///   This represents the html tag:  h6
    pub const H6: HtmlTag = HtmlTag(H6);
    ///   This represents the html tag:  head
    pub const HEAD: HtmlTag = HtmlTag(HEAD);
    ///   This represents the html tag:  header
    pub const HEADER: HtmlTag = HtmlTag(HEADER);
    ///   This represents the html tag:   hr
    pub const HR: HtmlTag = HtmlTag(HR);
    ///   This represents the html tag:  html
    pub const HTML: HtmlTag = HtmlTag(HTML);
    ///   This represents the html tag:  i
    pub const I: HtmlTag = HtmlTag(I);
    ///   This represents the html tag:  iframe
    pub const IFRAME: HtmlTag = HtmlTag(IFRAME);
    ///   This represents the html tag:  img
    pub const IMG: HtmlTag = HtmlTag(IMG);
    ///   This represents the html tag:  input
    pub const INPUT: HtmlTag = HtmlTag(INPUT);
    ///   This represents the html tag:  ins
    pub const INS: HtmlTag = HtmlTag(INS);
    ///   This represents the html tag:  kbd
    pub const KBD: HtmlTag = HtmlTag(KBD);
    ///   This represents the html tag:  keygen
    pub const KEYGEN: HtmlTag = HtmlTag(KEYGEN);
    ///   This represents the html tag:  label
    pub const LABEL: HtmlTag = HtmlTag(LABEL);
    ///   This represents the html tag:  legend
    pub const LEGEND: HtmlTag = HtmlTag(LEGEND);
    ///   This represents the html tag:  li
    pub const LI: HtmlTag = HtmlTag(LI);
    ///   This represents the html tag:  link
    pub const LINK: HtmlTag = HtmlTag(LINK);
    ///   This represents the html tag:  main
    pub const MAIN: HtmlTag = HtmlTag(MAIN);
    ///   This represents the html tag:  map
    pub const MAP: HtmlTag = HtmlTag(MAP);
    ///   This represents the html tag:  mark
    pub const MARK: HtmlTag = HtmlTag(MARK);
    ///   This represents the html tag:  menu
    pub const MENU: HtmlTag = HtmlTag(MENU);
    ///   This represents the html tag:  menuitem
    pub const MENUITEM: HtmlTag = HtmlTag(MENUITEM);
    ///   This represents the html tag:  meta
    pub const META: HtmlTag = HtmlTag(META);
    ///   This represents the html tag:  meter
    pub const METER: HtmlTag = HtmlTag(METER);
    ///   This represents the html tag:  nav
    pub const NAV: HtmlTag = HtmlTag(NAV);
    ///   This represents the html tag:  noframes
    pub const NOFRAMES: HtmlTag = HtmlTag(NOFRAMES);
    ///   This represents the html tag:  noscript
    pub const NOSCRIPT: HtmlTag = HtmlTag(NOSCRIPT);
    ///   This represents the html tag:  object
    pub const OBJECT: HtmlTag = HtmlTag(OBJECT);
    ///   This represents the html tag:  ol
    pub const OL: HtmlTag = HtmlTag(OL);
    ///   This represents the html tag:  optgroup
    pub const OPTGROUP: HtmlTag = HtmlTag(OPTGROUP);
    ///   This represents the html tag:  option
    pub const OPTION: HtmlTag = HtmlTag(OPTION);
    ///   This represents the html tag:  output
    pub const OUTPUT: HtmlTag = HtmlTag(OUTPUT);
    ///   This represents the html tag:  p
    pub const P: HtmlTag = HtmlTag(P);
    ///   This represents the html tag:  param
    pub const PARAM: HtmlTag = HtmlTag(PARAM);
    ///   This represents the html tag:  pre
    pub const PRE: HtmlTag = HtmlTag(PRE);
    ///   This represents the html tag:  progress
    pub const PROGRESS: HtmlTag = HtmlTag(PROGRESS);
    ///   This represents the html tag:  q
    pub const Q: HtmlTag = HtmlTag(Q);
    ///   This represents the html tag:  s
    pub const S: HtmlTag = HtmlTag(S);
    ///   This represents the html tag:  samp
    pub const SAMP: HtmlTag = HtmlTag(SAMP);
    ///   This represents the html tag:  script
    pub const SCRIPT: HtmlTag = HtmlTag(SCRIPT);
    ///   This represents the html tag:  section
    pub const SECTION: HtmlTag = HtmlTag(SECTION);
    ///   This represents the html tag:  select
    pub const SELECT: HtmlTag = HtmlTag(SELECT);
    ///   This represents the html tag:  small
    pub const SMALL: HtmlTag = HtmlTag(SMALL);
    ///   This represents the html tag:  source
    pub const SOURCE: HtmlTag = HtmlTag(SOURCE);
    ///   This represents the html tag:  span
    pub const SPAN: HtmlTag = HtmlTag(SPAN);
    ///   This represents the html tag:  strike
    pub const STRIKE: HtmlTag = HtmlTag(STRIKE);
    ///   This represents the html tag:  strong
    pub const STRONG: HtmlTag = HtmlTag(STRONG);
    ///   This represents the html tag:  style
    pub const STYLE: HtmlTag = HtmlTag(STYLE);
    ///   This represents the html tag:  sub
    pub const SUB: HtmlTag = HtmlTag(SUB);
    ///   This represents the html tag:  summary
    pub const SUMMARY: HtmlTag = HtmlTag(SUMMARY);
    ///   This represents the html tag:  sup
    pub const SUP: HtmlTag = HtmlTag(SUP);
    ///   This represents the html tag:  table
    pub const TABLE: HtmlTag = HtmlTag(TABLE);
    ///   This represents the html tag:  tbody
    pub const TBODY: HtmlTag = HtmlTag(TBODY);
    ///   This represents the html tag:  td
    pub const TD: HtmlTag = HtmlTag(TD);
    ///   This represents the html tag:  textarea
    pub const TEXTAREA: HtmlTag = HtmlTag(TEXTAREA);
    ///   This represents the html tag:  tfoot
    pub const TFOOT: HtmlTag = HtmlTag(TFOOT);
    ///   This represents the html tag:  th
    pub const TH: HtmlTag = HtmlTag(TH);
    ///   This represents the html tag:  thead
    pub const THEAD: HtmlTag = HtmlTag(THEAD);
    ///   This represents the html tag:  time
    pub const TIME: HtmlTag = HtmlTag(TIME);
    ///   This represents the html tag:  title
    pub const TITLE: HtmlTag = HtmlTag(TITLE);
    ///   This represents the html tag:  tr
    pub const TR: HtmlTag = HtmlTag(TR);
    ///   This represents the html tag:  track
    pub const TRACK: HtmlTag = HtmlTag(TRACK);
    ///   This represents the html tag:  tt
    pub const TT: HtmlTag = HtmlTag(TT);
    ///   This represents the html tag:  u
    pub const U: HtmlTag = HtmlTag(U);
    ///   This represents the html tag:  ul
    pub const UL: HtmlTag = HtmlTag(UL);
    ///   This represents the html tag:  var
    pub const VAR: HtmlTag = HtmlTag(VAR);
    ///   This represents the html tag:  video
    pub const VIDEO: HtmlTag = HtmlTag(VIDEO);
    ///  This gets the content inside the specified tag
    pub fn parse_tags(&self, input: &str) -> Vec<String> {
        let re = Regex::new(&format!(r#"(?s)<{}[^>]*>(.*?)</{}>"#, &self, &self)).unwrap();

        let mut result = Vec::new();

        for capture in re.captures_iter(input) {
            if let Some(text) = capture.get(1) {
                result.push(text.as_str().trim().to_string());
            }
        }

        result
    }
}

// endregion:  ---Make html tag struct
