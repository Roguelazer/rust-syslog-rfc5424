#[macro_use]
extern crate timeit;
extern crate syslog_rfc5424;

use syslog_rfc5424::parse_message;

// Stupid benchmark tool using the timeit! macro because the official benchmarking tools are
// **still* nightly-Rust-only, even though they're, like, a year old

fn main() {
    println!("Parsing the smallest possible message:");
    let simple_message = "<1>1 - - - - - -";
    timeit!({
        parse_message(simple_message).unwrap();
    });
    println!("Parsing a complicated message:");
    let complicated_message = "<78>1 2016-01-15T00:04:01Z host1 CROND 10391 - [meta sequenceId=\"29\" sequenceBlah=\"foo\"][my key=\"value\"] some_message";
    timeit!({
        parse_message(complicated_message).unwrap();
    });
    println!("Parsing a very long message:");
    let large_message = r#"<190>1 2016-02-21T01:19:11+00:00 batch6sj - - - [meta sequenceId="21881798" x-group="37051387"][origin x-service="tracking"] metascutellar conversationalist nephralgic exogenetic graphy streng outtaken acouasm amateurism prenotice Lyonese bedull antigrammatical diosphenol gastriloquial bayoneteer sweetener naggy roughhouser dighter addend sulphacid uneffectless ferroprussiate reveal Mazdaist plaudite Australasian distributival wiseman rumness Seidel topazine shahdom sinsion mesmerically pinguedinous ophthalmotonometer scuppler wound eciliate expectedly carriwitchet dictatorialism bindweb pyelitic idic atule kokoon poultryproof rusticial seedlip nitrosate splenadenoma holobenthic uneternal Phocaean epigenic doubtlessly indirection torticollar robomb adoptedly outspeak wappenschawing talalgia Goop domitic savola unstrafed carded unmagnified mythologically orchester obliteration imperialine undisobeyed galvanoplastical cycloplegia quinquennia foremean umbonal marcgraviaceous happenstance theoretical necropoles wayworn Igbira pseudoangelic raising unfrounced lamasary centaurial Japanolatry microlepidoptera"#;
    timeit!({
        parse_message(large_message).unwrap();
    });
    println!("Parsing an average message:");
    let average_message = r#"<29>1 2016-02-21T04:32:57+00:00 web1 someservice - - [origin x-service="someservice"][meta sequenceId="14125553"] 127.0.0.1 - - 1456029177 "GET /v1/ok HTTP/1.1" 200 145 "-" "hacheck 0.9.0" 24306 127.0.0.1:40124 575"#;
    timeit!({
        parse_message(average_message).unwrap();
    });
}
