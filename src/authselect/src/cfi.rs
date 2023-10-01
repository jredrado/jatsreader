use std::string::String;
use std::vec::Vec;

pub struct CFIComponent {

        pub node_index: usize,
        pub qualifier : Option<String>,
        pub character_offset : Option<u32>,
        pub temporal_offset: Option<f32>,
        pub text_qualifier: Option<String>

}

pub type CFIComponentList = Vec<CFIComponent>;

    /*
        Point           spatialOffset;      ///< The value of any spatial offset.
        SideBias        sideBias;
    */