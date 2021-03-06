use crate::config::Config;
use crate::util::init_db;
use clap::ArgMatches;
use log::{info, log};
use sync::{create_sync_blocks_writer, Error};

pub fn import(cfg: Config, matches: &ArgMatches) -> Result<(), String> {
    r#try!(init_db(&cfg));

    let blk_path = matches
        .value_of("PATH")
        .expect("PATH is required in cli.yml; qed");

    let mut writer = create_sync_blocks_writer(cfg.db, cfg.consensus, cfg.verification_params);

    let blk_dir =
        r#try!(::import::open_blk_dir(blk_path)
            .map_err(|_| "Import directory does not exist".to_owned()));
    let mut counter = 0;
    for blk in blk_dir {
        // TODO: verify magic!
        let blk = r#try!(blk.map_err(|_| "Cannot read block".to_owned()));
        match writer.append_block(blk.block) {
            Ok(_) => {
                counter += 1;
                if counter % 1000 == 0 {
                    info!(target: "sync", "Imported {} blocks", counter);
                }
            }
            Err(Error::TooManyOrphanBlocks) => {
                return Err("Too many orphan (unordered) blocks".into());
            }
            Err(_) => return Err("Cannot append block".into()),
        }
    }

    info!("Finished import of {} blocks", counter);

    Ok(())
}
