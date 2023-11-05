use anyhow::Result;

use http_server::*;
use slight_http_handler_macro::register_handler;
use slight_http_server_macro::on_server_init;

wit_bindgen_rust::import!("../wit/http-server.wit");
wit_bindgen_rust::export!("../wit/http-server-export.wit");
wit_bindgen_rust::import!("../wit/http-client.wit");
wit_error_rs::impl_error!(http_server::HttpRouterError);
wit_error_rs::impl_error!(http_client::HttpError);

use configs::*;
wit_bindgen_rust::import!("../wit/configs.wit");
wit_error_rs::impl_error!(ConfigsError);

use keyvalue::*;
wit_bindgen_rust::import!("../wit/keyvalue.wit");
wit_error_rs::impl_error!(KeyvalueError);

use manifestverifier::ManifestVerifierClient;
use resourceverifier::ResourceVerifierClient;
use metadataverifier::MetadataVerifierClient;
use locateverifier::{LocateVerifierClient,SimplifiedLocatorCFI};

use resolver::{ResolverClient,StreamerInfo};
use register::RegisterClient;


use std::collections::HashMap;

use std::borrow::BorrowMut;
use std::sync::{Mutex, Once};

use std::path::Path;
use std::ffi::OsStr;
use std::io::Cursor;
use std::io::Read;

use multipart_2021 as multipart;

use multipart::server::{Multipart};
use serde::{Deserialize, Serialize};

use std::convert::TryFrom;
use uriparse::RelativeReference;
use qstring::QString;

mod policy;
use policy::Policy;

static mut CONFIG_INSTANCE: Option<Mutex<String>> = None;
static INIT: Once = Once::new();

fn static_path<'a>() -> &'a Mutex<String> {
    INIT.call_once(|| {
        // Since this access is inside a call_once, it is safe
        if let Ok(configs) = Configs::open("config-store") {
            if let Ok(r) = configs.get(&"FILEPATH") {
                if let Ok(instance) = String::from_utf8(r){
                    unsafe {
                        *CONFIG_INSTANCE.borrow_mut() = Some(Mutex::new(instance));
                    }
                }
            }
        }
    });

    // As long as this function is the only place with access to the static variable,
    // giving out read-only borrow here is safe because it is guaranteed no more mutable
    // references will exist at this point or in the future.
    unsafe { CONFIG_INSTANCE.as_ref().unwrap() }
}

static mut API_CONFIG_INSTANCE: Option<Mutex<String>> = None;
static API_INIT: Once = Once::new();

fn streamer_api<'a>() -> &'a Mutex<String> {
    API_INIT.call_once(|| {
        // Since this access is inside a call_once, it is safe
        if let Ok(configs) = Configs::open("config-store") {
            if let Ok(r) = configs.get(&"STREAMER_API") {
                if let Ok(instance) = String::from_utf8(r){
                    unsafe {
                        *API_CONFIG_INSTANCE.borrow_mut() = Some(Mutex::new(instance));
                    }
                }
            }
        }
    });

    // As long as this function is the only place with access to the static variable,
    // giving out read-only borrow here is safe because it is guaranteed no more mutable
    // references will exist at this point or in the future.
    unsafe { API_CONFIG_INSTANCE.as_ref().unwrap() }
}

static mut KEYVALUE_INSTANCE: Option<Mutex<Keyvalue>> = None;
static KEYVALUE_INIT: Once = Once::new();

fn keyvalue<'a>() -> &'a Mutex<Keyvalue> {
    KEYVALUE_INIT.call_once(|| {

        if let Ok(kv) = Keyvalue::open("streamer") {
                unsafe {
                    *KEYVALUE_INSTANCE.borrow_mut() = Some(Mutex::new(kv));
                }
        }

    });

    unsafe { KEYVALUE_INSTANCE.as_ref().unwrap() }
}

fn extract_boundary(content_type: &str) -> Option<&str> {
    // Buscar el parámetro "boundary" en la cabecera Content-Type
    if let Some(start) = content_type.find("boundary=") {
        let boundary_start = &content_type[start + "boundary=".len()..];
        let boundary_end = boundary_start.find(';').unwrap_or(boundary_start.len());
        Some(&boundary_start[..boundary_end])
    } else {
        None
    }
}


#[derive(Serialize, Deserialize,Default,Debug)]
struct EPubForm {
    pub name: Option<String>,
    pub tel: Option<String>,
    pub epub: Vec<u8>,
    pub storage: Option<String>
}

#[derive(Serialize, Deserialize,Default,Debug)]
struct AddStorageForm {
    pub storage: Option<String>,
}


const p : &str = r#"

    {
    "metadata":{
       "title":"OPDS 2.0 Test Catalog"
    },
    "links":[
       {
          "type":"application/opds+json",
          "rel":"self",
          "href":"https://3000-brown-lamprey-6p01dlro.ws-eu104.gitpod.io/opds2/publications.json"
       }
    ],
    "publications":[
       {
          "metadata":{
             "@type":"http://schema.org/Book",
             "title":"A Fractional Supervision Game Model of Multiple Stakeholders and Numerical Simulation",
             "identifier":"9123624",
             "author":[
                {
                   "name":"Rongwu Lu"
                },
                {
                   "name":"Xinhua Wang"
                },
                {
                   "name":"Dan Li"
                },
                {
                   "name":"Vladimir Turetsky"
                }
             ],
             "language":[
                "en"
             ]
          },
          "links":[
             {
                "type":"application/webpub+json",
                "rel":"http://opds-spec.org/acquisition",
                "href":"https://3000-brown-lamprey-6p01dlro.ws-eu104.gitpod.io/pub/352c0eedf4e42db2eea772a2d923cc97322589097ce1ba5928bc429aafcb38b6/manifest.json"
             }
          ],
          "images":[
             {
                "type":"image/jpeg",
                "height":169,
                "width":110,
                "href":"https://test.opds.io/assets/moby/small.jpg"
             },
             {
                "type":"image/jpeg",
                "height":400,
                "width":260,
                "href":"https://test.opds.io/assets/moby/normal.jpg"
             }
          ]
       }
    ]
 }
 "#;

