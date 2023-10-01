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

use manifestverifier::ManifestVerifierClient;
use resourceverifier::ResourceVerifierClient;
use resolver::{ResolverClient,StreamerInfo};

use std::borrow::BorrowMut;
use std::sync::{Mutex, Once};

use std::path::Path;
use std::ffi::OsStr;

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


const document_url : &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Lens Viewer for Books and Journal articles</title>

    <link rel="stylesheet" type="text/css" media="all"
          href="https://maxcdn.bootstrapcdn.com/font-awesome/4.2.0/css/font-awesome.min.css"/>

    <link rel='stylesheet' type='text/css' href="/static/reader/lib/fonts.css"/>


    <!-- A combined lens.css will be generated in the bundling process -->
    <!-- While in development, separate links for each CSS file are added, so we don't need a source map -->
    <link rel="stylesheet" type="text/css" href="/static/reader/lens.css"/>

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

    <script src='/static/reader/lens.js'></script>
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
        .get("/raw/:id/*","handle_raw_resource")?        
        .get("/opds2/publications.json","handle_opds2")?
        .get("/urs/:locator","handle_urs")?
        .get("/static/*","handle_file")?;

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
fn handle_urs(request: Request) -> Result<Response, HttpError> {

    let locator = request.params.into_iter().find(|x| x.0 == "locator").unwrap_or(("".into(), "".into()));

    //let client = ResolverClient::new("resolver_1").map_err( |e| HttpError::UnexpectedError(e.to_string()))?;

    //let streamers = client.get_streamers(id.1.to_owned()).map_err( |e| HttpError::UnexpectedError(e.to_string()))?;

    //let manifests : Vec<StreamerInfo> = streamers.iter().map(|s| StreamerInfo{id:s.id.clone(),endpoint:format!("{}/pub/{}/manifest.json",&s.endpoint,&id.1)}).collect();

    Ok(Response {
        headers: Some(request.headers),
        body: Some(locator.1.as_bytes().to_vec()),
        status: 200,
    })
}


#[register_handler]
fn handle_resource_or_manifest(request: Request) -> Result<Response, HttpError> {

    let binding = ("".into(), "".into());

    let id = request.params.iter().find(|x| x.0 == "id").unwrap_or(&binding);
    let path = request.params.iter().find(|x| x.0 == "*").unwrap_or(&binding);

    if path.1 == "manifest.json" {
        handle_manifest(request)
    }else {
        handle_resource(request)
    }
}


fn handle_manifest(request: handle_resource_or_manifest_mod::Request) -> Result<handle_resource_or_manifest_mod::Response, handle_resource_or_manifest_mod::HttpError> {
    let id = request.params.into_iter().find(|x| x.0 == "id").unwrap_or(("".into(), "".into()));

    let mclient = ManifestVerifierClient::new("manifestverifier_1").map_err( |e| handle_resource_or_manifest_mod::HttpError::UnexpectedError(e.to_string()))?;
    let manifest = mclient.manifest(id.1).map_err( |e| handle_resource_or_manifest_mod::HttpError::UnexpectedError(e.to_string()))?;

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

    let rclient = ResourceVerifierClient::new("resourceverifier_1").map_err( |e| handle_resource_or_manifest_mod::HttpError::UnexpectedError(e.to_string()))?;
    let resource = rclient.resource(id.1.to_owned(),path.1.to_owned().into_bytes()).map_err( |e| handle_resource_or_manifest_mod::HttpError::UnexpectedError(e.to_string()))?;

    let mut headers = request.headers.clone();

    if let Some(ctype) = resource.0 {        

        if ctype == "text/xml" {

            headers.push((String::from("Content-Type"),String::from("text/html")));

            let unencode_url = format!("https://{}/raw/{}/{}",&host.1,&id.1,&path.1);
            let new_url = urlencoding::encode(&unencode_url);

            let body = document_url.replace("$DOCUMENT_URL$",&unencode_url);

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

    let rclient = ResourceVerifierClient::new("resourceverifier_1").map_err( |e| HttpError::UnexpectedError(e.to_string()))?;
    let resource = rclient.resource(id.1.to_owned(),path.1.to_owned().into_bytes()).map_err( |e| HttpError::UnexpectedError(e.to_string()))?;

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
    let mut headers = request.headers.clone();

    headers.push((String::from("Content-Type"),String::from("application/opds+json")));
    //headers.push((String::from("Content-Type"),String::from("application/xml")));

    Ok(Response {
        headers: Some(headers),
        body: Some(p.as_bytes().to_vec()),
        status: 200,
    })
}