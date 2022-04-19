/*
  Copyright (C) 2022 hidenorly

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use std::collections::HashMap;
use std::cmp;

#[derive(Clone)]
pub struct OptParseItem
{
    option : String,        // e.g. "-h"
    full_option : String,   // e.g. "--help"
    arg_required : bool,    // true: the value required / false: the value not required
    value : String,
    description : String,
}

impl OptParseItem
{
    pub fn new(
        option : &str,        // e.g. "-h"
        full_option : &str,   // e.g. "--help"
        arg_required : bool,    // true: the value required / false: the value not required
        value : &str,         // default value
        description : &str    // this is displayed in the help
    ) -> Self
    {
        Self {
            option : option.to_string(),
            full_option : full_option.to_string(),
            arg_required : arg_required,
            value : value.to_string(),
            description : description.to_string()
        }
    }
}


pub trait IOptParse
{
    fn new( args : Vec<String>, options : Vec<OptParseItem>, description : &str ) -> Self;
    fn parse_options( &mut self, is_finish_if_help : bool );
    fn parse_option( &mut self, option : &OptParseItem );
    fn print_help( &self );
    fn get_value( &self, option : &str ) -> String;
    fn get_args_count( &self ) -> usize;
    fn get_args(&self, index : usize ) -> String;
}

pub struct OptParse
{
    args : Vec<String>,
    options : Vec<OptParseItem>,
    values : HashMap<String, String>,
    arg_values : Vec<String>,
    description : String,
}

impl IOptParse for OptParse
{
    fn new( args : Vec<String>, options : Vec<OptParseItem>, description : &str ) -> Self {
        Self {
            args : args,
            options : options,
            values : HashMap::new(),
            arg_values : Vec::new(),
            description : description.to_string(),
        }
    }

    fn parse_options( &mut self, is_finish_if_help : bool ){
        let  _options = &self.options.clone();
        for option in _options {
            self.parse_option( &option );
        }

        let argc = &self.args.len();

        // -h or --help and call print_help()
        for i in 0..*argc {
            let arg = &self.args[i];
            if arg.eq( "-h" ) || arg.starts_with( "--help" ){
                self.print_help();
                if is_finish_if_help {
                    std::process::exit(0);
                }
            }
        }

        // parse args
        let mut i : usize = 0;
        while i < *argc {
            let arg = &self.args[i];
            if arg.starts_with( "-" ){
                for option in _options {
                    if arg.eq( &option.option ) && option.arg_required {
                        i = i + 1;
                    }
                }
            } else {
                self.arg_values.push( arg.to_string() );
            }
            i = i + 1;
        }
    }

    fn parse_option( &mut self, option : &OptParseItem ){
        let argc = &self.args.len();
        let mut value : String = option.value.clone();
        let mut found_set_true = false;
        for i in 0..*argc {
            let arg = &self.args[i];
            if option.option.eq( arg ) {
                // -s case
                if option.arg_required {
                    if (i+1) < *argc  {
                        if !self.args[ i+1 ].starts_with("-") {
                            value = self.args[ i+1 ].clone();
                        } else {
                            // TODO: this is arg required case but i+1 is not the value for the option
                        }
                    } else {
                        // TODO: this is arg required case but i+1 isn't present
                    }
                } else {
                    found_set_true = true;
                }
            } if arg.starts_with( &option.full_option ) {
                // --something case
                if option.arg_required {
                    let pos = arg.find("=");
                    match pos {
                        Some(the_pos) => {
                            value = arg[the_pos+1..].to_string();
                        },
                        None => {}
                    }
                } else {
                    found_set_true = true;
                }
            }
        }
        if found_set_true {
            value = "true".to_string();
        }
        let _ = &self.values.insert( option.option.clone(), value );
    }

    fn print_help(&self){
        let options_len = &self.options.len();
        let mut max_short_option_len : usize = 0;
        let mut max_full_option_len : usize = 0;
        for i in 0..*options_len {
            max_short_option_len = cmp::max( max_short_option_len, self.options[i].option.len() );
            max_full_option_len  = cmp::max( max_full_option_len,  self.options[i].full_option.len() );
        }
        if !&self.description.is_empty() {
            println!( "{}", &self.description );
        }
        for i in 0..*options_len {
            println!( " {:short_len$}\t {:full_len$}\t : {}", &self.options[i].option, &self.options[i].full_option, &self.options[i].description, short_len = max_short_option_len, full_len = max_full_option_len );
        }
    }

    fn get_value( &self, option : &str ) -> String {
        match self.values.get( option ){
            Some( v ) => v.to_string(),
            None => String::from("")
        }
    }

    fn get_args_count(&self) -> usize {
        self.arg_values.len()
    }

    fn get_args(&self, index : usize ) -> String {
        let mut result = String::from("");
        if index < self.get_args_count() {
            result = self.arg_values[ index ].to_string();
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opt_parse() {
        let mut options = Vec::new();
        options.push( OptParseItem::new( "-r", "--samplingRate", true, "48000", "Set Sampling Rate") );
        options.push( OptParseItem::new( "-e", "--encoding", true, "PCM16", "Set Encoding PCM8, PCM16, PCM24, PCM32, PCMFLOAT") );
        options.push( OptParseItem::new( "-c", "--channel", true, "2", "Set channel 2, 2.1, 4, 4.1, 5, 5.1, 5.1.2, 7.1") );

        let mut argv : Vec<String> = Vec::new();
        argv.push( "-r".to_string() );
        argv.push( "44100".to_string());
        argv.push( "--encoding=PCM32".to_string() );

        let mut opt_parse = OptParse::new( argv, options, "rst_opt_parse_test" );
        opt_parse.parse_options( false );

        assert_eq!( opt_parse.get_value("-r"), "44100" );
        assert_eq!( opt_parse.get_value("-e"), "PCM32" );
        assert_eq!( opt_parse.get_value("-c"), "2" );
    }

    #[test]
    fn test_opt_parse_help() {
        let mut options = Vec::new();
        options.push( OptParseItem::new( "-r", "--samplingRate", true, "48000", "Set Sampling Rate") );
        options.push( OptParseItem::new( "-e", "--encoding", true, "PCM16", "Set Encoding PCM8, PCM16, PCM24, PCM32, PCMFLOAT") );
        options.push( OptParseItem::new( "-c", "--channel", true, "2", "Set channel 2, 2.1, 4, 4.1, 5, 5.1, 5.1.2, 7.1") );

        let mut argv : Vec<String> = Vec::new();
        argv.push( "-h".to_string() );

        let mut opt_parse = OptParse::new( argv, options, "rst_opt_parse_test" );
        opt_parse.parse_options( false );
    }

    #[test]
    fn test_opt_parse_exception_no_opt_parse_item() {
        let options = Vec::new();

        let mut argv : Vec<String> = Vec::new();
        argv.push( "-r".to_string() );
        argv.push( "44100".to_string());
        argv.push( "--encoding=PCM32".to_string() );

        let mut opt_parse = OptParse::new( argv, options, "rst_opt_parse_test" );
        opt_parse.parse_options( false );

        assert_eq!( opt_parse.get_value("-r"), "" );
        assert_eq!( opt_parse.get_value("-e"), "" );
        assert_eq!( opt_parse.get_value("-c"), "" );
    }

    #[test]
    fn test_opt_parse_no_arg() {
        let mut options = Vec::new();
        options.push( OptParseItem::new( "-v", "--verbose", false, "false", "Enable verbose mode") );
        options.push( OptParseItem::new( "-q", "--quiet", false, "false", "Enable quiet mode") );

        let mut argv : Vec<String> = Vec::new();
        argv.push( "-v".to_string() );
        argv.push( "-s".to_string() );

        let mut opt_parse = OptParse::new( argv, options, "rst_opt_parse_test" );
        opt_parse.parse_options( false );

        assert_eq!( opt_parse.get_value("-v"), "true" );
        assert_eq!( opt_parse.get_value("-q"), "false" );
        assert_eq!( opt_parse.get_value("-s"), "" );
    }

    #[test]
    fn test_opt_parse_arg_required_but_nothing() {
        let mut options = Vec::new();
        options.push( OptParseItem::new( "-v", "--verbose", false, "false", "Enable verbose mode") );
        options.push( OptParseItem::new( "-s", "--samplingRate", true, "48000", "Set sampling rate e.g. 44100") );

        let mut argv : Vec<String> = Vec::new();
        argv.push( "-s".to_string() );
        argv.push( "-v".to_string() ); // Expects the -s's value here but not specified

        let mut opt_parse = OptParse::new( argv, options, "rst_opt_parse_test" );
        opt_parse.parse_options( false );

        assert_eq!( opt_parse.get_value("-v"), "true" );
        assert_eq!( opt_parse.get_value("-s"), "48000" );
        assert_eq!( opt_parse.get_value("-x"), "" );
    }

    #[test]
    fn test_opt_parse_args() {
        let mut options = Vec::new();
        options.push( OptParseItem::new( "-v", "--verbose", false, "false", "Enable verbose mode") );
        options.push( OptParseItem::new( "-s", "--samplingRate", true, "48000", "Set sampling rate e.g. 44100") );

        let mut argv : Vec<String> = Vec::new();
        argv.push( "input.csv".to_string() );
        argv.push( "-v".to_string() );
        argv.push( "output1.csv".to_string() );
        argv.push( "--samplingRate=44100".to_string() );
        argv.push( "output2.csv".to_string() );

        let mut opt_parse = OptParse::new( argv, options, "rst_opt_parse_test" );
        opt_parse.parse_options( false );

        assert_eq!( opt_parse.get_value("-v"), "true" );
        assert_eq!( opt_parse.get_value("-s"), "44100" );
        assert_eq!( opt_parse.get_value("-x"), "" );

        assert_eq!( opt_parse.get_args_count(), 3 );
        assert_eq!( opt_parse.get_args(0), "input.csv" );
        assert_eq!( opt_parse.get_args(0), "input.csv" );
        assert_eq!( opt_parse.get_args(1), "output1.csv" );
        assert_eq!( opt_parse.get_args(2), "output2.csv" );
        assert_eq!( opt_parse.get_args(3), "" );
    }
}