const p_xml : &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns:dcterms="http://purl.org/dc/terms/" xmlns:thr="http://purl.org/syndication/thread/1.0" xmlns:opds="http://opds-spec.org/2010/catalog" xml:lang="en" xmlns:opensearch="http://a9.com/-/spec/opensearch/1.1/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns="http://www.w3.org/2005/Atom">
  <id>main.xml</id>
  <title>First Acquisition Feed</title>
  <updated>2012-10-20T01:11:18Z</updated>
  <author>
    <name>Hadrien Gardeur</name>
    <uri>https://github.com/Feedbooks</uri>
  </author>
  
  <link type="application/atom+xml; profile=opds-catalog; kind=acquisition" rel="self" href="main.xml" />
  <link type="application/atom+xml;profile=opds-catalog;kind=navigation" rel="start" href="../root.xml"/>
  <link type="application/opensearchdescription+xml" title="Search on test catalog" rel="search" href="../opensearch.xml" />
  
  <link type="application/atom+xml; profile=opds-catalog; kind=acquisition" rel="next" href="page2.xml" />
  <opensearch:totalResults>18</opensearch:totalResults>
  
  <entry>
    <title>Pride and Prejudice</title>
    <id>http://feedbooks.github.io/test/1</id>
    <author>
      <name>Jane Austen</name>
    </author>
    <published>2006-12-28T20:00:54Z</published>
    <updated>2012-10-12T17:43:18Z</updated>
    <dcterms:language>en</dcterms:language>
    <dcterms:issued>1813</dcterms:issued>
    <category label="Fiction" term="FBFIC000000" />
    <category label="Romance" term="FBFIC027000" />
    <content type="html">
      Pride And Prejudice, the story of Mrs. Bennet's attempts to marry off her five daughters is one of the best-loved and most enduring classics in English literature. Excitement fizzes through the Bennet household at Longbourn in Hertfordshire when young, eligible Mr. Charles Bingley rents the fine house nearby. He may have sisters, but he also has male friends, and one of these—the haughty, and even wealthier, Mr. Fitzwilliam Darcy—irks the vivacious Elizabeth Bennet, the second of the Bennet girls. She annoys him. Which is how we know they must one day marry. The romantic clash between the opinionated Elizabeth and Darcy is a splendid rendition of civilized sparring. As the characters dance a delicate quadrille of flirtation and intrigue, Jane Austen's radiantly caustic wit and keen observation sparkle.
    </content>
    <rights>This work is available for countries where copyright is Life+70 and in the USA.</rights>
    <dcterms:extent>120,697 words</dcterms:extent>
    <link type="application/epub+zip" rel="http://opds-spec.org/acquisition" href="http://www.feedbooks.com/book/52.epub" />
    <link type="image/jpeg" rel="http://opds-spec.org/image" href="../larger.jpg" />
    <link type="image/jpeg" rel="http://opds-spec.org/image/thumbnail" href="../thumbnail.jpg" />
    <link type="text/html" rel="related" href="http://www.feedbooks.com/book/52" title="Some link for the test" />
  </entry>


  <entry>
    <title>Acquisition: Multiple links</title>
    <id>http://feedbooks.github.io/test/15</id>
    <author>
      <name>Jane Austen</name>
    </author>
    <published>2006-12-28T20:00:54Z</published>
    <updated>2012-10-12T17:43:18Z</updated>
    <dcterms:language>en</dcterms:language>
    <dcterms:issued>1813</dcterms:issued>
    <category label="Fiction" term="FBFIC000000" />
    <category label="Romance" term="FBFIC027000" />
    <content type="html">
      Pride And Prejudice, the story of Mrs. Bennet's attempts to marry off her five daughters is one of the best-loved and most enduring classics in English literature. Excitement fizzes through the Bennet household at Longbourn in Hertfordshire when young, eligible Mr. Charles Bingley rents the fine house nearby. He may have sisters, but he also has male friends, and one of these—the haughty, and even wealthier, Mr. Fitzwilliam Darcy—irks the vivacious Elizabeth Bennet, the second of the Bennet girls. She annoys him. Which is how we know they must one day marry. The romantic clash between the opinionated Elizabeth and Darcy is a splendid rendition of civilized sparring. As the characters dance a delicate quadrille of flirtation and intrigue, Jane Austen's radiantly caustic wit and keen observation sparkle.
    </content>
    <link type="text/html" rel="http://opds-spec.org/acquisition/buy" href="http://www.feedbooks.com/book/52">
      <opds:price currencycode="USD">10.99</opds:price>
      <opds:indirectAcquisition type="application/epub+zip" />
    </link>
    <link type="text/html" rel="http://opds-spec.org/acquisition/borrow" href="http://www.feedbooks.com/book/52">
      <opds:indirectAcquisition type="application/epub+zip" />
    </link>
    <link type="application/epub+zip" rel="http://opds-spec.org/acquisition/sample" href="http://www.feedbooks.com/book/52.epub" />
    <link type="image/jpeg" rel="http://opds-spec.org/image" href="../larger.jpg" />
    <link type="image/jpeg" rel="http://opds-spec.org/image/thumbnail" href="../thumbnail.jpg" />
  </entry>

  <entry>
    <title>Acquisition: Indirect good</title>
    <id>http://feedbooks.github.io/test/16</id>
    <author>
      <name>Jane Austen</name>
    </author>
    <published>2006-12-28T20:00:54Z</published>
    <updated>2012-10-12T17:43:18Z</updated>
    <dcterms:language>en</dcterms:language>
    <dcterms:issued>1813</dcterms:issued>
    <category label="Fiction" term="FBFIC000000" />
    <category label="Romance" term="FBFIC027000" />
    <content type="html">
      Pride And Prejudice, the story of Mrs. Bennet's attempts to marry off her five daughters is one of the best-loved and most enduring classics in English literature. Excitement fizzes through the Bennet household at Longbourn in Hertfordshire when young, eligible Mr. Charles Bingley rents the fine house nearby. He may have sisters, but he also has male friends, and one of these—the haughty, and even wealthier, Mr. Fitzwilliam Darcy—irks the vivacious Elizabeth Bennet, the second of the Bennet girls. She annoys him. Which is how we know they must one day marry. The romantic clash between the opinionated Elizabeth and Darcy is a splendid rendition of civilized sparring. As the characters dance a delicate quadrille of flirtation and intrigue, Jane Austen's radiantly caustic wit and keen observation sparkle.
    </content>
    <link type="text/html" rel="http://opds-spec.org/acquisition" href="http://www.feedbooks.com/book/52">
      <opds:indirectAcquisition type="application/epub+zip" />
    </link>
    <link type="image/jpeg" rel="http://opds-spec.org/image" href="../larger.jpg" />
    <link type="image/jpeg" rel="http://opds-spec.org/image/thumbnail" href="../thumbnail.jpg" />
  </entry>

  <entry>
    <title>Acquisition: Indirect bad</title>
    <id>http://feedbooks.github.io/test/17</id>
    <author>
      <name>Jane Austen</name>
    </author>
    <published>2006-12-28T20:00:54Z</published>
    <updated>2012-10-12T17:43:18Z</updated>
    <dcterms:language>en</dcterms:language>
    <dcterms:issued>1813</dcterms:issued>
    <category label="Fiction" term="FBFIC000000" />
    <category label="Romance" term="FBFIC027000" />
    <content type="html">
      Pride And Prejudice, the story of Mrs. Bennet's attempts to marry off her five daughters is one of the best-loved and most enduring classics in English literature. Excitement fizzes through the Bennet household at Longbourn in Hertfordshire when young, eligible Mr. Charles Bingley rents the fine house nearby. He may have sisters, but he also has male friends, and one of these—the haughty, and even wealthier, Mr. Fitzwilliam Darcy—irks the vivacious Elizabeth Bennet, the second of the Bennet girls. She annoys him. Which is how we know they must one day marry. The romantic clash between the opinionated Elizabeth and Darcy is a splendid rendition of civilized sparring. As the characters dance a delicate quadrille of flirtation and intrigue, Jane Austen's radiantly caustic wit and keen observation sparkle.
    </content>
    <link type="text/imaginarytype" rel="http://opds-spec.org/acquisition" href="http://www.feedbooks.com/book/52">
      <opds:indirectAcquisition type="application/epub+zip" />
    </link>
    <link type="image/jpeg" rel="http://opds-spec.org/image" href="../larger.jpg" />
    <link type="image/jpeg" rel="http://opds-spec.org/image/thumbnail" href="../thumbnail.jpg" />
  </entry>

