use zip_structs::zip_central_directory::ZipCDEntry;
use zip_structs::zip_eocd::ZipEOCD;
use zip_structs::zip_local_file_header::ZipLocalFileHeader;
use zip_structs::zip_error::ZipReadError;

use std::format;
use std::vec::Vec;
use std::string::String;
use std::collections::BTreeMap;
use std::boxed::Box;

use core::str;

use core::include_bytes;

use std::io::Cursor;

use compression::prelude::*;

use anyhow::{anyhow,Result,Error};

use serde::{Serialize,Deserialize};
use serde::de::DeserializeOwned;

use strong_xml::XmlRead;

use crate::epub::ncx::*;
use crate::epub::opf::*;
use crate::epub::container::*;

#[derive(Debug)]
pub struct Epub<'a> {

    pub archive: &'a EPubArchive,

    pub opf: Opf<'a>,
    pub ncx: Ncx<'a>,
    pub container: Container<'a>,
    
    pub dir: String,
    pub ncx_path : String
    
}

#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq)]
pub struct EPubArchive {

    data: BTreeMap<Vec<u8>,Vec<u8>>,
    
}

impl EPubArchive {

    pub fn new(source: &[u8]) ->Result<Self> {

        let mut data = BTreeMap::new();

        let mut zip_file = Cursor::new(source);
        let eocd = ZipEOCD::from_reader(&mut zip_file).unwrap();
        let cd_list = ZipCDEntry::all_from_eocd(&mut zip_file, &eocd).unwrap();

        for cd in cd_list {

            match ZipLocalFileHeader::from_central_directory(&mut zip_file, &cd) {
                Ok(local_file_header) => {

                                            match local_file_header.compression_method {
                                                8 => {
                                                    let decompressed = local_file_header.compressed_data.as_ref()
                                                        .iter()
                                                        .cloned()
                                                        .decode(&mut Deflater::new())
                                                        .collect::<Result<Vec<_>, _>>();

                                                    match decompressed {
                                                            Ok(content) => data.insert ( local_file_header.file_name_raw, content ),                                                        
                                                            Err(e) => return Err(anyhow!(format!("Decompressed {:?}",e)))
                                                    };
                                                    
                                                }
                                                0 => {data.insert ( local_file_header.file_name_raw, local_file_header.compressed_data.into());}
                                                                                              
                                                _ => return Err(anyhow!(format!("Usupported compression method {}",local_file_header.compression_method)))

                                            }

                                        }
                Err(e) =>  return Err(anyhow!(format!("Local file header {:?}",e)))
            }
        }


        Ok (EPubArchive {
            data,
        })
    }

    pub fn epub (&self) -> Result<Epub> {

            let container = Container::from_str (EPubArchive::get_as_str_from_data(&self.data,"META-INF/container.xml")? ).map_err(|e| anyhow!(format!("Container {:?}",e)))?;

            let opf = Opf::from_str (EPubArchive::get_as_str_from_data(&self.data,&container.rootfiles.rootfile.full_path)?).map_err(|e| anyhow!(format!("OPF {:?}",e)))?;

            let dir = EPubArchive::get_dir(&container.rootfiles.rootfile.full_path);

            let mut ncx_path = String::from("");

            let mut ncx = Ncx::default();

            if opf.spine.toc.is_some() {
                for item in &opf.manifest.items {
                        if opf.spine.toc.as_ref().expect("Not in spine").contains(&*item.id) {
                            ncx_path = format!("{}/{}",&dir,&item.href);
                            ncx = Ncx::from_str(EPubArchive::get_as_str_from_data(&self.data,&ncx_path)?).map_err(|e| anyhow!(format!("OPF {:?}",e)))?;
                            break;
                        }
                }
            }

            Ok(Epub {
                container,
                opf,
                ncx,
                archive: self,
                dir,
                ncx_path
            })
    }

    pub fn get_dir (file_name:&str) -> String {
            match file_name.rsplit_once('/') {  
                Some((path,_name)) => String::from(path),
                None => String::from("")
            }
    }



    //Lazy decompression??
    pub fn get ( &self, name: &Vec<u8>) -> Option<&Vec<u8>> {
        self.data.get(name)
    }

    pub fn get_as_str ( &self, name: &Vec<u8>) -> Option<&str> {
        match self.data.get(name){
            Some(content) => {
                            match str::from_utf8(content){
                                Ok(data) => Some(data),
                                Err(_) => None
                            }
            }
            None => None
        }
    }

    fn get_as_str_from_data<'d> ( epub: &'d BTreeMap<Vec<u8>,Vec<u8>>, name: &str) -> Result<&'d str> {

        match epub.get(&name.as_bytes().to_vec()){
            Some(content) => str::from_utf8(content).map_err(|e| anyhow!(format!("Can't get utf8 {:?}",e))),
            None => Err(anyhow!(format!("Can't get {:?}",name)))
        }
    }    

    pub fn get_container_file(&self) -> Option<&Vec<u8>> {
        let s = Box::new(b"META-INF/container.xml");

        self.get(&s.to_vec())
    }

    pub fn get_data(&self) -> BTreeMap<Vec<u8>,Vec<u8>> {
        use std::borrow::ToOwned;

        self.data.to_owned()
    }

}