// use log::{info, error};
use sqlx::{Pool, Postgres};
use crate::AppError;


pub async fn create_src_tables(_pool : &Pool<Postgres>) -> Result<(), AppError>
{
   
    Ok(())

}

pub async fn process_data(_data_version: &String, _pool : &Pool<Postgres>) -> Result<(), AppError>
{

    // Import the data from ror schema to src schema.

    // if data version = "" obtain it from the ror tables


    // Update lang codes from scripts where possible, record lang code source type

    //src_script_coder::update_lang_code_source("ror", pool).await?;
    //src_script_coder::add_langs_for_nonlatin_codes(pool).await?;
    //src_script_coder::update_lang_code_source("script_auto", pool).await?;

    Ok(())
}