</feed>
"#;


const DOCUMENT_URL : &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>JATS Viewer for Books and Journal articles</title>
    
    <link rel="stylesheet" type="text/css" media="all"
          href="https://maxcdn.bootstrapcdn.com/font-awesome/4.2.0/css/font-awesome.min.css"/>

    <link rel='stylesheet' type='text/css' href="https://$HOST$/static/reader/lib/fonts.css"/>


    <!-- A combined lens.css will be generated in the bundling process -->
    <!-- While in development, separate links for each CSS file are added, so we don't need a source map -->
    <link rel="stylesheet" href="https://$HOST$/static/reader/lens.css"/>


    <!-- jQuery -->
    <script src="https://code.jquery.com/jquery-3.6.0.min.js"></script>

    <!-- SweetAlert2 -->
    <script src="https://cdn.jsdelivr.net/npm/sweetalert2@11/dist/sweetalert2.all.min.js"></script>

    <!-- MathJax Configuration -->
    <script type="text/x-mathjax-config">
        MathJax.Hub.Config({
          jax: ["input/TeX", "input/MathML","output/HTML-CSS"],
          SVG: { linebreaks: { automatic: true }, EqnChunk: 9999  },
          "displayAlign": "left",
          styles: {".MathJax_Display": {padding: "0em 0em 0em 3em" },".MathJax_SVG_Display": {padding: "0em 0em 0em 3em" }}
        });
      </script>
      <script type="text/javascript" src="https://cdnjs.cloudflare.com/ajax/libs/mathjax/2.7.1/MathJax.js?config=TeX-AMS-MML_HTMLorMML"></script>

    <!-- MathJax Configuration 
    <script type="text/x-mathjax-config">
       MathJax.Hub.Config({
         jax: ["input/TeX", "input/MathML","output/HTML-CSS"],
         extensions: ["MathMenu.js","MathZoom.js", "CHTML-preview.js"],
         "HTML-CSS": { linebreaks: { automatic: true }, EqnChunk: 9999 },
         SVG: { linebreaks: { automatic: true }, EqnChunk: 9999  },
         TeX: {
           extensions: ["AMSmath.js","AMSsymbols.js","noErrors.js","noUndefined.js"]
         },
         "displayAlign": "left",
         styles: {".MathJax_Display": {padding: "0em 0em 0em 3em" },".MathJax_SVG_Display": {padding: "0em 0em 0em 3em" },}
          });


    </script>
    

    <script type="text/javascript"     src="https://cdnjs.cloudflare.com/ajax/libs/mathjax/2.7.1/MathJax.js?config=MML_HTMLorMML-full"></script>
-->

    <script type="text/javascript">
        let locales = {
            'Author': 'Author',
            'Back': 'Back',
            'Contents': 'Contents',
            'CopyRightAndLicenses': 'Copyright & License',
            'Figures': 'Figures',
            'Focus': 'Focus',
            'Reference': '',
            'References': 'References',
        };
      const  submissionID=7;
      const  fileID=7;

    </script>

    <script src='https://$HOST$/static/reader/prelude.js'></script>

    <script type="text/javascript">

        if ('registerProtocolHandler' in navigator) {
           console.log('registerProtocolHandler');
           navigator.registerProtocolHandler('web+urs', '/urs?url=%s');
        }

        // This document gets loaded by default
        // --------

        var documentURL = "$DOCUMENT_URL$";

        $(function() {

            // Create a new Lens app instance
            // --------
            //
            // Injects itself into body

            var app = new window.Lens({
                document_url: documentURL

            });

            app.start();

            window.app = app;

        });
    </script>

    <script src='https://$HOST$/static/reader/lens.js'></script>
