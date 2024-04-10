mod keystore;

use std::ops::Add;
use clap::{Parser, Subcommand};
use std::path::Path;
use std::path::PathBuf;
use ed25519_dalek::Keypair;
use rand_core::{RngCore, OsRng};
use crate::keystore::{get_file_path, Keystore};


#[derive(Debug, Parser)]
#[command(version, about)]
pub struct KeystoreBundle {
    #[command(subcommand)]
    pub subcommand: KeystoreBundleSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum KeystoreBundleSubcommand {
    /// Create a new, empty Holochain DNA bundle working directory and create a new
    /// sample `dna.yaml` manifest inside.
    New {
        name: String,
        /// The path to create the working directory.
        path: Option<PathBuf>,
    },

    /// Pack into the `[name].dna` bundle according to the `dna.yaml` manifest,
    /// found inside the working directory. The `[name]` is taken from the `name`
    /// property of the manifest file.
    ///
    /// e.g.:
    ///
    /// $ hc dna pack ./some/directory/foo
    ///
    /// creates a file `./some/directory/foo/[name].dna`, based on
    /// `./some/directory/foo/dna.yaml`.
    Pack {
        /// The path to the working directory containing a `dna.yaml` manifest.
        path: std::path::PathBuf,

        /// Specify the output path for the packed bundle file.
        ///
        /// If not specified, the `[name].dna` bundle will be placed inside the
        /// provided working directory.
        #[arg(short = 'o', long)]
        output: Option<PathBuf>,

        /// Output shared object "dylib" files
        /// that can be used to run this happ on iOS
        #[arg(long)]
        dylib_ios: bool,
    },

    /// Unpack parts of the `.dna` bundle file into a specific directory.
    ///
    /// e.g.:
    ///
    /// $ hc dna unpack ./some/dir/my-dna.dna
    ///
    /// creates a new directory `./some/dir/my-dna`, containining a new `dna.yaml`
    /// manifest.
    // #[arg(short = 'u', long)]
    Unpack {
        /// The path to the bundle to unpack.
        path: std::path::PathBuf,

        /// Specify the directory for the unpacked content.
        ///
        /// If not specified, the directory will be placed alongside the
        /// bundle file, with the same name as the bundle file name.
        #[arg(short = 'o', long)]
        output: Option<PathBuf>,

        /// Don't attempt to parse the manifest. Useful if you have a manifest
        /// of an outdated format. This command will allow you to unpack the
        /// manifest so that it may be modified and repacked into a valid bundle.
        #[arg(short = 'r', long)]
        raw: bool,

        /// Overwrite an existing directory, if one exists.
        #[arg(short = 'f', long)]
        force: bool,
    },

    /// Print the schema for a DNA manifest
    Schema,
}


impl KeystoreBundle {
    /// Run this subcommand, passing off all the work to the sub-sub-command enum
    pub async fn run(self) -> anyhow::Result<()> {
        self.subcommand.run().await
    }
}

impl KeystoreBundleSubcommand {
    /// Run this command
    pub async fn run(self) -> anyhow::Result<()> {
        match self {
            Self::New { name, path } => {
                println!("init");
                let mut csprng = OsRng {};
                let keypair = Keypair::generate(&mut csprng).to_bytes();
                let save_keystore = Keystore {
                    name: name.clone(),
                    keypair,
                };
                let keypair_json_file = name.add(".json");
                let ap = get_file_path(keypair_json_file);
                save_keystore.save(&ap);
                println!("credentials_default_path = {:?}", ap);
            }
            Self::Pack {
                path,
                output,
                dylib_ios,
            } => {
                println!("Wrote bundle {:?}", path);
            }
            Self::Unpack {
                path,
                output,
                raw,
                force,
            } => {
                println!("Unpacked to directory {:?}", path);
            }
            Self::Schema => {
                println!("Schema");
            }
        }
        Ok(())
    }
}
