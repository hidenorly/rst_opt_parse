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

use std::env;
use rst_opt_parse::IOptParse;
use rst_opt_parse::OptParse;
use rst_opt_parse::OptParseItem;

fn main() {
    let mut options = Vec::new();
    options.push( OptParseItem::new( "-s", "--samplingRate", true, "48000", "Set Sampling Rate") );
    options.push( OptParseItem::new( "-e", "--encoding", true, "PCM16", "Set Encoding PCM8, PCM16, PCM24, PCM32, PCMFLOAT") );
    options.push( OptParseItem::new( "-c", "--channel", true, "2", "Set channel 2, 2.1, 4, 4.1, 5, 5.1, 5.1.2, 7.1") );

    let argv: Vec<String> = env::args().collect();

    let mut opt_parse = OptParse::new( argv, options, "rst_opt_parse_test  e.g.input1.pcm input2.pcm -s 44100" );
    opt_parse.parse_options_with_required_args( true, 1, -1 );
    /* if --help is specified, following lines are not executed since true is specified */

    println!( "encoding:{}, samplingRate:{}, channel:{}", opt_parse.get_value("-e"), opt_parse.get_value("-s"), opt_parse.get_value("-c") );

    for i in 0..opt_parse.get_args_count(){
        println!("{}:{}", i, opt_parse.get_args(i) );
    }
}