<style type="text/css">
    .sc-pinned-message {
        display: flex;
        max-width: 600px;
        margin: 7px auto;
        color: #000;
        text-align: center;
    }
    .sc-pinned-message > .se-msg-wrap {
        color: #ff0000;
        font-size: 26px;
        margin-right: 20px;
    }
    .sc-pinned-message > .se-msg-wrap > .se-icon {
        text-decoration: underline;
    }
    .fa {
        display: inline-block;
        font: normal normal normal 14px/1 FontAwesome;
        font-size: inherit;
        text-rendering: auto;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
    }
    .sc-pinned-message > .se-msg-wrap > .se-msg {
        content: attr(data-placeholder);
        position: absolute;
        color: #ccc;
        font-weight: 400;
    }
</style>
</head>
<body class="loading">

</body>

</html>"#;

const document_url_old : &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Lens Viewer for Books and Journal articles</title>

    <link rel="stylesheet" type="text/css" media="all"
          href="https://maxcdn.bootstrapcdn.com/font-awesome/4.2.0/css/font-awesome.min.css"/>

    <link rel='stylesheet' type='text/css' href="https://$HOST$/static/reader/lib/fonts.css"/>


    <!-- A combined lens.css will be generated in the bundling process -->
    <!-- While in development, separate links for each CSS file are added, so we don't need a source map -->
    <link rel="stylesheet" type="text/css" href="https://$HOST$/static/reader/lens.css"/>

    <script src="https://www.ub.uni-heidelberg.de/cdn/jquery/3.2.1/jquery.js"></script>

    <!-- MathJax Configuration -->
    <script type="text/x-mathjax-config">
        MathJax.Hub.Config({
          jax: ["input/TeX", "input/MathML","output/HTML-CSS"],
          SVG: { linebreaks: { automatic: true }, EqnChunk: 9999  },
          "displayAlign": "left",
          styles: {".MathJax_Display": {padding: "0em 0em 0em 3em" },".MathJax_SVG_Display": {padding: "0em 0em 0em 3em" }}
        });
      </script>

	  <script type="text/javascript" src="https://cdnjs.cloudflare.com/ajax/libs/mathjax/2.7.1/MathJax.js?config=TeX-AMS-MML_HTMLorMML"></script>

    <!-- MathJax Configuration 
    <script type="text/x-mathjax-config">
       MathJax.Hub.Config({
         jax: ["input/TeX", "input/MathML","output/HTML-CSS"],
         extensions: ["MathMenu.js","MathZoom.js", "CHTML-preview.js"],
         "HTML-CSS": { linebreaks: { automatic: true }, EqnChunk: 9999 },
         SVG: { linebreaks: { automatic: true }, EqnChunk: 9999  },
         TeX: {
           extensions: ["AMSmath.js","AMSsymbols.js","noErrors.js","noUndefined.js"]
         },
         "displayAlign": "left",
         styles: {".MathJax_Display": {padding: "0em 0em 0em 3em" },".MathJax_SVG_Display": {padding: "0em 0em 0em 3em" },}
          });


    </script>
    

    <script type="text/javascript"     src="https://cdnjs.cloudflare.com/ajax/libs/mathjax/2.7.1/MathJax.js?config=MML_HTMLorMML-full"></script>
-->

    <script type="text/javascript">
        let locales = {
            'Author': 'Author',
            'Back': 'Back',
            'Contents': 'Contents',
            'CopyRightAndLicenses': 'Copyright  License',
            'Figures': 'Figures',
            'Focus': 'Focus',
            'Reference': '',
            'References': 'References',
        };
      const  submissionID=7;
      const  fileID=7;

    </script>

    <script type="text/javascript">

        if ('registerProtocolHandler' in navigator) {
           console.log('registerProtocolHandler');
           navigator.registerProtocolHandler('web+urs', '/urs/%s');
        }

        // This document gets loaded by default
        // --------

        var documentURL = "$DOCUMENT_URL$";

        $(function() {

            // Create a new Lens app instance
            // --------
            //
            // Injects itself into body

            console.log('Test:',window.Lens)
            var app = new window.Lens({
                document_url: documentURL

            });

            app.start();

            window.app = app;

        });
    </script>

    <script src='https://$HOST$/static/reader/lens.js'></script>

<style type="text/css">
    .sc-pinned-message {
        display: flex;
        max-width: 600px;
        margin: 7px auto;
        color: #000;
        text-align: center;
    }
    .sc-pinned-message > .se-msg-wrap {
        color: #ff0000;
        font-size: 26px;
        margin-right: 20px;
    }
    .sc-pinned-message > .se-msg-wrap > .se-icon {
        text-decoration: underline;
    }
    .fa {
        display: inline-block;
        font: normal normal normal 14px/1 FontAwesome;
        font-size: inherit;
        text-rendering: auto;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
    }
    .sc-pinned-message > .se-msg-wrap > .se-msg {
        content: attr(data-placeholder);
        position: absolute;
        color: #ccc;
        font-weight: 400;
    }
</style>
</head>
<body class="loading">

</body>

</html>"#;

#[on_server_init]
fn main() -> Result<()> {
    let router = Router::new()?;
    let router_with_route = router
        .get("/hello", "handle_hello")?
        .get("/pub/:id/*","handle_resource_or_manifest")?
        .get("/metadata/:id","handle_metadata")?        
        .get("/raw/:id/*","handle_raw_resource")?        
        .get("/opds2/publications.json","handle_opds2")?
        .get("/locate/:id/:locator","handle_locator")?   
        .get("/static/*","handle_file")?
        .post("/add", "handle_add")?
        .post("/addstorage","handle_add_storage")?;

    println!("Starting server");
    let _ = Server::serve("0.0.0.0:3000", &router_with_route)?;
    // server.stop().unwrap();
    println!("Serving...");

    Ok(())
}

#[register_handler]
fn handle_hello(req: Request) -> Result<Response, HttpError> {
    Ok(Response {
        headers: Some(req.headers),
        body: Some("hello".as_bytes().to_vec()),
        status: 200,
    })
}

