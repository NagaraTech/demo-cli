mod keystore;

use std::ops::Add;
use clap::{Parser, Subcommand};
use std::path::Path;
use std::path::PathBuf;
use ed25519_dalek::Keypair;
use rand_core::{RngCore, OsRng};
use crate::keystore::{get_file_path,get_keypairs_list, Keystore};


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
    Import {
        /// The path to the working directory containing a `dna.yaml` manifest.
        path: std::path::PathBuf,
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
    List,
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
                let ap = get_file_path(&name);
                save_keystore.save(&ap);
                println!("credentials_default_path = {:?}", ap);
            }
            Self::Import {
                path,
            } => {
                Keystore::load(&path);
            }
            Self::Unpack {
                path,
                output,
                raw,
                force,
            } => {
                println!("Unpacked to directory {:?}", path);
            }
            Self::List => {
                get_keypairs_list();
            }
        }
        Ok(())
    }
}
