use super::*;

pub fn process(format: &Format, output_path : String, output_files : &mut Files, file_path : String, item : &mut Vec<u8>) -> LLResult<()> {

    generic_processor(format, output_path, output_files, file_path, item)

}