#[register_handler]
fn handle_add(request: Request) -> Result<Response, HttpError> {
    assert_eq!(request.method, Method::Post);

    let empty_body = Vec::<u8>::new();
    let binding = ("".into(), "".into());

    let content_type = request.headers.iter().find(|x| x.0 == "content-type").unwrap_or(&binding);

    let boundary_opt = extract_boundary(&content_type.1);

    let policy = Policy::default();

    if let Some(boundary) = boundary_opt {
        
        let body = Cursor::new(request.body.as_ref().unwrap_or(&empty_body));
        let mut multipart = Multipart::with_body(body,boundary);
        let mut form = EPubForm::default();

        multipart.foreach_entry(| mut entry| match &*entry.headers.name {
            "name" => {
                let mut vec = Vec::new();
                entry.data.read_to_end(&mut vec).expect("can't read");
                form.name = String::from_utf8(vec).ok();
                println!("name got");
            }
    
            "tel" => {
                let mut vec = Vec::new();
                entry.data.read_to_end(&mut vec).expect("can't read");
                form.tel = String::from_utf8(vec).ok();
                println!("tel got");
            }
    
            "epub" => {
                let mut vec = Vec::new();
                entry.data.read_to_end(&mut vec).expect("can't read");
                form.epub = vec;

                println!("file got");
            }

            "storage" => {
                let mut vec = Vec::new();
                entry.data.read_to_end(&mut vec).expect("can't read");
                form.storage =  String::from_utf8(vec).ok();

                println!("storage got");
            }
    
            _ => {
                // as multipart has a bug https://github.com/abonander/multipart/issues/114
                // we manually do read_to_end here
                //let mut _vec = Vec::new();
                //entry.data.read_to_end(&mut _vec).expect("can't read");
                println!("key neglected");
            }
        })
        .expect("Unable to iterate multipart?");

        let client = RegisterClient::new(&policy.register_provider).map_err( |e| HttpError::UnexpectedError(e.to_string()))?;

        println!("Form: {:?}",&form.storage);

        let (id,st) = if let Some(storage)=form.storage {
            let st = storage.clone();
            (client.register_with(form.epub,storage).map_err( |e| HttpError::UnexpectedError(e.to_string()))?,st)
        }
        else {
            (client.register(form.epub).map_err( |e| HttpError::UnexpectedError(e.to_string()))?,"default".to_string())
        };
        
        let mut headers = Vec::new();
        headers.push((String::from("content-type"),"application/json".to_string()));

        let message = format!("{} on {}",&id,&st);

        Ok(Response {
            headers: Some(headers),
            body: serde_json::to_vec(&message).ok(),
            status: 200,
        })
    } else {
        Err(HttpError::UnexpectedError(std::format!("Boundary {} not found in header",content_type.1)))
    }
}

#[register_handler]
fn handle_add_storage(request: Request) -> Result<Response, HttpError> {
    assert_eq!(request.method, Method::Post);

    let empty_body = Vec::<u8>::new();
    let binding = ("".into(), "".into());

    let content_type = request.headers.iter().find(|x| x.0 == "content-type").unwrap_or(&binding);

    let boundary_opt = extract_boundary(&content_type.1);

    if let Some(boundary) = boundary_opt {
        
        let body = Cursor::new(request.body.as_ref().unwrap_or(&empty_body));
        let mut multipart = Multipart::with_body(body,boundary);
        let mut form = AddStorageForm::default();

        multipart.foreach_entry(| mut entry| match &*entry.headers.name {
            "storage" => {
                let mut vec = Vec::new();
                entry.data.read_to_end(&mut vec).expect("can't read");
                form.storage = String::from_utf8(vec).ok();
                println!("storage got");
            }
    
            _ => {
                // as multipart has a bug https://github.com/abonander/multipart/issues/114
                // we manually do read_to_end here
                //let mut _vec = Vec::new();
                //entry.data.read_to_end(&mut _vec).expect("can't read");
                println!("key neglected");
            }
        })
        .expect("Unable to iterate multipart?");


        let mut headers = Vec::new();
        headers.push((String::from("content-type"),"application/json".to_string()));

        let message = if let Some(storage) = &form.storage {
            (&*keyvalue().lock().unwrap()).set(storage,&[]).map_err( |e| HttpError::UnexpectedError(e.to_string()))?;
            "Ok".to_string()
        }else {
            "No content provider".to_string()
        };

        Ok(Response {
            headers: Some(headers),
            body: serde_json::to_vec(&message).ok(),
            status: 200,
        })
    } else {
        Err(HttpError::UnexpectedError(std::format!("Boundary {} not found in header",content_type.1)))
    }
}

#[register_handler]
fn handle_locator(request: Request) -> Result<Response, HttpError> {

    let binding = ("".into(), "".into());

    let id = request.params.iter().find(|x| x.0 == "id").unwrap_or(&binding);
    let locator = request.params.iter().find(|x| x.0 == "locator").unwrap_or(&binding);

    let reference = RelativeReference::try_from(request.uri.as_str()).map_err( |e| HttpError::UnexpectedError(e.to_string()))?;

    let policy = Policy::new(&id.1).map_err( |e|HttpError::UnexpectedError(e.to_string()))?;

    let locator_decoded = hex::decode(&locator.1).map_err( |e|HttpError::UnexpectedError(e.to_string()))?;
    let locator_decoded_str = std::str::from_utf8(&locator_decoded).map_err( |e|HttpError::UnexpectedError(e.to_string()))?;

    let s_locator : SimplifiedLocatorCFI = serde_json::from_str(locator_decoded_str).map_err( |e|HttpError::UnexpectedError(e.to_string()))?;

    println!("SimplifiedLocator {:?}",&s_locator);
    
    let locate = if reference.has_query() {
        let query = reference.query().ok_or( HttpError::UnexpectedError("No query but it seems there is one".to_string()) )?;

        let qs = QString::from(query.as_str());
        let rservice_param = qs.get("rservice");
        let lservice_param = qs.get("lservice");

        if let (Some(rservice),Some(lservice)) = (rservice_param, lservice_param) {
            println!("{} {} ",&rservice,&lservice);
            
            let rclient = ResolverClient::new(rservice).map_err( |e| HttpError::UnexpectedError(e.to_string()))?;
            let storages = rclient.get_streamers(id.1.to_owned()).map_err( |e| HttpError::UnexpectedError(e.to_string()))?;

            //Pick the first storage
            let storage = Policy::resolve_storage(&storages).map_err( |e| HttpError::UnexpectedError(e.to_string()))?;

            let lclient = LocateVerifierClient::new(&policy.locator_verifier_service).map_err( |e|HttpError::UnexpectedError(e.to_string()))?;
            lclient.locate_with_cfi(id.1.to_owned(), s_locator.href, s_locator.media_type, s_locator.cfi,
                                lservice.to_string(),storage.endpoint.to_owned()
                                ).map_err( |e| HttpError::UnexpectedError(e.to_string()))?

        }else {

            let rclient = ResolverClient::new(&policy.resolver_provider).map_err( |e| HttpError::UnexpectedError(e.to_string()))?;
            let storages = rclient.get_streamers(id.1.to_owned()).map_err( |e| HttpError::UnexpectedError(e.to_string()))?;

            //Pick the first storage
            let storage = Policy::resolve_storage(&storages).map_err( |e| HttpError::UnexpectedError(e.to_string()))?;

            let lclient = LocateVerifierClient::new(&policy.locator_verifier_service).map_err( |e| HttpError::UnexpectedError(e.to_string()))?;            
            lclient.locate_with_cfi(id.1.to_owned(),s_locator.href, s_locator.media_type, s_locator.cfi,
                            policy.locator_service,storage.endpoint.to_owned()
                            ).map_err( |e| HttpError::UnexpectedError(e.to_string()))?
        }

    } else {

        let rclient = ResolverClient::new(&policy.resolver_provider).map_err( |e| HttpError::UnexpectedError(e.to_string()))?;
        let storages = rclient.get_streamers(id.1.to_owned()).map_err( |e| HttpError::UnexpectedError(e.to_string()))?;

        //Pick the first storage
        let storage = Policy::resolve_storage(&storages).map_err( |e| HttpError::UnexpectedError(e.to_string()))?;

        let lclient = LocateVerifierClient::new(&policy.locator_verifier_service).map_err( |e| HttpError::UnexpectedError(e.to_string()))?;
        lclient.locate_with_cfi(id.1.to_owned(),s_locator.href, s_locator.media_type, s_locator.cfi,
                            policy.locator_service,storage.endpoint.to_owned()
                            ).map_err( |e| HttpError::UnexpectedError(e.to_string()))?

    };

    
    Ok(Response {
        headers: Some(request.headers),
        body: Some(locate.as_bytes().to_vec()),
        status: 200,
    })
}


#[register_handler]
fn handle_resource_or_manifest(request: Request) -> Result<Response, HttpError> {

    let binding = ("".into(), "".into());

    println!("{:?}",request);

    let id = request.params.iter().find(|x| x.0 == "id").unwrap_or(&binding);
    let path = request.params.iter().find(|x| x.0 == "*").unwrap_or(&binding);

    if path.1 == "manifest.json" {
        handle_manifest(request)
    }else {
        handle_resource(request)
    }
}


#[register_handler]
fn handle_metadata(request: Request) -> Result<Response, HttpError> {

    let binding = ("".into(), "".into());

    let id = request.params.iter().find(|x| x.0 == "id").unwrap_or(&binding);
    let path = request.params.iter().find(|x| x.0 == "*").unwrap_or(&binding);

    let host = request.headers.iter().find(|x| x.0 == "host").unwrap_or(&binding);

    let reference = RelativeReference::try_from(request.uri.as_str()).map_err( |e| HttpError::UnexpectedError(e.to_string()))?;

    let policy = Policy::new(&id.1).map_err( |e|HttpError::UnexpectedError(e.to_string()))?;

    let metadata = if reference.has_query() {
        let query = reference.query().ok_or( HttpError::UnexpectedError("No query but it seems there is one".to_string()) )?;

        let qs = QString::from(query.as_str());
        let service_param = qs.get("service");
        let storage_param = qs.get("storage");

        if let (Some(service),Some(storage)) = (service_param, storage_param) {
            println!("{} {} ",&service,&storage);
            
            let mclient = MetadataVerifierClient::new(&policy.metadata_verifier_service).map_err( |e|HttpError::UnexpectedError(e.to_string()))?;
            mclient.metadata_with(id.1.to_owned(),service.to_string(),storage.to_string()).map_err( |e| HttpError::UnexpectedError(e.to_string()))?

        }else {
            let mclient = MetadataVerifierClient::new(&policy.metadata_verifier_service).map_err( |e| HttpError::UnexpectedError(e.to_string()))?;            
            mclient.metadata_with(id.1.to_owned(),policy.metadata_service,policy.storage_provider)
                            .map_err( |e| HttpError::UnexpectedError(e.to_string()))?
        }

    } else {

        let mclient = MetadataVerifierClient::new(&policy.metadata_verifier_service).map_err( |e| HttpError::UnexpectedError(e.to_string()))?;
        mclient.metadata_with(id.1.to_owned(),policy.metadata_service,policy.storage_provider)
                            .map_err( |e| HttpError::UnexpectedError(e.to_string()))?

    };


    let mut headers = Vec::new();
    headers.push((String::from("content-type"),"application/json".to_string()));

    Ok(Response {
        headers: Some(headers),
        body: Some(metadata.into()),
        status: 200,
    })
}

fn handle_manifest(request: handle_resource_or_manifest_mod::Request) -> Result<handle_resource_or_manifest_mod::Response, handle_resource_or_manifest_mod::HttpError> {
    let id = request.params.into_iter().find(|x| x.0 == "id").unwrap_or(("".into(), "".into()));

    let reference = RelativeReference::try_from(request.uri.as_str()).map_err( |e| handle_resource_or_manifest_mod::HttpError::UnexpectedError(e.to_string()))?;
    
    let policy = Policy::new(&id.1).map_err( |e| handle_resource_or_manifest_mod::HttpError::UnexpectedError(e.to_string()))?;

    let manifest = if reference.has_query() {
        let query = reference.query().ok_or( handle_resource_or_manifest_mod::HttpError::UnexpectedError("No query but it seems there is one".to_string()) )?;

        let qs = QString::from(query.as_str());
        let service_param = qs.get("service");
        let storage_param = qs.get("storage");

        if let (Some(service),Some(storage)) = (service_param, storage_param) {
            println!("{} {} ",&service,&storage);

            let mclient = ManifestVerifierClient::new(&policy.manifest_verifier_service).map_err( |e| handle_resource_or_manifest_mod::HttpError::UnexpectedError(e.to_string()))?;
            mclient.manifest_with(id.1,service.to_string(),storage.to_string()).map_err( |e| handle_resource_or_manifest_mod::HttpError::UnexpectedError(e.to_string()))?
        }else {
            let mclient = ManifestVerifierClient::new(&policy.manifest_verifier_service).map_err( |e| handle_resource_or_manifest_mod::HttpError::UnexpectedError(e.to_string()))?;

            mclient.manifest_with(id.1,policy.manifest_service,policy.storage_provider)
                    .map_err( |e| handle_resource_or_manifest_mod::HttpError::UnexpectedError(e.to_string()))?
        }

    } else {

        let mclient = ManifestVerifierClient::new(&policy.manifest_verifier_service).map_err( |e| handle_resource_or_manifest_mod::HttpError::UnexpectedError(e.to_string()))?;
        
        mclient.manifest_with(id.1,policy.manifest_service,policy.storage_provider)
                    .map_err( |e| handle_resource_or_manifest_mod::HttpError::UnexpectedError(e.to_string()))?

    };

    let mut headers = request.headers.clone();
    headers.push((String::from("Content-Type"),manifest.0));

    Ok(handle_resource_or_manifest_mod::Response {
        headers: Some(headers),
        body: Some(manifest.1.into_bytes()),
        status: 200,
    })
}


fn handle_resource(request: handle_resource_or_manifest_mod::Request) -> Result<handle_resource_or_manifest_mod::Response, handle_resource_or_manifest_mod::HttpError> {

    let binding = ("".into(), "".into());

    let id = request.params.iter().find(|x| x.0 == "id").unwrap_or(&binding);
    let path = request.params.iter().find(|x| x.0 == "*").unwrap_or(&binding);

    let host = request.headers.iter().find(|x| x.0 == "host").unwrap_or(&binding);

    let reference = RelativeReference::try_from(request.uri.as_str()).map_err( |e| handle_resource_or_manifest_mod::HttpError::UnexpectedError(e.to_string()))?;

    let policy = Policy::new(&id.1).map_err( |e| handle_resource_or_manifest_mod::HttpError::UnexpectedError(e.to_string()))?;

    let resource = if reference.has_query() {
        let query = reference.query().ok_or( handle_resource_or_manifest_mod::HttpError::UnexpectedError("No query but it seems there is one".to_string()) )?;

        let qs = QString::from(query.as_str());
        let service_param = qs.get("service");
        let storage_param = qs.get("storage");

        if let (Some(service),Some(storage)) = (service_param, storage_param) {
            println!("{} {} ",&service,&storage);
            
            let rclient = ResourceVerifierClient::new(&policy.resource_verifier_service).map_err( |e| handle_resource_or_manifest_mod::HttpError::UnexpectedError(e.to_string()))?;
            rclient.resource_with(id.1.to_owned(),path.1.to_owned().into_bytes(),service.to_string(),storage.to_string()).map_err( |e| handle_resource_or_manifest_mod::HttpError::UnexpectedError(e.to_string()))?

        }else {
            let rclient = ResourceVerifierClient::new(&policy.resource_verifier_service).map_err( |e| handle_resource_or_manifest_mod::HttpError::UnexpectedError(e.to_string()))?;
            
            rclient.resource_with(id.1.to_owned(),path.1.to_owned().into_bytes(),policy.resource_service,policy.storage_provider)
                    .map_err( |e| handle_resource_or_manifest_mod::HttpError::UnexpectedError(e.to_string()))?
        }

    } else {

        let rclient = ResourceVerifierClient::new(&policy.resource_verifier_service).map_err( |e| handle_resource_or_manifest_mod::HttpError::UnexpectedError(e.to_string()))?;
        
        rclient.resource_with(id.1.to_owned(),path.1.to_owned().into_bytes(),policy.resource_service,policy.storage_provider)
            .map_err( |e| handle_resource_or_manifest_mod::HttpError::UnexpectedError(e.to_string()))?

    };

    //TO FIX. Get the content-type of the resource in a separate call
    let mut headers = request.headers.clone();

    if let Some(ctype) = resource.0 {        

        if ctype == "text/xml" {

            headers.push((String::from("Content-Type"),String::from("text/html")));

            let unencode_url = format!("https://{}/raw/{}/{}",&host.1,&id.1,&path.1);
            let new_url = urlencoding::encode(&unencode_url);

            let body = DOCUMENT_URL.replace("$DOCUMENT_URL$",&unencode_url);
            let body = body.replace("$HOST$",&host.1);

            return Ok(handle_resource_or_manifest_mod::Response {
                headers: Some(headers),
                body: Some(body.as_bytes().to_vec()),
                status: 200,
            })
            
        }else {
            headers.push((String::from("Content-Type"),ctype.to_owned()));
        }
        
    }

    
    Ok(handle_resource_or_manifest_mod::Response {
        headers: Some(headers),
        body: Some(resource.1),
        status: 200,
    })
}

/* 
fn handle_resource2(request: handle_resource_or_manifest_mod::Request) -> Result<handle_resource_or_manifest_mod::Response, handle_resource_or_manifest_mod::HttpError> {

    let binding = ("".into(), "".into());

    let id = request.params.iter().find(|x| x.0 == "id").unwrap_or(&binding);
    let path = request.params.iter().find(|x| x.0 == "*").unwrap_or(&binding);

    let host = request.headers.iter().find(|x| x.0 == "host").unwrap_or(&binding);

    let rclient = ResourceVerifierClient::new("resourceverifier_1").map_err( |e| handle_resource_or_manifest_mod::HttpError::UnexpectedError(e.to_string()))?;
    let resource = rclient.resource(id.1.to_owned(),path.1.to_owned().into_bytes()).map_err( |e| handle_resource_or_manifest_mod::HttpError::UnexpectedError(e.to_string()))?;

    let mut headers = request.headers.clone();

    if let Some(ctype) = resource.0 {        
        headers.push((String::from("Content-Type"),ctype.to_owned()));

        if ctype == "text/xml" {

            let unencode_url = format!("https://{}/raw/{}/{}",&host.1,&id.1,&path.1);
            let new_url = urlencoding::encode(&unencode_url);

            headers.push((String::from("Location"),format!("https://glittery-pavlova-a24826.netlify.app/?url={}",new_url)));

            return Ok(handle_resource_or_manifest_mod::Response {
                headers: Some(headers),
                body: None,
                status: 302,
            })
            
        }
        
    }

    
    Ok(handle_resource_or_manifest_mod::Response {
        headers: Some(headers),
        body: Some(resource.1),
        status: 200,
    })
}
*/


#[register_handler]
fn handle_raw_resource(request: Request) -> Result<Response,HttpError> {

    let binding = ("".into(), "".into());

    let id = request.params.iter().find(|x| x.0 == "id").unwrap_or(&binding);
    let path = request.params.iter().find(|x| x.0 == "*").unwrap_or(&binding);

    let policy = Policy::new(&id.1).map_err( |e|HttpError::UnexpectedError(e.to_string()))?;

    let rclient = ResourceVerifierClient::new(&policy.resource_verifier_service).map_err( |e| HttpError::UnexpectedError(e.to_string()))?;
    let resource = rclient.resource_with(id.1.to_owned(),path.1.to_owned().into_bytes(),policy.resource_service,policy.storage_provider)
            .map_err( |e| HttpError::UnexpectedError(e.to_string()))?;

    let mut headers = request.headers.clone();

    if let Some(ctype) = resource.0 {        
        headers.push((String::from("Content-Type"),ctype.to_owned()));
        
    }
    
    Ok(Response {
        headers: Some(headers),
        body: Some(resource.1),
        status: 200,
    })
}

#[register_handler]
fn handle_file(request: Request) -> Result<Response, HttpError> {

    let binding = ("".into(), "".into());

    let path = request.params.iter().find(|x| x.0 == "*").unwrap_or(&binding);

    let full_path = format!("{}/{}",&*static_path().lock().unwrap(),&path.1);

    let data = std::fs::read(&full_path).map_err( |e| HttpError::UnexpectedError(e.to_string() + &full_path ))?;

    let mut headers = request.headers.clone();

    if let Some(ctype) = Path::new(&full_path).extension().and_then(OsStr::to_str) {    
        match ctype {
            "html" => {
                headers.push((String::from("Content-Type"),String::from("text/html")));
            },
            "css" => {
                headers.push((String::from("Content-Type"),String::from("text/css")));
            },
            "js" => {
                headers.push((String::from("Content-Type"),String::from("application/javascript")));
            },                        
            _ => {}
        }    
        
    }

    Ok(Response {
        headers: Some(headers),
        body: Some(data),
        status: 200,
    })
}

#[register_handler]
fn handle_opds2(request: Request) -> Result<Response,HttpError> {

    let binding = ("".into(), "".into());
    let host = request.headers.iter().find(|x| x.0 == "host").unwrap_or(&binding);

    let storages = (&*keyvalue().lock().unwrap()).keys().map_err( |e| HttpError::UnexpectedError(e.to_string()))?;

    let policy = Policy::default();

    let m : HashMap<String,Vec<String>> =  
        storages.iter().flat_map ( |storage: &String| -> Vec<(String,String)> {
                if let Ok(storage_client) = storage::StorageClient::new(storage) {
                    if let Ok(list_of_pub_ids) = storage_client.list() {
                        list_of_pub_ids.iter().map(|value| (value.clone(),storage.clone())).collect()
                    }else {
                        Vec::new()
                    }
                }else {
                    Vec::new()
                }
                

        }).fold ( HashMap::new() , |mut acc, (key,value) |  { acc.entry(key).or_insert_with(Vec::new).push(value); acc });

    println!("Publications {:?}", m);

    let publications = m.iter().map( |(key,value)| {
            if let Ok(metadata_verifier) = metadataverifier::MetadataVerifierClient::new(&policy.metadata_verifier_service) {

                println!("Getting ... {}",key);
                if let Some(first_storage) = value.get(0){

                    if let Ok(metadata) = metadata_verifier.metadata_with(key.to_string(),
                                                                            policy.metadata_service.to_owned(),
                                                                            first_storage.to_string()) {
                        
                        let links = format!(",\"links\":[
                            {{
                            \"type\":\"application/webpub+json\",
                            \"rel\":\"http://opds-spec.org/acquisition\",
                            \"href\":\"https://{}/pub/{}/manifest.json\"
                            }}
                        ]",&host.1,key);

                        format!("{{ \"metadata\": {} {} }}",metadata,links)

                    }else {
                        String::from("")
                    }
                }else {
                    String::from("")
                }
            }else {
                String::from("")
            }
    }).collect::<Vec<String>>().join(",");
    
    let message = format!("{{
        \"metadata\":{{
           \"title\":\"OPDS 2.0 Catalog\"
        }},
        \"links\":[
           {{
              \"type\":\"application/opds+json\",
              \"rel\":\"self\",
              \"href\":\"https://{}/opds2/publications.json\"
           }}
        ],
        \"publications\":[ {} ]}}",
        &host.1,&publications);

    let mut headers = request.headers.clone();

    headers.push((String::from("Content-Type"),String::from("application/opds+json")));
    //headers.push((String::from("Content-Type"),String::from("application/xml")));

    //let message = format!("{:?}",&m);

    Ok(Response {
        headers: Some(headers),
        body: Some(message.into()),
        status: 200,
    })
}